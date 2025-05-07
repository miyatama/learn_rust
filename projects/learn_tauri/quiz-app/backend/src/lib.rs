use tauri::{AppHandle, Manager};
use tauri_plugin_store::{Store, StoreBuilder, StoreExt};

#[tauri::command]
fn get_settings(app: AppHandle) -> Result<shared::settings::Settings, shared::error::QuizAppError> {
    tracing::debug!("get_settings: start");
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
                mail_addr: "".to_string(),
            })
        }
        Err(err) => Err(shared::error::QuizAppError::SettingsError(err.to_string())),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_settings])
        .setup(|app| {
            let store = app.store("settings.json")?;
            // store.set("is_random-key", serde_json::json!({ "value": true }));
            store.close_resource();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
