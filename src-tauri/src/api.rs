/// let's greet
#[tauri::command]
#[specta::specta]
pub fn greet(name: &str) -> String {
    log::info!("挨拶します。");
    format!("Hello, {}! You've been greeted from Rust!", name)
}
