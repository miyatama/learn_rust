#[cfg(feature = "ssr")]
use axum::{
    body::Body,
    extract::Path,
    http::Request,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

#[cfg(feature = "ssr")]
async fn custom_handler(Path(id): Path<String>, req: Request<Body>) -> Response {
    let handler = leptos_axum::render_app_to_stream_with_context(
        move || {
            provide_context(id.clone());
        },
        todo::TodoApp,
    );
    handler(req).await.into_response();
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use crate::todo::{ssr::db, *};
    use leptos_axum::{generate_route_list, LeptosRoutes};

    simple_logger::init_with_level(log::level::Error).expect("could not initializing log");
    let mut conn = db().await.expect("could not initializing db");
    if let Err(e) = sqlx::migrate!().run(&mut conn).await {
        eprintln!("{e:?}");
    }

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(TodoApp);

    let app = Router::new()
        .route("/special/{id}", get(custom_handler))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    use leptos::mount::mount_to_body;
    _ = console_error_panick_hook::set_once();
    mount_to_body(todo::TodoApp);
}
