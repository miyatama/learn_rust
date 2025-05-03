use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use leptos_router::{
    components::{
        Route, Routes, Router,
    },
};
use leptos_router_macro::path;

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
    view! {
      <img src="public/banner.png"/>
      <div class="padding5"/>
      <div class="center menu-btn-radius-gradient-wrap">
        <a href="" class="menu-btn menu-btn-radius-gradient">"START！"</a>
      </div>
      <div class="padding3"/>
      <div class="center menu-btn-radius-gradient-wrap">
        <a href="" class="menu-btn menu-btn-radius-gradient">"SETTINGS!"</a>
      </div>
    }
}
