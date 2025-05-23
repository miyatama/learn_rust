use leptos::{ev::SubmitEvent, prelude::*, suspense::Suspense, task::spawn_local};
use leptos_router::components::{Route, Router, Routes};
use leptos_router_macro::path;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
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
    let navigate = leptos_router::hooks::use_navigate();
    let (get_error, update_error) = signal(None::<String>);
    spawn_local({
        let navigate = navigate.clone();
        async move {
            tracing::debug!("invoke setup_quiz");
            match invoke("setup_quiz", JsValue::from_str("{}")).await {
                Ok(_) => {
                    navigate("/quiz_main", Default::default());
                }
                Err(js_value) => {
                    let parsed: Result<shared::error::QuizAppError, serde_wasm_bindgen::Error> =
                        serde_wasm_bindgen::from_value(js_value.clone());
                    match parsed {
                        Ok(error) => {
                            update_error
                                .set(Some(format!("settings error: {}", error.to_string())));
                        }
                        Err(error) => {
                            update_error.set(Some(format!(
                                "can not parse error: {:?}. js value is {:?}",
                                error.to_string(),
                                &js_value.clone(),
                            )));
                        }
                    }
                }
            };
        }
    });
    view! {
      <p>"QuizSetup"</p>
      <Suspense fallback=move || view!{<p>"loading data..."</p>}>
      {
        move || {
          get_error.get().map(|error| view!{
            <p>{error}</p>
            <button class="menu-btn menu-btn-radius-gradient" on:click={
              let navigate = navigate.clone();
              move |_| navigate("/", Default::default())
            }>"Top"</button>
          })
        }
      }
      </Suspense>
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
            match invoke("get_settings", JsValue::from_str("{}")).await {
                Ok(js_value) => {
                    let parsed: Result<shared::settings::Settings, serde_wasm_bindgen::Error> =
                        serde_wasm_bindgen::from_value(js_value.clone());
                    match parsed {
                        Ok(settings) => {
                            update_settings.set(Some(settings.clone()));
                        }
                        Err(error) => {
                            update_error.set(Some(format!(
                                "settings parse error: {:?}. js value is {:?}",
                                error.to_string(),
                                &js_value.clone(),
                            )));
                        }
                    }
                }
                Err(js_value) => {
                    let parsed: Result<shared::error::QuizAppError, serde_wasm_bindgen::Error> =
                        serde_wasm_bindgen::from_value(js_value.clone());
                    match parsed {
                        Ok(error) => {
                            update_error
                                .set(Some(format!("settings error: {}", error.to_string())));
                        }
                        Err(error) => {
                            update_error.set(Some(format!(
                                "can not parse error: {:?}. js value is {:?}",
                                error.to_string(),
                                &js_value.clone(),
                            )));
                        }
                    }
                }
            }
        }
    });

    view! {
      <p>"Settings"</p>
      {
        move || {
          get_error.get().map(|error| view!{<p>{error}</p>})
        }
      }
      <Suspense fallback=move || view!{<p>"loading data..."</p>}>
      {
        move || {
          get_settings.get().map(|settings| view!{<p>{format!("ランダム設定: {:?}", settings.is_random)}</p>})
        }
      }
      </Suspense>

      <button class="menu-btn menu-btn-radius-gradient" on:click={
        let navigate = navigate.clone();
        move |_| navigate("/", Default::default())
      }>"Top"</button>
    }
}
