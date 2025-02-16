use std::os::windows::process::CommandExt;
use std::process::{Child, Command};
use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use tokio::sync::Mutex;

pub async fn start_django_server(
    State(process_handle): State<Arc<Mutex<Option<Child>>>>,
) -> impl IntoResponse {
    let mut handle = process_handle.lock().await;

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
        "runserver",
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
            <br /><a href=\"http://{}/admin\" target=\"_blank\">Admin page</a>\
            <br /><a href=\"admin_stop\">Stop Admin page</a>\
            ",
                    admin_origin
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

pub async fn stop_django_server(
    State(process_handle): State<Arc<Mutex<Option<Child>>>>,
) -> impl IntoResponse {
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
            Html("Error stopping Django server")
        } else {
            println!("Django server stopped successfully.");
            Html(
                "Django server stopped successfully.\
            <br />redirect to home page in 2 seconds.\
            <script>setTimeout(() => (location.assign('/')), 2000)</script>"
                    .into(),
            )
        }
    } else {
        println!("No server is running.");
        Html("No server is running.")
    }
}
