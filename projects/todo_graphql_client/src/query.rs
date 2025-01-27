#![allow(clippy::all, warnings)]
pub struct TodoRepositories;
pub mod todo_repositories {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "TodoRepositories";
    pub const QUERY: &str =
        "query TodoRepositories {\r\n  todos {\r\n    id,\r\n    text\r\n  }\r\n}";
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
    pub struct Variables;
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub todos: Vec<TodoRepositoriesTodos>,
    }
    #[derive(Deserialize)]
    pub struct TodoRepositoriesTodos {
        pub id: Int,
        pub text: String,
    }
}
impl graphql_client::GraphQLQuery for TodoRepositories {
    type Variables = todo_repositories::Variables;
    type ResponseData = todo_repositories::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: todo_repositories::QUERY,
            operation_name: todo_repositories::OPERATION_NAME,
        }
    }
}
