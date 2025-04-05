mod app;

use app::*;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();

    // https://docs.rs/leptos/latest/leptos/mount/fn.mount_to_body.html
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
