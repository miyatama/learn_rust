use crate::domains::todo_api_client::TodoApiClient;
use log::debug;
use reqwest;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use std::future::Future;
use util::AppResult;
use util::Todo;

#[derive(Debug, Clone)]
pub struct TodoApiClientImpl {}

impl TodoApiClientImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Serialize, Deserialize)]
struct GetTodoParam {
    text: String,
}

#[derive(Serialize, Deserialize)]
struct GetTodoResponse {
    todos: Vec<Todo>,
}

#[derive(Serialize, Deserialize)]
struct AddTodoParam {
    text: String,
}

#[derive(Serialize, Deserialize)]
struct UpdateTodoParam {
    id: u32,
    text: String,
}

#[derive(Serialize, Deserialize)]
struct DeleteTodoParam {
    id: u32,
}

impl TodoApiClient for TodoApiClientImpl {
    fn list(&self) -> impl Future<Output = AppResult<Vec<Todo>>> + Sync {
        async {
            let param = GetTodoParam {
                text: "".to_string(),
            };
            let url = "http://localhost:8080/todos";
            let param_string = serde_json::to_string(&param).unwrap();
            let client = reqwest::Client::new();
            let res = client
                .get(url)
                .header(CONTENT_TYPE, "application/json")
                .body(param_string)
                .send()
                .await?;
            let content = res.text().await?;
            debug!("get todo response: {}", &content);
            let response: GetTodoResponse = serde_json::from_str(&content).unwrap();
            Ok(response.todos)
        }
    }

    fn create(&self, todo: Todo) -> impl Future<Output = AppResult<Todo>> + Sync {
        async move {
            let param = AddTodoParam { text: todo.text };
            let url = "http://localhost:8080/todo";
            let param_string = serde_json::to_string(&param).unwrap();
            let client = reqwest::Client::new();
            let res = client
                .post(url)
                .header(CONTENT_TYPE, "application/json")
                .body(param_string)
                .send()
                .await?;
            let content = res.text().await?;
            debug!("create todo response: {}", &content);
            let response: Todo = serde_json::from_str(&content).unwrap();
            Ok(response)
        }
    }

    fn update(&self, todo: Todo) -> impl Future<Output = AppResult<Todo>> + Sync {
        async move {
            let param = UpdateTodoParam {
                id: todo.id,
                text: todo.text,
            };
            let url = "http://localhost:8080/todo";
            let param_string = serde_json::to_string(&param).unwrap();
            let client = reqwest::Client::new();
            let res = client
                .put(url)
                .header(CONTENT_TYPE, "application/json")
                .body(param_string)
                .send()
                .await?;
            let content = res.text().await?;
            debug!("update todo response: {}", &content);
            let response: Todo = serde_json::from_str(&content).unwrap();
            Ok(response)
        }
    }

    fn delete(&self, todo: Todo) -> impl Future<Output = AppResult<()>> + Sync {
        async move {
            let param = DeleteTodoParam { id: todo.id };
            let url = "http://localhost:8080/todo";
            let param_string = serde_json::to_string(&param).unwrap();
            let client = reqwest::Client::new();
            let _res = client
                .delete(url)
                .header(CONTENT_TYPE, "application/json")
                .body(param_string)
                .send()
                .await?;
            Ok(())
        }
    }
}