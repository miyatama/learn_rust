use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use leptos_router::components::{Route, Router, Routes};
use leptos_router_macro::path;
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

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
          <main class="container">
            <Routes transition=true fallback=|| "this page could not be found">
              <Route path=path!("/") view=Top />
            </Routes>
          </main>
        </Router>
    }
}

#[component]
pub fn Top() -> impl IntoView {
    tracing::debug!("render Top");
    let navigate = leptos_router::hooks::use_navigate();
    let on_click_start = |_| {
        let navigate = navigate.clone();
        navigate("/quiz_main?step=0", Default::default());
    };
    let on_click_settings = |_| {
        let navigate = navigate.clone();
        navigate("/setting", Default::default());
    };
    view! {
      <img src="public/banner.png"/>
      <div class="padding5"/>
      <button class="menu-btn menu-btn-radius-gradient" on:click=on_click_start>"START!"</button>
      <div class="padding3"/>
      <button class="menu-btn menu-btn-radius-gradient" on:click=on_click_settings>"SETTINGS!"</button>
    }
}
