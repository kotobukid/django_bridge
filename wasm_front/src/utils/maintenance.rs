use web_sys::window;

pub fn is_maintenance_mode() -> bool {
    if let Some(window) = window() {
        let location = window.location();
        if let Ok(hostname) = location.hostname() {
            return hostname == "localhost"
                || hostname == "127.0.0.1"
                || hostname.contains("local")
                || hostname.contains("dev");
        }
    }
    false
}

pub fn get_fixed_data_server_url() -> String {
    if is_maintenance_mode() {
        "http://localhost:8004".to_string()
    } else {
        // Production環境では無効
        "".to_string()
    }
}
