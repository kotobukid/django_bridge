use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use std::os::unix::prelude::CommandExt;
use std::process::{Child, Command};
use std::sync::{Arc, OnceLock};
use tokio::sync::Mutex;

use crate::state::AppState;
use crate::tokiort::TokioIo;
use axum::{
    body::Body,
    extract::OriginalUri,
    http::{Request, Response, Uri},
    routing::any,
};
use http_body_util::BodyExt;
use hyper::client::conn::http1;
use serde::Serialize;

fn random_string() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut arr = [0u8; 10];
    rng.fill(&mut arr);
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
}

#[derive(Serialize)]
struct DjangoStartResult {
    success: bool,
    entry: Option<String>,
}

#[derive(Serialize)]
struct DjangoStatusResult {
    success: bool,
    is_running: bool,
    admin_root: Option<String>,
    error: Option<String>,
}

fn find_working_python() -> Option<String> {
    use std::env;

    let candidates = vec![
        // 1. 環境変数で指定されたPython
        env::var("DJANGO_BRIDGE_PYTHON").ok(),
        // 2. 現在のPATHに従ってpython実行可能ファイルを探す
        find_python_in_path(),
        // 3. 一般的なvenv位置を探す
        find_python_in_venvs(),
        // 4. システムのpython3/python
        Some("python3".to_string()),
        Some("python".to_string()),
    ];

    for candidate in candidates.into_iter().flatten() {
        if test_python_with_django(&candidate) {
            println!("Found working Python: {}", candidate);
            return Some(candidate);
        }
    }

    None
}

fn find_python_in_path() -> Option<String> {
    // which pythonやwhere pythonの結果を使用
    if let Ok(output) = Command::new("which").arg("python").output() {
        if output.status.success() {
            return Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
        }
    }

    if let Ok(output) = Command::new("which").arg("python3").output() {
        if output.status.success() {
            return Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
        }
    }

    None
}

fn find_python_in_venvs() -> Option<String> {
    use std::path::Path;

    // 1. poetry環境の検出
    if let Some(poetry_python) = detect_poetry_python() {
        return Some(poetry_python);
    }

    // 2. pipenv環境の検出
    if let Some(pipenv_python) = detect_pipenv_python() {
        return Some(pipenv_python);
    }

    // 3. 一般的なvenv位置をチェック
    let venv_candidates = vec![
        "../table_definition/.venv/bin/python",
        "../table_definition/venv/bin/python",
        "../.venv/bin/python",
        "./venv/bin/python",
        "../venv/bin/python",
    ];

    for candidate in venv_candidates {
        if Path::new(candidate).exists() {
            return Some(candidate.to_string());
        }
    }

    None
}

fn detect_poetry_python() -> Option<String> {
    use std::path::Path;

    // pyproject.tomlが存在するかチェック
    if Path::new("../table_definition/pyproject.toml").exists() {
        if let Ok(output) = Command::new("poetry")
            .args(["env", "info", "--path"])
            .current_dir("../table_definition")
            .output()
        {
            if output.status.success() {
                let venv_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let python_path = format!("{}/bin/python", venv_path);
                if Path::new(&python_path).exists() {
                    return Some(python_path);
                }
            }
        }
    }
    None
}

fn detect_pipenv_python() -> Option<String> {
    use std::path::Path;

    // Pipfileが存在するかチェック
    if Path::new("../table_definition/Pipfile").exists() {
        if let Ok(output) = Command::new("pipenv")
            .args(["--py"])
            .current_dir("../table_definition")
            .output()
        {
            if output.status.success() {
                let python_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if Path::new(&python_path).exists() {
                    return Some(python_path);
                }
            }
        }
    }
    None
}

fn test_python_with_django(python_path: &str) -> bool {
    // 実際にDjangoを使えるかテスト
    let test_cmd = Command::new(python_path)
        .args(["-c", "import django; print('Django available')"])
        .output();

    match test_cmd {
        Ok(output) => {
            if output.status.success() {
                // さらに、manage.pyが動作するかテスト
                let manage_test = Command::new(python_path)
                    .args(["../table_definition/manage.py", "--help"])
                    .output();

                if let Ok(manage_output) = manage_test {
                    return manage_output.status.success();
                }
            }
        }
        Err(_) => {}
    }

    false
}

// Python検出結果をキャッシュ
static PYTHON_PATH_CACHE: OnceLock<Option<String>> = OnceLock::new();

fn get_cached_python_path() -> Option<String> {
    PYTHON_PATH_CACHE
        .get_or_init(|| {
            println!("Detecting working Python installation...");
            find_working_python()
        })
        .clone()
}

fn create_python_command() -> Command {
    if let Some(python_path) = get_cached_python_path() {
        println!("Using verified Python: {}", python_path);
        Command::new(python_path)
    } else {
        println!("Warning: No working Python found, using fallback");
        Command::new("python3")
    }
}

fn should_use_gunicorn() -> bool {
    use std::env;

    // 環境変数でgunicorn使用を指定
    env::var("DJANGO_BRIDGE_USE_GUNICORN").is_ok()
}

fn create_gunicorn_command(
    admin_root: &str,
    django_admin_port: u16,
    axum_web_port: u16,
) -> Command {
    let mut cmd = Command::new("gunicorn");

    cmd.args([
        "--config",
        "../table_definition/gunicorn_config.py",
        "--bind",
        &format!("127.0.0.1:{}", django_admin_port),
        "--workers",
        "1",
        "--timeout",
        "30",
        "django_bridge_wsgi:application",
    ]);

    // 環境変数を設定
    cmd.env("DJANGO_BRIDGE_ADMIN_ROOT", admin_root);
    cmd.env("DJANGO_ADMIN_PORT", django_admin_port.to_string());
    cmd.env("AXUM_WEB_PORT", axum_web_port.to_string());
    cmd.current_dir("../table_definition");

    cmd
}

async fn start_django_server(State(router_state): State<Arc<RouterState>>) -> impl IntoResponse {
    let process_handle = Arc::clone(&router_state.django_process_handle); // クローンして取り出す
    let mut handle = process_handle.lock().await;

    let rand_string = random_string();
    let admin_root = format!("admin_proxy/{}/", rand_string);

    if handle.is_some() {
        println!("Django server is already running!");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(DjangoStartResult {
                success: true,
                entry: None,
            }),
        );
    }

    let admin_origin = format!("localhost:{}", &router_state.django_admin_port);
    let axum_web_port = router_state.axum_web_port;

    let mut command = if should_use_gunicorn() {
        println!("Using Gunicorn for Django server");
        create_gunicorn_command(&admin_root, router_state.django_admin_port, axum_web_port)
    } else {
        println!("Using Django development server");
        let mut cmd = create_python_command();
        cmd.args([
            "../table_definition/manage.py",
            "run_with_custom_admin",
            "--admin-root",
            &admin_root,
            admin_origin.as_str(),
            "--csrf-trust-port",
            axum_web_port.to_string().as_str(),
        ]);
        cmd
    };

    // Windows でプロセスグループを作成（UNIX系でも有効）
    #[cfg(windows)]
    command.creation_flags(winapi::um::winbase::CREATE_NEW_PROCESS_GROUP);
    #[cfg(unix)]
    unsafe {
        command.pre_exec(|| {
            // プロセスグループIDを自分自身に設定
            let _ = nix::unistd::setsid();
            Ok(())
        });
    }

    match command.spawn() {
        Ok(child) => {
            println!("Django server started successfully with PID {}", child.id());
            *handle = Some(child);
            (
                StatusCode::OK,
                Json(DjangoStartResult {
                    success: true,
                    entry: Some(admin_root.clone()),
                }),
            )
        }
        Err(error) => {
            println!("Failed to start Django server: {}", error);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(DjangoStartResult {
                    success: false,
                    entry: None,
                }),
            )
        }
    }
}

async fn stop_django_server(State(state): State<Arc<RouterState>>) -> impl IntoResponse {
    let process_handle = Arc::clone(&state.django_process_handle); // クローンして取り出す
    let mut handle = process_handle.lock().await;

    if let Some(mut child) = handle.take() {
        println!("Stopping Django server with PID {}", child.id());

        // WindowsではCtrl+Cに相当するCTRL_BREAKを送信
        #[cfg(windows)]
        {
            use winapi::um::wincon::GenerateConsoleCtrlEvent;
            unsafe {
                if GenerateConsoleCtrlEvent(winapi::um::wincon::CTRL_BREAK_EVENT, child.id()) == 0 {
                    println!("Failed to send CTRL_BREAK_EVENT to Django server");
                } else {
                    println!("CTRL_BREAK_EVENT sent to Django server");
                }
            }
        }

        // UNIXベース (Linux/Mac) の場合、プロセスグループ全体にSIGINT送信
        #[cfg(unix)]
        {
            use nix::sys::signal::{killpg, Signal};
            use nix::unistd::Pid;

            if let Err(err) = killpg(Pid::from_raw(child.id() as i32), Signal::SIGINT) {
                println!("Failed to send SIGINT to Django server: {}", err);
            } else {
                println!("SIGINT sent to Django server");
            }
        }

        // プロセスが自動停止せずにまだ動作している場合にkillを試みる
        if let Err(err) = child.kill() {
            println!("Error stopping Django server: {}", err);
            Json(DjangoStartResult {
                success: false,
                entry: None,
            })
        } else {
            println!("Django server stopped successfully.");

            Json(DjangoStartResult {
                success: true,
                entry: None,
            })
        }
    } else {
        println!("No server is running.");
        Json(DjangoStartResult {
            success: false,
            entry: None,
        })
    }
}

async fn get_django_status(State(state): State<Arc<RouterState>>) -> impl IntoResponse {
    let process_handle = Arc::clone(&state.django_process_handle);
    let handle = process_handle.lock().await;

    // プロセスが起動しているかチェック
    let is_running = if let Some(_child) = handle.as_ref() {
        // プロセスハンドルが存在すれば起動中とみなす
        // より正確なチェックが必要な場合は、ヘルスチェックエンドポイントを使用
        true
    } else {
        false
    };

    if !is_running {
        return Json(DjangoStatusResult {
            success: true,
            is_running: false,
            admin_root: None,
            error: None,
        });
    }

    // Djangoが起動している場合、管理画面ルートを取得
    drop(handle); // lockを早めに解放

    let mut command = create_python_command();
    command.args([
        "../table_definition/manage.py",
        "get_admin_root",
        "--format",
        "json",
    ]);

    match command.output() {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);

                // JSONをパース
                if let Ok(django_result) = serde_json::from_str::<serde_json::Value>(&stdout) {
                    let admin_root = django_result
                        .get("admin_root")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());

                    Json(DjangoStatusResult {
                        success: true,
                        is_running: true,
                        admin_root,
                        error: None,
                    })
                } else {
                    Json(DjangoStatusResult {
                        success: false,
                        is_running: true,
                        admin_root: None,
                        error: Some("Failed to parse Django command output".to_string()),
                    })
                }
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Json(DjangoStatusResult {
                    success: false,
                    is_running: true,
                    admin_root: None,
                    error: Some(format!("Django command failed: {}", stderr)),
                })
            }
        }
        Err(e) => Json(DjangoStatusResult {
            success: false,
            is_running: true,
            admin_root: None,
            error: Some(format!("Failed to execute Django command: {}", e)),
        }),
    }
}

#[derive(Clone)]
struct RouterState {
    django_process_handle: Arc<Mutex<Option<Child>>>, // プロセス管理用のデモ的な型
    django_admin_port: u16,
    axum_web_port: u16,
}

pub fn create_admin_portal_router(
    django_admin_port: u16,
    axum_web_port: u16,
) -> (Router<AppState>, Router<AppState>, Router<AppState>) {
    let state = Arc::new(RouterState {
        django_process_handle: Arc::new(Mutex::new(None)),
        django_admin_port,
        axum_web_port,
    });

    let operation_router = Router::new()
        .route("/api/start_admin.json", post(start_django_server))
        .route("/api/stop_admin.json", post(stop_django_server))
        .route("/api/status.json", get(get_django_status))
        .with_state(state);

    let proxy_router = Router::new()
        .route("/", any(proxy_handler))
        .route("/{*path}", any(proxy_handler));

    let admin_static_router = Router::new()
        .route("/", any(proxy_handler))
        .route("/{*path}", any(proxy_handler));

    (operation_router, proxy_router, admin_static_router)
}

async fn proxy_handler(
    state: State<AppState>,
    OriginalUri(uri): OriginalUri,
    mut req: Request<Body>,
) -> impl IntoResponse {
    let proxy_host_port = format!("127.0.0.1:{}", state.django_admin_port);
    let proxy_host = format!("http://{}", proxy_host_port);
    let target_uri = format!("{}{}", proxy_host, uri);

    let url = match Uri::try_from(target_uri) {
        Ok(uri) => uri,
        Err(err) => {
            eprintln!("Invalid URI: {}", err);
            return Response::builder()
                .status(500)
                .body(Body::from("Invalid URI"))
                .unwrap();
        }
    };

    *req.uri_mut() = uri.clone();

    println!("[proxy to2] {:?}", url);

    match tokio::net::TcpStream::connect(&proxy_host_port).await {
        Ok(stream) => {
            let io = TokioIo::new(stream); // `TokioIo` を使用

            let (mut sender, connection) = match http1::handshake(io).await {
                Ok(conn) => conn,
                Err(err) => {
                    eprintln!("Handshake failed: {:?}", err);
                    return Response::builder()
                        .status(500)
                        .body(Body::from("Internal Server Error"))
                        .unwrap();
                }
            };

            tokio::spawn(async move {
                if let Err(err) = connection.await {
                    eprintln!("Connection closed with error: {:?}", err);
                }
            });

            // プロキシ先リクエスト構築
            let mut proxied_req_builder = Request::builder()
                .uri(uri.path_and_query().unwrap().to_string()) // 修正済
                .method(req.method().clone())
                .header("Host", proxy_host_port);

            // 元のヘッダーをコピーする際に、Hostヘッダーは除外
            for (key, value) in req.headers() {
                if key.as_str().to_lowercase() != "host" {
                    // Hostヘッダーはスキップ
                    proxied_req_builder = proxied_req_builder.header(key, value);
                }
            }

            let proxied_req = proxied_req_builder.body(req.into_body()).unwrap();

            // プロキシ先へリクエスト送信
            let proxied_res = match sender.send_request(proxied_req).await {
                Ok(res) => res,
                Err(err) => {
                    eprintln!("Request failed: {:?}", err);
                    return Response::builder()
                        .status(502)
                        .body(Body::from("Bad Gateway"))
                        .unwrap();
                }
            };

            // レスポンス処理
            let mut response_builder = Response::builder().status(proxied_res.status());
            for (key, value) in proxied_res.headers() {
                response_builder = response_builder.header(key, value.clone());
            }

            let mut response_bytes = Vec::new();
            let mut proxied_body = proxied_res.into_body();

            while let Some(frame) = proxied_body.frame().await {
                match frame {
                    Ok(chunk) if chunk.is_data() => {
                        if let Some(bytes) = chunk.data_ref() {
                            response_bytes.extend_from_slice(bytes);
                        }
                    }
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("Error reading response body: {:?}", err);
                        break;
                    }
                };
            }

            response_builder.body(Body::from(response_bytes)).unwrap()
        }
        Err(err) => {
            eprintln!("Failed to connect to target: {:?}", err);
            Response::builder()
                .status(500)
                .body(Body::from("Internal Server Error"))
                .unwrap()
        }
    }
}
