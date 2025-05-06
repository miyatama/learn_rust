# Tauri + Leptos

This template should help get you started developing with Tauri and Leptos.

## Recommended IDE Setup

+ [VS Code](https://code.visualstudio.com/) 
+ [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) 
+ [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
+ [tauri examples](https://github.com/tauri-apps/tauri/tree/dev/examples)
+ docs
  + [tauri](https://docs.rs/tauri/latest/tauri/)
  + [tauri-plugin-store](https://docs.rs/tauri-plugin-store/latest/tauri_plugin_store/)
  + [tracing-subscriber-wasm](https://docs.rs/tracing-subscriber-wasm/latest/tracing_subscriber_wasm/)
+ tauri plugin
  + [window state](https://v2.tauri.app/ja/plugin/window-state/)
+ crates.io
  + [thiserror](https://crates.io/crates/thiserror)
  + [tracing_wasm](https://docs.rs/tracing-wasm/latest/tracing_wasm/)
+ other
  + [Command を使ってみよう](https://zenn.dev/kumassy/books/6e518fe09a86b2/viewer/81f1df)
  + [TauriとLeptosで作るデスクトップアプリ（4）イベントを送受信する](https://zenn.dev/daizutabi/articles/tauri-leptos-04)

# memo

## command定義のエラー

```text
error[E0599]: the method `blocking_kind` exists for reference `&Result<Settings, QuizAppError>`, but its trait bounds were not satisfied
   --> backend\src\lib.rs:8:1
    |
4   | pub struct Settings {
    | ------------------- doesn't satisfy `Settings: IpcResponse`
```

```rust
#[tauri::command]
fn get_settings(app: AppHandle) -> anyhow::Result<Settings, shared::error::QuizAppError> {
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
            store.close_resource();
            Ok(Settings {
                is_random: is_random,
            })
        }
        Err(err) => Err(shared::error::QuizAppError::SettingsError(err.to_string())),
    }
}
```

謎にSettingsを多重定義していたのが原因