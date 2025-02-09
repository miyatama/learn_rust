#![allow(clippy::all, warnings)]
pub struct TodoDeleteRepositories;
pub mod todo_delete_repositories {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "TodoDeleteRepositories";
    pub const QUERY: &str =
        "mutation TodoDeleteRepositories($id: Int) {\r\n  deleteTodo(id: $id)\r\n}";
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
        pub id: Option<Int>,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "deleteTodo")]
        pub delete_todo: Int,
    }
}
impl graphql_client::GraphQLQuery for TodoDeleteRepositories {
    type Variables = todo_delete_repositories::Variables;
    type ResponseData = todo_delete_repositories::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: todo_delete_repositories::QUERY,
            operation_name: todo_delete_repositories::OPERATION_NAME,
        }
    }
}
