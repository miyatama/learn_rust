use leptos::task::spawn_local;
use leptos::{
    ev::SubmitEvent, 
    error::ErrorBoundary,
    prelude::*,
};
use serde::{Deserialize, Serialize};
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

// componentマクロを指定すると関数名のタグが使えるようになる(main.rsで使ってるAppタグがこれ)
#[component]
pub fn App() -> impl IntoView {
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
        <main class="container">
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
fn ButtonA(
    setter: WriteSignal<bool>,
) -> impl IntoView {
    view!{
        <button on:click=move |_| setter.update(|value| *value = !*value)>"Toggle Red"</button>
    }
}

#[component]
fn ButtonB(
    on_click: impl FnMut(web_sys::MouseEvent) + 'static,
) -> impl IntoView {
    view!{
        <button on:click=on_click>"Toggle Right"</button>
    }
}

#[component]
fn ButtonC() -> impl IntoView {
    view!{
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