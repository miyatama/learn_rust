#![allow(clippy::all, warnings)]
pub struct TodoAddRepositories;
pub mod todo_add_repositories {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "TodoAddRepositories";
    pub const QUERY: &str =
        "mutation TodoAddRepositories($text: String) {\r\n  addTodo(text: $text)\r\n}";
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables {
        pub text: Option<String>,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "addTodo")]
        pub add_todo: Int,
    }
}
impl graphql_client::GraphQLQuery for TodoAddRepositories {
    type Variables = todo_add_repositories::Variables;
    type ResponseData = todo_add_repositories::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: todo_add_repositories::QUERY,
            operation_name: todo_add_repositories::OPERATION_NAME,
        }
    }
}
