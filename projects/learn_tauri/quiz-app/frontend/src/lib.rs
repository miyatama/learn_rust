use leptos::{ev::SubmitEvent, prelude::*, suspense::Suspense, task::spawn_local};
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
      <p>"QuizResult"</p>
    }
}

#[component]
fn Settings() -> impl IntoView {
    tracing::debug!("render Settings");
    let navigate = leptos_router::hooks::use_navigate();
    let (get_settings, update_settings) = signal(None::<shared::settings::Settings>);
    let (get_error, update_error) = signal(None::<String>);
    spawn_local({
        async move {
            tracing::debug!("invoke get_settings");
            let result = invoke("get_settings", JsValue::from_str("{}")).await;
            let parsed: Result<
                Result<shared::settings::Settings, shared::error::QuizAppError>,
                serde_wasm_bindgen::Error,
            > = serde_wasm_bindgen::from_value(result);
            match parsed {
                Ok(invoke_result) => match invoke_result {
                    Ok(settings) => {
                        update_settings.set(Some(settings.clone()));
                    }
                    Err(error) => {
                        update_error.set(Some(error.to_string()));
                    }
                },
                Err(error) => {
                    update_error.set(Some(format!("parse error: {:?}", error.to_string())));
                }
            }
        }
    });

    view! {
      <p>"Settings"</p>
      {
        match get_error.get() {
          Some(error) => {
            view! {<p>{error}</p>}.into_view()
          },
          None => {
            view!{<p>{"".to_string()}</p>}.into_view()
          },
        }
      }
      <Suspense fallback=move || view!{<p>"loading data..."</p>}>
      {
        if let Some(settings) = get_settings.get() {
            view!{
              <p>{format!("ランダム設定: {:?}", settings.is_random)}</p>
            }.into_view()
        } else {
          view!{
            <p>{"".to_string()}</p>
          }.into_view()
        }
      }
      </Suspense>

      <button class="menu-btn menu-btn-radius-gradient" on:click={
        let navigate = navigate.clone();
        move |_| navigate("/", Default::default())
      }>"Top"</button>
    }
}
