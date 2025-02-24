use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use rand::prelude::*;
use std::os::windows::process::CommandExt;
use std::process::{Child, Command};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::tokiort::TokioIo;
use axum::{
    body::Body,
    extract::OriginalUri,
    http::{Request, Response, Uri},
    routing::any,
};
use http_body_util::BodyExt;
use hyper::client::conn::http1;

fn random_string() -> String {
    let mut rng = rand::rng();
    let mut arr = [0u8; 10];
    rng.fill(&mut arr);
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
}

async fn start_django_server(State(state): State<Arc<RouterState>>) -> impl IntoResponse {
    let process_handle = Arc::clone(&state.django_process_handle); // クローンして取り出す
    let mut handle = process_handle.lock().await;
    let router_root_path = state.router_root_path.clone();

    let rand_string = random_string();
    let admin_root = format!("admin_proxy/{}/", rand_string);

    if handle.is_some() {
        println!("Django server is already running!");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html("Django server is already running!".to_string()),
        );
    }

    // コマンドを設定し、プロセスを起動
    let mut command = Command::new("python");
    let admin_origin = "localhost:8001";

    command.args([
        "../table_definition/manage.py", // 適切なmanage.pyへのパス
        "run_with_custom_admin",
        "--admin-root",
        &admin_root,
        admin_origin,
    ]);

    // Windows でプロセスグループを作成（UNIX系でも有効）
    #[cfg(windows)]
    command.creation_flags(winapi::um::winbase::CREATE_NEW_PROCESS_GROUP);
    #[cfg(unix)]
    command.pre_exec(|| {
        // プロセスグループIDを自分自身に設定
        let _ = nix::unistd::setsid();
        Ok(())
    });

    match command.spawn() {
        Ok(child) => {
            println!("Django server started successfully with PID {}", child.id());
            *handle = Some(child);
            (
                StatusCode::OK,
                Html(format!(
                    r#"<!doctype html><html lang="ja"><body>Django server started successfully.
            <br /><a id="entrance" href="/{admin_root}" target="_blank">Admin page</a>
            <br /><a href="{router_root_path}_admin_stop">Stop Admin page</a>
            <style>#entrance {{ color: grey; }} #entrance.on {{ color: green; }}</style>
            <script>
                const health_check_endpoint = "/admin_proxy/{rand_string}/health-check";
                setInterval(() => {{
                    fetch(health_check_endpoint).then((res) => {{
                        const alive = res.statusText == "OK";
                        if (alive) {{
                            document.getElementById("entrance").classList.add("on");
                        }} else {{
                            document.getElementById("entrance").classList.remove("on");
                        }}
                    }});
                }}, 1000);
            </script>
            </body></html>
            "#
                )),
            )
        }
        Err(error) => {
            println!("Failed to start Django server: {}", error);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html("Failed to start Django server".to_string()),
            )
        }
    }
}

async fn stop_django_server(State(state): State<Arc<RouterState>>) -> impl IntoResponse {
    let process_handle = Arc::clone(&state.django_process_handle); // クローンして取り出す
    let mut handle = process_handle.lock().await;
    let router_root_path = state.router_root_path.clone();

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
            Html("Error stopping Django server".to_string())
        } else {
            println!("Django server stopped successfully.");
            Html(format!(
                "Django server stopped successfully.\
            <br />redirect to home page in 2 seconds.\
            <script>setTimeout(() => (location.assign('{}')), 2000)</script>",
                router_root_path
            ))
        }
    } else {
        println!("No server is running.");
        Html(format!(
            "No server is running.\
            <br />redirect to home page in 2 seconds.\
            <script>setTimeout(() => (location.assign('{}')), 2000)</script>",
            router_root_path
        ))
    }
}

async fn get_index(State(state): State<Arc<RouterState>>) -> impl IntoResponse {
    let router_root_path = state.router_root_path.clone();
    (
        StatusCode::OK,
        Html(
            format!("<!doctype html><html><title>admin site portal</title><body><a href=\"/\">HOME</a><br /><a href=\"{router_root_path}_admin_start\">Start Django Server</a>\
        <br />\
        <a href=\"{router_root_path}_admin_stop\">Stop Django Server</a>\
    </body></html>")
        ),
    )
        .into_response()
}

#[derive(Clone)]
struct RouterState {
    router_root_path: String,
    django_process_handle: Arc<Mutex<Option<Child>>>, // プロセス管理用のデモ的な型
}

pub fn create_admin_portal_router(router_root_path: &str) -> (Router, Router, Router) {
    let state = Arc::new(RouterState {
        router_root_path: router_root_path.to_string(),
        django_process_handle: Arc::new(Mutex::new(None)),
    });

    let operation_router = Router::new()
        .route("/", get(get_index))
        .route("/_admin_start", get(start_django_server))
        .route("/_admin_stop", get(stop_django_server))
        .with_state(state);

    let proxy_router = Router::new()
        .route("/", any(proxy_handler))
        .route("/{*path}", any(proxy_handler));

    let admin_static_router = Router::new()
        .route("/", any(proxy_handler))
        .route("/{*path}", any(proxy_handler));

    (operation_router, proxy_router, admin_static_router)
}

const PROXY_HOST: &str = "http://127.0.0.1:8001";
const PROXY_HOST_PORT: &str = "127.0.0.1:8001";

async fn proxy_handler(OriginalUri(uri): OriginalUri, mut req: Request<Body>) -> impl IntoResponse {
    let target_uri = format!("{}{}", PROXY_HOST, uri);

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

    println!("[proxy to] {:?}", url);

    match tokio::net::TcpStream::connect(PROXY_HOST_PORT).await {
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
                .header("Host", PROXY_HOST_PORT);

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
