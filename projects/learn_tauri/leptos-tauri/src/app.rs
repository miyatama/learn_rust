use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, ev::MouseEvent, prelude::*};
use serde_wasm_bindgen::to_value;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(Serialize, Deserialize)]
struct CountArgs {
    count: i32,
}

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (greet_msg, set_greet_msg) = signal(String::new());

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    let (count, set_count) = signal(0);
    let increase_me = move |ev: MouseEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let count = count.get_untracked();
            let args = to_value(&CountArgs {count}).unwrap();
            let new_value = invoke("increase", args).await.as_f64().unwrap();
            set_count.set(new_value as i32);
        });
    };
    let decrease_me = move |ev: MouseEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let count = count.get_untracked();
            let args = to_value(&CountArgs {count}).unwrap();
            let new_value = invoke("decrease", args).await.as_f64().unwrap();
            set_count.set(new_value as i32);
        });
    };

    view! {
        <div>
        <button on:click=increase_me>"+1"</button>
        <button on:click=decrease_me>"-1"</button>
        <p class:red=move || count.get() < 0>"counter: "{count}</p>
        </div>
    }
}
