use axum::{routing::get, Router};
use event_sourcing::{
    route_handler::{command_handler, query_handler},
    state::new_application_state,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let state = new_application_state().await;
    let router = Router::new()
        .route(
            "/account/:account_id",
            get(query_handler).post(command_handler),
        )
        .with_state(state);
    let addr = SocketAddr::from(([0, 0, 0, 0], 3030));
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), router)
        .await
        .unwrap();
}
