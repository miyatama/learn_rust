use tauri::{AppHandle, Manager};
use tauri_plugin_store::{Store, StoreBuilder, StoreExt};

#[tauri::command]
fn get_settings(app: AppHandle) -> Result<shared::settings::Settings, shared::error::QuizAppError> {
    tracing::debug!("get_settings: start");
    Ok(shared::settings::Settings { is_random: true })
    /*
    // app: App
    // https://docs.rs/tauri/latest/tauri/struct.App.html
    match app.store("settings.json") {
        Ok(store) => {
            tracing::debug!("get_settings: get store succeed");
            let is_random = store
                .get("is_random".to_string())
                .unwrap()
                .as_bool()
                .unwrap();
            store.close_resource();
            Ok(shared::settings::Settings {
                is_random: is_random,
            })
        }
        Err(err) => Err(shared::error::QuizAppError::SettingsError(err.to_string())),
    }
     */
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let store = app.store("settings.json")?;
            // store.set("is_random-key", serde_json::json!({ "value": true }));
            store.close_resource();
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
