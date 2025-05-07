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
            let is_random = match store.get("is_random") {
                Some(json_value) => json_value.as_bool().unwrap(),
                None => {
                    tracing::debug!("get_settings: is_random not exists");
                    true
                }
            };
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
    tracing::debug!("backend run");
    // app: App
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_settings])
        .setup(|app| {
            tracing::debug!("backend run: create store");
            let store = tauri_plugin_store::StoreBuilder::new(app, "settings.json")
                .default("is_random", true)
                .default("mail_addr", "".to_string())
                .build()?;
            store.save()?;
            store.close_resource();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
