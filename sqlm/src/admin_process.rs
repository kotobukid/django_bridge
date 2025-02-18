use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use std::os::windows::process::CommandExt;
use std::process::{Child, Command};
use std::sync::Arc;
use tokio::sync::Mutex;
// use rand::{distributions::Alphanumeric, Rng};
use rand::prelude::*;

fn random_string() -> String {
    let mut rng = rand::rng();
    let mut arr2 = [0u8; 10];
    rng.fill(&mut arr2);
    arr2.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
}

async fn start_django_server(State(state): State<Arc<RouterState>>) -> impl IntoResponse {
    let process_handle = Arc::clone(&state.django_process_handle); // クローンして取り出す
    let mut handle = process_handle.lock().await;
    let router_root_path = state.router_root_path.clone();

    let admin_root = format!("custom_{}/admin/", random_string());

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
        &admin_origin,
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
                    "Django server started successfully.\
            <br /><a href=\"http://{}/{}\" target=\"_blank\">Admin page</a>\
            <br /><a href=\"{}admin_stop\">Stop Admin page</a>\
            ",
                    admin_origin, admin_root, router_root_path
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
            format!("<!doctype html><html><title>admin site portal</title><body><a href=\"/\">HOME</a><br /><a href=\"{router_root_path}admin_start\">Start Django Server</a>\
        <br />\
        <a href=\"{router_root_path}admin_stop\">Stop Django Server</a>\
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

pub fn create_admin_portal_router(router_root_path: &str) -> Router {
    let state = Arc::new(RouterState {
        router_root_path: router_root_path.to_string(),
        django_process_handle: Arc::new(Mutex::new(None)),
    });

    Router::new()
        .route("/", get(get_index))
        .route("/admin_start", get(start_django_server))
        .route("/admin_stop", get(stop_django_server))
        .with_state(state)
}
