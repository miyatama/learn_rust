# Tauri + Leptos

This template should help get you started developing with Tauri and Leptos.

Leptos + Tauri構成でSQL使おうとしたら沼

## Recommended IDE Setup

+ [VS Code](https://code.visualstudio.com/) 
+ [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) 
+ [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
+ [tauri examples](https://github.com/tauri-apps/tauri/tree/dev/examples)
+ docs
  + [tauri](https://docs.rs/tauri/latest/tauri/)
  + [tauri-plugin-store](https://docs.rs/tauri-plugin-store/latest/tauri_plugin_store/)
  + [tracing-subscriber-wasm](https://docs.rs/tracing-subscriber-wasm/latest/tracing_subscriber_wasm/)
  + [tauri-plugin-sql](https://docs.rs/tauri-plugin-sql/2.2.0/tauri_plugin_sql/)
+ tauri plugin
  + [window state](https://v2.tauri.app/ja/plugin/window-state/)
  + [SQL](https://v2.tauri.app/ja/plugin/sql/)
+ crates.io
  + [thiserror](https://crates.io/crates/thiserror)
  + [tracing_wasm](https://docs.rs/tracing-wasm/latest/tracing_wasm/)
  + [tauri-pluigin-sql](https://crates.io/crates/tauri-plugin-sql)
+ other
  + [Command を使ってみよう](https://zenn.dev/kumassy/books/6e518fe09a86b2/viewer/81f1df)
  + [TauriとLeptosで作るデスクトップアプリ（4）イベントを送受信する](https://zenn.dev/daizutabi/articles/tauri-leptos-04)
  + [tauriでテトリスっぽいものを作ってみる](https://catalina1344.hatenablog.jp/entry/2023/04/12/105021)
  + [Tauri error handling recipes](https://tbt.qkation.com/posts/tauri-error-handling/)

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

## invoke呼び出し時にfrontend側でエラー出る

```text
Uncaught TypeError: Cannot read properties of undefined (reading 'then')
```

<details><summary>全文</summary><div>

```text
Uncaught TypeError: Cannot read properties of undefined (reading 'then')
    at imports.wbg.__wbg_then_48b406749878a531 (quiz_app_bin-55d2355bcc0a763c.js:920:26)
    at quiz_app_bin-e54063a48f62d774.wasm.__wbg_then_48b406749878a531 externref shim (quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x257033)
    at quiz_app_bin-e54063a48f62d774.wasm.js_sys::Promise::then2::h5e0ac9fde5fd8206 (quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x1bf682)
    at quiz_app_bin-e54063a48f62d774.wasm.<wasm_bindgen_futures::JsFuture as core::convert::From<js_sys::Promise>>::from::h2a8dd4909b95cccd (quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x9440c)
    at quiz_app_bin-e54063a48f62d774.wasm.quiz_app_ui::invoke_no_args_with_result::{{closure}}::h3aebbc311ed34071 (quiz_app_bin-55d2355bcc0a763c_bg.wasm:0xc197e)
    at quiz_app_bin-e54063a48f62d774.wasm.quiz_app_ui::__Settings::{{closure}}::hdc44c2604670d922 (quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x4d5db)
    at quiz_app_bin-e54063a48f62d774.wasm.<core::pin::Pin<P> as core::future::future::Future>::poll::h41288a6cc2c5eb6e (quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x1f2abe)
    at quiz_app_bin-e54063a48f62d774.wasm.wasm_bindgen_futures::task::singlethread::Task::run::h18f231f89865aabc (quiz_app_bin-55d2355bcc0a763c_bg.wasm:0xfb591)
    at quiz_app_bin-e54063a48f62d774.wasm.wasm_bindgen_futures::queue::QueueState::run_all::h6f17f415b762bfd4 (quiz_app_bin-55d2355bcc0a763c_bg.wasm:0xda9c3)
    at quiz_app_bin-e54063a48f62d774.wasm.wasm_bindgen_futures::queue::Queue::new::{{closure}}::h9dda635d0c8dd683 (quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x1f2bb5)
imports.wbg.__wbg_then_48b406749878a531 @ quiz_app_bin-55d2355bcc0a763c.js:920
$__wbg_then_48b406749878a531 externref shim @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x257033
$js_sys::Promise::then2::h5e0ac9fde5fd8206 @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x1bf682
$<wasm_bindgen_futures::JsFuture as core::convert::From<js_sys::Promise>>::from::h2a8dd4909b95cccd @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x9440c
$quiz_app_ui::invoke_no_args_with_result::{{closure}}::h3aebbc311ed34071 @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0xc197e
$quiz_app_ui::__Settings::{{closure}}::hdc44c2604670d922 @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x4d5db
$<core::pin::Pin<P> as core::future::future::Future>::poll::h41288a6cc2c5eb6e @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x1f2abe
$wasm_bindgen_futures::task::singlethread::Task::run::h18f231f89865aabc @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0xfb591
$wasm_bindgen_futures::queue::QueueState::run_all::h6f17f415b762bfd4 @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0xda9c3
$wasm_bindgen_futures::queue::Queue::new::{{closure}}::h9dda635d0c8dd683 @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x1f2bb5
$<dyn core::ops::function::FnMut<(A,)>+Output = R as wasm_bindgen::closure::WasmClosure>::describe::invoke::h42f3ad8d7adfa0f7 @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x1c82d0
$closure682 externref shim @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x257fad
__wbg_adapter_45 @ quiz_app_bin-55d2355bcc0a763c.js:235
real @ quiz_app_bin-55d2355bcc0a763c.js:121
quiz_app_bin-55d2355bcc0a763c.js:536  panicked at C:\Users\nmiya\.cargo\registry\src\index.crates.io-6f17d22bba15001f\leptos_router-0.7.8\src\lib.rs:202:34:
called `Result::unwrap()` on an `Err` value: JsValue(TypeError: window.__TAURI__.core.invoke_no_args_with_result is not a function
TypeError: window.__TAURI__.core.invoke_no_args_with_result is not a function
    at http://localhost:1420/quiz_app_bin-55d2355bcc0a763c.js:645:43
    at handleError (http://localhost:1420/quiz_app_bin-55d2355bcc0a763c.js:29:18)
    at imports.wbg.__wbg_invokenoargswithresult_a43e270b27cc5806 (http://localhost:1420/quiz_app_bin-55d2355bcc0a763c.js:644:85)
    at quiz_app_bin-e54063a48f62d774.wasm.__wbg_invokenoargswithresult_a43e270b27cc5806 externref shim (http://localhost:1420/quiz_app_bin-55d2355bcc0a763c_bg.wasm:wasm-function[10477]:0x257973)
    at quiz_app_bin-e54063a48f62d774.wasm.quiz_app_ui::invoke_no_args_with_result::{{closure}}::h3aebbc311ed34071 (http://localhost:1420/quiz_app_bin-55d2355bcc0a763c_bg.wasm:wasm-function[555]:0xc194d)
    at quiz_app_bin-e54063a48f62d774.wasm.quiz_app_ui::__Settings::{{closure}}::hdc44c2604670d922 (http://localhost:1420/quiz_app_bin-55d2355bcc0a763c_bg.wasm:wasm-function[209]:0x4d5db)
    at quiz_app_bin-e54063a48f62d774.wasm.<core::pin::Pin<P> as core::future::future::Future>::poll::h41288a6cc2c5eb6e (http://localhost:1420/quiz_app_bin-55d2355bcc0a763c_bg.wasm:wasm-function[4856]:0x1f2abe)
    at quiz_app_bin-e54063a48f62d774.wasm.wasm_bindgen_futures::task::singlethread::Task::run::h18f231f89865aabc (http://localhost:1420/quiz_app_bin-55d2355bcc0a763c_bg.wasm:wasm-function[916]:0xfb591)
    at quiz_app_bin-e54063a48f62d774.wasm.wasm_bindgen_futures::queue::QueueState::run_all::h6f17f415b762bfd4 (http://localhost:1420/quiz_app_bin-55d2355bcc0a763c_bg.wasm:wasm-function[693]:0xda9c3)
    at quiz_app_bin-e54063a48f62d774.wasm.wasm_bindgen_futures::queue::Queue::new::{{closure}}::h9dda635d0c8dd683 (http://localhost:1420/quiz_app_bin-55d2355bcc0a763c_bg.wasm:wasm-function[4858]:0x1f2bb5))

Stack:

Error
    at imports.wbg.__wbg_new_8a6f238a6ece86ea (http://localhost:1420/quiz_app_bin-55d2355bcc0a763c.js:735:21)
    at quiz_app_bin-e54063a48f62d774.wasm.__wbg_new_8a6f238a6ece86ea externref shim (http://localhost:1420/quiz_app_bin-55d2355bcc0a763c_bg.wasm:wasm-function[10566]:0x258197)
    at quiz_app_bin-e54063a48f62d774.wasm.console_error_panic_hook::Error::new::hf98c62e58feafbd8 (http://localhost:1420/quiz_app_bin-55d2355bcc0a763c_bg.wasm:wasm-function[6555]:0x21e99c)
    at quiz_app_bin-e54063a48f62d774.wasm.console_error_panic_hook::hook_impl::hab0807a5b1d16b31 (http://localhost:1420/quiz_app_bin-55d2355bcc0a763c_bg.wasm:wasm-function[1444]:0x137a7c)
    at quiz_app_bin-e54063a48f62d774.wasm.console_error_panic_hook::hook::h2f5cba2485098170 (http://localhost:1420/quiz_app_bin-55d2355bcc0a763c_bg.wasm:wasm-function[9075]:0x24851b)
    at quiz_app_bin-e54063a48f62d774.wasm.core::ops::function::Fn::call::h48bb22077ce170d6 (http://localhost:1420/quiz_app_bin-55d2355bcc0a763c_bg.wasm:wasm-function[7924]:0x237fff)
    at quiz_app_bin-e54063a48f62d774.wasm.std::panicking::rust_panic_with_hook::he163a328f096b027 (http://localhost:1420/quiz_app_bin-55d2355bcc0a763c_bg.wasm:wasm-function[3481]:0x1bda92)
    at quiz_app_bin-e54063a48f62d774.wasm.std::panicking::begin_panic_handler::{{closure}}::h9b4576c502f796bb (http://localhost:1420/quiz_app_bin-55d2355bcc0a763c_bg.wasm:wasm-function[4201]:0x1dc9c7)
    at quiz_app_bin-e54063a48f62d774.wasm.std::sys::backtrace::__rust_end_short_backtrace::h9dcf082627c76851 (http://localhost:1420/quiz_app_bin-55d2355bcc0a763c_bg.wasm:wasm-function[10734]:0x258aa5)
    at quiz_app_bin-e54063a48f62d774.wasm.rust_begin_unwind (http://localhost:1420/quiz_app_bin-55d2355bcc0a763c_bg.wasm:wasm-function[9951]:0x2533d5)


imports.wbg.__wbg_error_7534b8e9a36f1ab4 @ quiz_app_bin-55d2355bcc0a763c.js:536
$console_error_panic_hook::error::h1397ac4034b608e0 @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x1e38c7
$console_error_panic_hook::hook_impl::hab0807a5b1d16b31 @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x137b68
$console_error_panic_hook::hook::h2f5cba2485098170 @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x24851b
$core::ops::function::Fn::call::h48bb22077ce170d6 @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x237fff
$std::panicking::rust_panic_with_hook::he163a328f096b027 @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x1bda92
$std::panicking::begin_panic_handler::{{closure}}::h9b4576c502f796bb @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x1dc9c7
$std::sys::backtrace::__rust_end_short_backtrace::h9dcf082627c76851 @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x258aa5
$rust_begin_unwind @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x2533d5
$core::panicking::panic_fmt::hf09e831ea9f1651a @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x253401
$core::result::unwrap_failed::h1da5b2ca20e25174 @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x1f1617
$leptos_router::view_transition::start_view_transition::{{closure}}::he76be25e036c33b3 @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0xd923f
$<alloc::boxed::Box<F,A> as core::ops::function::FnMut<Args>>::call_mut::h2d7bbae1e65e3a0a @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x21b20b
$<dyn core::ops::function::FnMut<(A,)>+Output = R as wasm_bindgen::closure::WasmClosure>::describe::invoke::h42f3ad8d7adfa0f7 @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x1c82d0
$closure682 externref shim @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x257fad
__wbg_adapter_45 @ quiz_app_bin-55d2355bcc0a763c.js:235
real @ quiz_app_bin-55d2355bcc0a763c.js:121
Promise.then
imports.wbg.__wbg_then_44b73946d2fb3e7d @ quiz_app_bin-55d2355bcc0a763c.js:916
$__wbg_then_44b73946d2fb3e7d externref shim @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x25761d
$js_sys::Promise::then::h227a098c501536ac @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x1db2d9
$leptos_router::view_transition::start_view_transition::h42ddcffe185eed97 @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x6198c
$<leptos_router::nested_router::NestedRoutesView<Loc,Defs,FalFn> as tachys::view::Render>::rebuild::{{closure}}::hc1c1d6817be9d377 @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x9b3cd
$<core::pin::Pin<P> as core::future::future::Future>::poll::h41288a6cc2c5eb6e @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x1f2abe
$wasm_bindgen_futures::task::singlethread::Task::run::h18f231f89865aabc @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0xfb591
$wasm_bindgen_futures::queue::QueueState::run_all::h6f17f415b762bfd4 @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0xda9c3
$wasm_bindgen_futures::queue::Queue::new::{{closure}}::h9dda635d0c8dd683 @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x1f2bb5
$<dyn core::ops::function::FnMut<(A,)>+Output = R as wasm_bindgen::closure::WasmClosure>::describe::invoke::h42f3ad8d7adfa0f7 @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x1c82d0
$closure682 externref shim @ quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x257fad
__wbg_adapter_45 @ quiz_app_bin-55d2355bcc0a763c.js:235
real @ quiz_app_bin-55d2355bcc0a763c.js:121
quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x258d27  Uncaught (in promise) RuntimeError: unreachable
    at quiz_app_bin-e54063a48f62d774.wasm.__rust_start_panic (quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x258d27)
    at quiz_app_bin-e54063a48f62d774.wasm.rust_panic (quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x25860b)
    at quiz_app_bin-e54063a48f62d774.wasm.std::panicking::rust_panic_with_hook::he163a328f096b027 (quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x1bdabd)
    at quiz_app_bin-e54063a48f62d774.wasm.std::panicking::begin_panic_handler::{{closure}}::h9b4576c502f796bb (quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x1dc9c7)
    at quiz_app_bin-e54063a48f62d774.wasm.std::sys::backtrace::__rust_end_short_backtrace::h9dcf082627c76851 (quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x258aa5)
    at quiz_app_bin-e54063a48f62d774.wasm.rust_begin_unwind (quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x2533d5)
    at quiz_app_bin-e54063a48f62d774.wasm.core::panicking::panic_fmt::hf09e831ea9f1651a (quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x253401)
    at quiz_app_bin-e54063a48f62d774.wasm.core::result::unwrap_failed::h1da5b2ca20e25174 (quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x1f1617)
    at quiz_app_bin-e54063a48f62d774.wasm.leptos_router::view_transition::start_view_transition::{{closure}}::he76be25e036c33b3 (quiz_app_bin-55d2355bcc0a763c_bg.wasm:0xd923f)
    at quiz_app_bin-e54063a48f62d774.wasm.<alloc::boxed::Box<F,A> as core::ops::function::FnMut<Args>>::call_mut::h2d7bbae1e65e3a0a (quiz_app_bin-55d2355bcc0a763c_bg.wasm:0x21b20b)
```

</div></detials>