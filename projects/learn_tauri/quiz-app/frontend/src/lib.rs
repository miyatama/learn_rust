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
    // let (get_error, update_error) = signal(None::<String>);
    let (get_error, update_error) = signal(Some("きままにとべ".to_string()));
    spawn_local({
        async move {
            tracing::debug!("invoke get_settings");
            update_error.set(Some("どこまでもゆけ".to_string()));
            // get_settingsがエラーを返したときの対応が謎。
            let result = invoke("get_settings", JsValue::from_str("{}")).await;
            tracing::debug!("invoke result: {:?}", &result);
            update_error.set(Some("100年先も".to_string()));
            update_error.set(Some(format!("js value is {:?}", &result.clone())));
            let parsed: Result<shared::settings::Settings, serde_wasm_bindgen::Error> =
                serde_wasm_bindgen::from_value(result.clone());
            match parsed {
                Ok(settings) => {
                    tracing::debug!("get_settings result: {:?}", settings);
                    update_settings.set(Some(settings.clone()));
                }
                Err(_) => {
                    let parsed: Result<shared::error::QuizAppError, serde_wasm_bindgen::Error> =
                        serde_wasm_bindgen::from_value(result.clone());
                    match parsed {
                        Ok(error) => {
                            tracing::debug!("get_settings parse error: {:?}", error.to_string());
                            update_error
                                .set(Some(format!("quiz app error: {:?}. ", error.to_string())));
                        }
                        Err(error) => {
                            tracing::debug!("get_settings parse error: {:?}", error.to_string());
                            update_error.set(Some(format!(
                                "parse error: {:?}. js value is {:?}",
                                error.to_string(),
                                &result
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
