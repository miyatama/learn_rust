// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn increase(count: i32) -> i32 {
    count + 1
}

#[tauri::command]
fn decrease(count: i32) -> i32 {
    count - 1
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, increase, decrease])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
