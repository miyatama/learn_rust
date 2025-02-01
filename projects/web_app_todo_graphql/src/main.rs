mod models;
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription};
use async_graphql_axum::GraphQL;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use models::roots::{QueryRoot, Storage};
use models::todo_service_data::TodoServiceData;

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    let schema = async_graphql::Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(Storage::default())
        .finish();
    let app = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
