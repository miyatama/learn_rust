#![allow(clippy::all, warnings)]
pub struct TodoChanged;
pub mod todo_changed {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "TodoChanged";
    pub const QUERY : & str = "subscription TodoChanged ($mutationType: MutationType) {\r\n  todos(mutationType: $mutationType) {\r\n    mutationType,\r\n    id,\r\n    todo {\r\n      id,\r\n      text,\r\n    } \r\n  }\r\n}" ;
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
    #[derive()]
    pub enum MutationType {
        CREATED,
        DELETED,
        Other(String),
    }
    impl ::serde::Serialize for MutationType {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                MutationType::CREATED => "CREATED",
                MutationType::DELETED => "DELETED",
                MutationType::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MutationType {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "CREATED" => Ok(MutationType::CREATED),
                "DELETED" => Ok(MutationType::DELETED),
                _ => Ok(MutationType::Other(s)),
            }
        }
    }
    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "mutationType")]
        pub mutation_type: Option<MutationType>,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub todos: TodoChangedTodos,
    }
    #[derive(Deserialize)]
    pub struct TodoChangedTodos {
        #[serde(rename = "mutationType")]
        pub mutation_type: MutationType,
        pub id: Int,
        pub todo: Option<TodoChangedTodosTodo>,
    }
    #[derive(Deserialize)]
    pub struct TodoChangedTodosTodo {
        pub id: Int,
        pub text: String,
    }
}
impl graphql_client::GraphQLQuery for TodoChanged {
    type Variables = todo_changed::Variables;
    type ResponseData = todo_changed::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: todo_changed::QUERY,
            operation_name: todo_changed::OPERATION_NAME,
        }
    }
}
