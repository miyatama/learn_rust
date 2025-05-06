use leptos::prelude::*;
use quiz_app_ui::App;

fn main() {
    tracing_wasm::set_as_global_default();
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
