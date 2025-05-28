use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use rand::prelude::*;
use std::os::unix::prelude::CommandExt;
use std::process::{Child, Command};
use std::sync::Arc;
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
    let mut rng = rand::rng();
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

    // コマンドを設定し、プロセスを起動
    let mut command = Command::new("python");
    let admin_origin = {
        let state = router_state;
        format!("localhost:{}", state.django_admin_port)
    };

    command.args([
        "../table_definition/manage.py", // 適切なmanage.pyへのパス
        "run_with_custom_admin",
        "--admin-root",
        &admin_root,
        admin_origin.as_str(),
    ]);

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

#[derive(Clone)]
struct RouterState {
    django_process_handle: Arc<Mutex<Option<Child>>>, // プロセス管理用のデモ的な型
    django_admin_port: u16,
}

pub fn create_admin_portal_router(
    django_admin_port: u16,
) -> (Router<AppState>, Router<AppState>, Router<AppState>) {
    let state = Arc::new(RouterState {
        django_process_handle: Arc::new(Mutex::new(None)),
        django_admin_port,
    });

    let operation_router = Router::new()
        .route("/api/start_admin.json", post(start_django_server))
        .route("/api/stop_admin.json", post(stop_django_server))
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
