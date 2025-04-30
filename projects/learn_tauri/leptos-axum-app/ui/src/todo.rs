// なぜかモジュールが見えない
// use crate::error_template::ErrorTemplate;
use leptos::{either::Either, prelude::*};
use serde::{Deserialize, Serialize};
use server_fn::ServerFnError;

#[allow(dead_code)]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <link rel="stylesheet" id="leptos" href="/pkg/todo_app_sqlite_axum.css"/>
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico"/>
            </head>
            <body>
                <TodoApp/>
            </body>
        </html>
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Todo {
    id: u16,
    title: String,
    completed: bool,
}

#[cfg(feature = "ssr")]
pub mod ssr {
    // use http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode};
    use leptos::server_fn::ServerFnError;
    use sqlx::{Connection, SqliteConnection};

    pub async fn db() -> Result<SqliteConnection, ServerFnError> {
        Ok(SqliteConnection::connect("sqlite:Todos.db").await?)
    }
}

#[server]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    use self::ssr::*;
    use http::request::Parts;

    // this is just an example of how to access server context injected in the handlers
    let req_parts = use_context::<Parts>();

    if let Some(req_parts) = req_parts {
        println!("Uri = {:?}", req_parts.uri);
    }

    use futures::TryStreamExt;

    let mut conn = db().await?;

    let mut todos = Vec::new();
    let mut rows = sqlx::query_as::<_, Todo>("SELECT * FROM todos").fetch(&mut conn);
    while let Some(row) = rows.try_next().await? {
        todos.push(row);
    }

    // Lines below show how to set status code and headers on the response
    // let resp = expect_context::<ResponseOptions>();
    // resp.set_status(StatusCode::IM_A_TEAPOT);
    // resp.insert_header(SET_COOKIE, HeaderValue::from_str("fizz=buzz").unwrap());

    Ok(todos)
}

#[server]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    use self::ssr::*;
    let mut conn = db().await?;

    // fake API delay
    std::thread::sleep(std::time::Duration::from_millis(250));

    match sqlx::query("INSERT INTO todos (title, completed) VALUES ($1, false)")
        .bind(title)
        .execute(&mut conn)
        .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

// https://docs.rs/leptos/latest/leptos/attr.server.html
#[server]
pub async fn delete_todo(id: u16) -> Result<(), ServerFnError> {
    use self::ssr::*;
    let mut conn = db().await?;

    Ok(sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&mut conn)
        .await
        .map(|_| ())?)
}

#[component]
pub fn TodoApp() -> impl IntoView {
    view! {
        <header>
            <h1>"My Tasks"</h1>
        </header>
        <main>
            <Todos/>
        </main>
    }
}

#[component]
pub fn Todos() -> impl IntoView {
    let add_todo = ServerMultiAction::<AddTodo>::new();
    let submissions = add_todo.submissions();
    // https://docs.rs/leptos/latest/leptos/prelude/struct.ServerAction.html
    // ServerAction: サーバファンクション呼び出し
    // DeleteTodo: server macroで定義されたdelete_todo()。name指定してないのでPascalCaseで変換される。
    let delete_todo = leptos::prelude::ServerAction::<DeleteTodo>::new();

    // データの非同期ロード
    // https://docs.rs/leptos/latest/leptos/prelude/struct.Resource.html
    // list of todos is loaded from the server in reaction to changes
    let todos = leptos::prelude::Resource::new(
        move || {
            (
                delete_todo.version().get(),
                add_todo.version().get(),
                delete_todo.version().get(),
            )
        },
        move |_| get_todos(),
    );

    let existing_todos = move || {
        tracing::debug!("call existing_todos");
        Suspend::new(async move {
            todos.await.map(|todos| {
                if todos.is_empty() {
                    Either::Left(view! { <p>"No tasks were found."</p> })
                } else {
                    println!("todos: {:?}", &todos);
                    Either::Right(
                        todos
                            .iter()
                            .map(move |todo| {
                                let id = todo.id;
                                view! {
                                    <li>
                                        {todo.title.clone()}
                                        <ActionForm action=delete_todo>
                                            <input type="hidden" name="id" value=id/>
                                            <input type="submit" value="X"/>
                                        </ActionForm>
                                    </li>
                                }
                            })
                            .collect::<Vec<_>>(),
                    )
                }
            })
        })
    };

    // use crate::error_template::ErrorTemplate;

    view! {
        <MultiActionForm action=add_todo>
            <label>"Add a Todo" <input type="text" name="title"/></label>
            <input type="submit" value="Add"/>
        </MultiActionForm>
        <div>
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                <ErrorBoundary fallback=|errors| view! { <ErrorTemplate2 errors/> }>
                    <ul>
                        {existing_todos}
                        {move || {
                            submissions
                                .get()
                                .into_iter()
                                .filter(|submission| submission.pending().get())
                                .map(|submission| {
                                    view! {
                                        <li class="pending">
                                            {move || submission.input().get().map(|data| data.title)}
                                        </li>
                                    }
                                })
                                .collect::<Vec<_>>()
                        }}
                    </ul>
                </ErrorBoundary>
            </Transition>
        </div>
    }
}

#[component]
pub fn ErrorTemplate2(
    #[prop(optional)] outside_errors: Option<Errors>,
    #[prop(optional, into)] errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
    let errors = match outside_errors {
        Some(e) => RwSignal::new(e),
        None => match errors {
            Some(e) => e,
            None => panic!("No Errors found and we expected errors!"),
        },
    };

    // Get Errors from Signal
    // Downcast lets us take a type that implements `std::error::Error`
    let errors = move || errors.get().into_iter().map(|(_, v)| v).collect::<Vec<_>>();

    // Only the response code for the first error is actually sent from the server
    // this may be customized by the specific application
    /*#[cfg(feature = "ssr")]
    {
        let response = use_context::<ResponseOptions>();
        if let Some(response) = response {
            response.set_status(errors[0].status_code());
        }
    }*/

    view! {
        <h1>"Errors"</h1>
        {move || {
            errors()
                .into_iter()
                .map(|error| {
                    view! { <p>"Error: " {error.to_string()}</p> }
                })
                .collect::<Vec<_>>()
        }}
    }
}
