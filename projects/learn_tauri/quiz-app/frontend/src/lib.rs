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
              <Route path=path!("/quiz_setup") view=QuizSetup/>
              <Route path=path!("/quiz_main") view=QuizMain/>
              <Route path=path!("/quiz_result") view=QuizResult/>
              <Route path=path!("/settings") view=Settings/>
            </Routes>
          </main>
        </Router>
    }
}

#[component]
pub fn Top() -> impl IntoView {
    tracing::debug!("render Top");
    let navigate = leptos_router::hooks::use_navigate();
    view! {
      <img src="public/banner.png"/>
      <div class="padding5"/>
      <button class="menu-btn menu-btn-radius-gradient" on:click={
        let navigate = navigate.clone();
        move |_| navigate("/quiz_setup", Default::default())
      }>"START!"</button>
      <div class="padding3"/>
      <button class="menu-btn menu-btn-radius-gradient" on:click={
        let navigate = navigate.clone();
        move |_| navigate("/settings", Default::default())
      }>"SETTINGS!"</button>
    }
}

#[component]
fn QuizSetup() -> impl IntoView {
    tracing::debug!("render QuizSetup");
    view! {
      <p>"QuizSetup"</p>
    }
}

#[component]
fn QuizMain() -> impl IntoView {
    tracing::debug!("render QuizMain");
    view! {
      <p>"QuizMain"</p>
    }
}

#[component]
fn QuizResult() -> impl IntoView {
    tracing::debug!("render QuizResult");
    view! {
      <p>"QQuizResult"</p>
    }
}

#[component]
fn Settings() -> impl IntoView {
    tracing::debug!("render Settings");
    view! {
      <p>"Settings"</p>
    }
}
