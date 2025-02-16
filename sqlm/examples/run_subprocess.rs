use std::process::Command;

fn main() {
    let output = if cfg!(target_os = "windows") {
        Command::new("../table_definition/.venv/Scripts/python.exe")
            .args(["../table_definition/manage.py", "runserver", "localhost:8001"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process")
    };

    let hello = output.stdout;
    println!("Hello: {}", hello.into_iter().map(|x| x as char).collect::<String>());
}
