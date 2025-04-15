mod api;
use api::get_contacts;
use leptos::{
    either::Either, error::ErrorBoundary, ev::SubmitEvent, prelude::*, suspense::Suspense,
    task::spawn_local,
};
use leptos_router::{
    components::{
        Form, Outlet, ParentRoute, ProtectedRoute, Redirect, Route, Router, Routes,
        RoutingProgress, A,
    },
    hooks::{use_navigate, use_params, use_query_map},
    params::Params,
};
use leptos_router_macro::path;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use wasm_bindgen::prelude::*;

const MANY_COUNTERS: usize = 5;
type CounterHolder = Vec<(usize, ArcRwSignal<i32>)>;

#[derive(Copy, Clone)]
struct CounterUpdater {
    set_counters: WriteSignal<CounterHolder>,
}

#[derive(Copy, Clone)]
struct SmallcapsContext(WriteSignal<bool>);

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
struct NewGreetArgs<'a> {
    name: &'a str,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct ExampleContext(i32);

// componentマクロを指定すると関数名のタグが使えるようになる(main.rsで使ってるAppタグがこれ)
#[component]
pub fn App() -> impl IntoView {
    tracing::info!("rendering <App>");
    provide_context(ExampleContext(0));
    let (logged_in, set_logged_in) = signal(true);
    let (is_routing, set_is_routing) = signal(false);

    let (name, set_name) = signal(String::new());
    let (greet_msg, set_greet_msg) = signal(String::new());

    let (new_name, set_new_name) = signal(String::new());
    let (new_greet_msg, set_new_greet_msg) = signal(String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let update_new_name = move |ev| {
        let v = event_target_value(&ev);
        set_new_name.set(v);
    };

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

    let new_greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = new_name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
            let new_msg = invoke("new_greet", args).await.as_string().unwrap();
            set_new_greet_msg.set(new_msg);
        });
    };

    let (next_counter_id, set_next_counter_id) = signal(0);
    let (counters, set_counters) = signal::<CounterHolder>(vec![]);
    provide_context(CounterUpdater { set_counters });

    let add_counter = move |_| {
        let id = next_counter_id.get();
        let sig = ArcRwSignal::new(0);
        set_counters.update(move |counters| counters.push((id, sig)));
        set_next_counter_id.update(|id| *id += 1);
    };

    let add_many_counters = move |_| {
        let next_id = next_counter_id.get();
        let new_counters = (next_id..next_id + MANY_COUNTERS).map(|id| {
            let sig = ArcRwSignal::new(0);
            (id, sig)
        });
        set_counters.update(move |counters| counters.extend(new_counters));
        set_next_counter_id.update(|id| *id += MANY_COUNTERS);
    };
    let clear_counters = move |_| {
        set_counters.update(|counters| counters.clear());
    };

    // Error Boundary
    let (error_boundary_value, set_error_boundary_value) = signal("".parse::<i32>());

    view! {
        <Router set_is_routing>
          <div class="routing-progress">
            <RoutingProgress is_routing max_time=std::time::Duration::from_millis(200) />
          </div>
          <nav>
            <A href="/">"Contacts"</A>
            <A href="/about">"About"</A>
            <A href="/settings">"Settings"</A>
            <A href="/indirect-home">"Home"</A>
            <button on:click=move |_| {set_logged_in.update(|n| *n = !*n)}>{move || if logged_in.get() {"Log Out"} else {"Log In"}}</button>
          </nav>
          <main class="container">
              <Routes transition=true fallback=|| "This page could not be found.">
                <Route path=path!("about") view=About />
                <ProtectedRoute
                  path=path!("settings")
                  condition=move || Some(logged_in.get())
                  redirect_path=|| "/"
                  view=Settings
                />
                <Route path=path!("redirect-home") view=|| view!{ <Redirect path="/"/>}/>
                <ContactRoutes/>
              </Routes>
              <h1>"Welcome to Tauri + Leptos"</h1>
              <div class="row">
                  <a href="https://tauri.app" target="_blank">
                      <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                  </a>
                  <a href="https://docs.rs/leptos/" target="_blank">
                      <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
                  </a>
              </div>
              <p>"Click on the Tauri and Leptos logos to learn more."</p>

              <form class="row" on:submit=greet>
                  <input
                      id="greet-input"
                      placeholder="Enter a name..."
                      on:input=update_name
                  />
                  <button type="submit">"Greet"</button>
              </form>
              <p>{ move || greet_msg.get() }</p>

              <form class="row" on:submit=new_greet>
                  <input
                      id="greet-input"
                      placeholder="Enter a name..."
                      on:input=update_new_name
                  />
                  <button type="submit">"New Greet"</button>
              </form>
              <p>{ move || new_greet_msg.get() }</p>
              <FetchExample/>
              <h1>Parent Child</h1>
              <ParentChild/>
              <h1>Error Handling</h1>
              <label>type integer
                <input
                  type="number"
                  value=move || error_boundary_value.get().unwrap_or_default()
                  on:input:target=move |ev| set_error_boundary_value.set(ev.target().value().parse::<i32>())
                />
                <ErrorBoundary fallback=|errors| {
                  let errors = errors.clone();
                  view! {
                      <div class="error">
                        <p>not an integer error</p>
                        <ul>
                          {
                              move || {
                                  errors
                                  .read()
                                  .iter()
                                  .map(|(_, e)| view!{<li>{e.to_string()}</li>})
                                  .collect::<Vec<_>>()
                              }
                          }
                        </ul>
                      </div>
                  }
                }>
                  <p>you entered <strong>{error_boundary_value}</strong></p>
                </ErrorBoundary>
              </label>

              <button on:click=add_counter>"Add Counter"</button>
              <button on:click=add_many_counters>{format!("Add {MANY_COUNTERS} Counters")}</button>
              <button on:click=clear_counters>"Clear Counters"</button>
              <p>
              "Total: "
              <span data-testid="total">
              {
                  move || {
                      counters.get().iter().map(|(_, count) | count.get()).sum::<i32>().to_string()
                  }
              }
              </span> " from "
              <span data-testid="counters">{move || counters.get().len().to_string()}</span>
              " counters."
              </p>
              <ul>
                <For
                  each=move || counters.get()
                  key=|counter| counter.0
                  children=move |(id, value)| {
                    view!{ <Counter id value/>}
                  }
                />
              </ul>
          </main>
        </Router>
    }
}

#[component]
fn Counter(id: usize, value: ArcRwSignal<i32>) -> impl IntoView {
    let value = RwSignal::from(value);
    let CounterUpdater { set_counters } = use_context().unwrap();
    view! {
        <li>
        <button on:click=move |_| value.update(move |value| *value -= 1)>"-1"</button>
        <input
        type="text"
        prop:value=value
        on:input:target=move |ev| {
            value.set(ev.target().value().parse::<i32>().unwrap_or_default())
        }
        />
        <span>{value}</span>
        <button on:click=move |_| value.update(move |value| *value += 1)>"+1"</button>
        <button on:click=move |_| {
            set_counters.update(move |counters| counters.retain(|(counter_id, _)| counter_id != &id))
        }>"x"</button>
        </li>
    }
}

/**
 *  ex) leptos::mount::mount_to_body(|| greeting())
 */
pub fn greeting() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (greet_msg, set_greet_msg) = signal(String::new());
    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };
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
    leptos::html::form()
        .on(leptos::ev::submit, move |ev| update_name(ev))
        .child((leptos::html::input(), leptos::html::button()))
}

#[component]
fn ParentChild() -> impl IntoView {
    // Parent Child
    let (red, set_red) = signal(false);
    let (right, set_right) = signal(false);
    let (italics, set_italics) = signal(false);
    let (smallcaps, set_smallcaps) = signal(false);
    provide_context(SmallcapsContext(set_smallcaps));

    view! {
        <main>
          <p
            class:red=red
            class:right=right
            class:italics=italics
            class:smallcaps=smallcaps
          >lorem ipsum sit dolor amet.</p>
          <ButtonA setter=set_red/>
          <ButtonB on_click=move |_| set_right.update(|value| *value=!*value)/>
          <ButtonC on:click=move |_| set_italics.update(|value| *value=!*value)/>
          <ButtonD/>
        </main>
    }
}

#[component]
fn ButtonA(setter: WriteSignal<bool>) -> impl IntoView {
    view! {
        <button on:click=move |_| setter.update(|value| *value = !*value)>"Toggle Red"</button>
    }
}

#[component]
fn ButtonB(on_click: impl FnMut(web_sys::MouseEvent) + 'static) -> impl IntoView {
    view! {
        <button on:click=on_click>"Toggle Right"</button>
    }
}

#[component]
fn ButtonC() -> impl IntoView {
    view! {
        <button>"Toggle Italics"</button>
    }
}

#[component]
fn ButtonD() -> impl IntoView {
    let setter = use_context::<SmallcapsContext>().unwrap().0;
    view! {
        <button on:click=move |_| {
            setter.update(|value| *value = !*value)
        }>"Toggle Small Caps"</button>
    }
}

/**
 * fetch example
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cat {
    url: String,
}

#[derive(Error, Clone, Debug)]
pub enum CatError {
    #[error("please request more than zero cats.")]
    NonZeroCats,
}

type CatCount = usize;

async fn fetch_cats(count: CatCount) -> Result<Vec<String>, Error> {
    if count > 0 {
        gloo_timers::future::TimeoutFuture::new(1000).await;
        let res = reqwasm::http::Request::get(&format!(
            "https://api.thecatapi.com/v1/images/search?limit={count}",
        ))
        .send()
        .await?
        .json::<Vec<Cat>>()
        .await?
        .into_iter()
        .take(count)
        .map(|cat| cat.url)
        .collect::<Vec<_>>();
        Ok(res)
    } else {
        Err(CatError::NonZeroCats)?
    }
}

#[component]
fn FetchExample() -> impl IntoView {
    let (cat_count, set_cat_count) = signal::<CatCount>(1);
    let cats = LocalResource::new(move || fetch_cats(cat_count.get()));
    let fallback = move |errors: ArcRwSignal<Errors>| {
        let error_list = move || {
            errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| view! {<li>{e.to_string()}</li>})
                    .collect::<Vec<_>>()
            })
        };
        view! {
            <div class="error">
              <h2>"Error"</h2>
              <ul>
                {error_list}
              </ul>
            </div>
        }
    };
    view! {
        <h1>"Fetch Example"</h1>
        <div>
          <label>
            "how many cats would you like?"
            <input
              type="number"
              prop:value=move || cat_count.get().to_string()
              on:input:target=move |ev| {
                let val = ev.target().value().parse::<CatCount>().unwrap_or(0);
                set_cat_count.set(val);
              }
            />
          </label>
          <Transition fallback=|| view!{<div>"loading..."</div>}>
            <ErrorBoundary fallback>
              <ul>
                {move || Suspend::new(async move {
                    cats.await
                    .map(|cats|{
                        cats
                        .iter()
                        .map(|cat|{
                            view!{
                                <li>
                                  <img src=cat.clone() />
                                </li>
                            }
                        })
                        .collect::<Vec<_>>()
                    })
                })}
              </ul>
            </ErrorBoundary>
          </Transition>
        </div>
    }
}

#[component(transparent)]
pub fn ContactRoutes() -> impl leptos_router::MatchNestedRoutes + Clone {
    tracing::info!("rendering <ContactRoutes>");
    view! {
        <ParentRoute path=path!("") view=ContactList>
          <Route path=path!("/") view=|| "Select a contact." />
          <Route path=path!("/:id") view=Contact />
        </ParentRoute>
    }
    .into_inner()
}

#[component]
pub fn ContactList() -> impl IntoView {
    tracing::info!("rendering <ContactList>");
    provide_context(ExampleContext(42));
    Owner::on_cleanup(|| {
        tracing::info!("cleanup <ContactList>");
    });
    let query = use_query_map();
    let search = Memo::new(move |_| query.read().get("q").unwrap_or_default());
    let contacts = AsyncDerived::new(move || {
        leptos::logging::log!("reloading contacts");
        get_contacts(search.get())
    });
    let contacts = move || {
        Suspend::new(async move {
            contacts
                .await
                .into_iter()
                .map(|contact| {
                    view! {
                        <li>
                            <A href=contact.id.to_string()>
                              <span> {contact.first_name} " " {contact.last_name} </span>
                            </A>
                        </li>
                    }
                })
                .collect::<Vec<_>>()
        })
    };
    view! {
        <div class="contact-list">
           <h1>"Contacts"</h1>
           <Suspense fallback=move || view!{<p>"loading contacts..."</p>}>
             <ul>{contacts}</ul>
           </Suspense>
           <Outlet/>
        </div>
    }
}

#[derive(PartialEq, Clone, Debug, Params)]
pub struct ContactParams {
    id: Option<usize>,
}

#[component]
pub fn Contact() -> impl IntoView {
    tracing::info!("rendering <Contact/>");

    tracing::info!(
        "ExampleContext should be Some(42). It is {:?}",
        use_context::<ExampleContext>()
    );

    Owner::on_cleanup(|| {
        tracing::info!("cleaning up <Contact/>");
    });

    let params = use_params::<ContactParams>();

    let contact = AsyncDerived::new(move || {
        api::get_contact(
            params
                .get()
                .map(|params| params.id.unwrap_or_default())
                .ok(),
        )
    });

    let contact_display = move || {
        Suspend::new(async move {
            match contact.await {
                None => Either::Left(view! { <p>"No contact with this ID was found."</p> }),
                Some(contact) => Either::Right(view! {
                    <section class="card">
                        <h1>{contact.first_name} " " {contact.last_name}</h1>
                        <p>{contact.address_1} <br /> {contact.address_2}</p>
                    </section>
                }),
            }
        })
    };

    view! {
        <div class="contact">
            <Transition fallback=move || {
                view! { <p>"Loading..."</p> }
            }>{contact_display}</Transition>
        </div>
    }
}

#[component]
pub fn About() -> impl IntoView {
    tracing::info!("rendering <About/>");

    Owner::on_cleanup(|| {
        tracing::info!("cleaning up <About/>");
    });

    tracing::info!(
        "ExampleContext should be Some(0). It is {:?}",
        use_context::<ExampleContext>()
    );

    // use_navigate allows you to navigate programmatically by calling a function
    let navigate = use_navigate();

    // note: this is just an illustration of how to use `use_navigate`
    // <button on:click> to navigate is an *anti-pattern*
    // you should ordinarily use a link instead,
    // both semantically and so your link will work before WASM loads
    view! {
        <button on:click=move |_| navigate("/", Default::default())>"Home"</button>
        <h1>"About"</h1>
        <p>
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
        </p>
    }
}

#[component]
pub fn Settings() -> impl IntoView {
    tracing::info!("rendering <Settings/>");

    Owner::on_cleanup(|| {
        tracing::info!("cleaning up <Settings/>");
    });

    view! {
        <h1>"Settings"</h1>
        <Form action="">
            <fieldset>
                <legend>"Name"</legend>
                <input type="text" name="first_name" placeholder="First" />
                <input type="text" name="last_name" placeholder="Last" />
            </fieldset>
            <input type="submit" />
            <p>
                "This uses the " <code>"<Form/>"</code>
                " component, which enhances forms by using client-side navigation for "
                <code>"GET"</code> " requests, and client-side requests for " <code>"POST"</code>
                " requests, without requiring a full page reload."
            </p>
        </Form>
    }
}
