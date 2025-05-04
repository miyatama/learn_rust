use tauri::{AppHandle, Manager};
use tauri_plugin_store::{Store, StoreBuilder, StoreExt};

pub struct Settings {
    is_random: bool,
}

#[tauri::command]
fn get_settings(app: AppHandle) -> Result<Settings> {
    // app: App
    // https://docs.rs/tauri/latest/tauri/struct.App.html
    match app.store("settings.json") {
        Ok(store) => {
            /*
            let mut store = StoreBuilder::new(app.path_resolver().app_dir().unwrap.join("settings.json"));
            store.insert("key", "value".into()).unwrap();
            store.save().unwrap();
             */
            let is_random = store.get("is_random".to_string()).unwrap().as_bool().unwrap();
            Ok(Settings {
                is_random: is_random,
            })
        }
        Err(err) => Err(err),
    }
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
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
