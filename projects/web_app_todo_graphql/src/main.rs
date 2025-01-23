use async_graphql::{
    http::GraphiQLSource, EmptyMutation, EmptySubscription, 
};
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};

pub type Todo = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;

async fn graphql_handler(schema: Extension<Todo>, req: Json<async_graphql::Request>) -> Json<async_graphql::Response> {
    schema.execute(req.0).await.into()
}

async fn graphql() -> impl IntoResponse {
    Html(GraphiQLSource::build().finish())
}

#[tokio::main]
async fn main() {
    let schema = async_graphql::Schema::build(Query, EmptyMutation, EmptySubscription).finish();
    let app = Router::new().route("/", get(graphql).post(graphql_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


struct Query;
#[async_graphql::Object]
impl Query {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}