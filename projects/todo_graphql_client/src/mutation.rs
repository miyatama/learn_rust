#![allow(clippy::all, warnings)]
pub struct TodoUpdateRepositories;
pub mod todo_update_repositories {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "TodoUpdateRepositories";
    pub const QUERY: &str =
        "mutation TodoUpdateRepositories($text: String) {\r\n  addTodo(text: $text) \r\n}";
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
impl graphql_client::GraphQLQuery for TodoUpdateRepositories {
    type Variables = todo_update_repositories::Variables;
    type ResponseData = todo_update_repositories::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: todo_update_repositories::QUERY,
            operation_name: todo_update_repositories::OPERATION_NAME,
        }
    }
}
