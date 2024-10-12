use crate::utils::app_dir;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub async fn get_app_dir() -> String {
    app_dir().display().to_string()
}
