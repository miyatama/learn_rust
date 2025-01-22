use async_graphql::{
    http::GraphiQLSource, EmptyMutation, EmptySubscription, Object, Request, Schema,
};
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use std::error::Error;

mod models;
pub type Todo = Schema<models::QueryRoot, EmptyMutation, EmptySubscription>;

async fn graphql_handler(schema: Extension<Todo>, req: Json<Request>) -> Json<Response> {
    schema.execute(req.0).await.into()
}

async fn graphql() -> impl IntoResponse {
    Html(GraphiQLSource::build().finish())
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();
    let app = Router::new().route("/", get(graphql).post(graphql_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
