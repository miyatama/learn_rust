use crate::domains::todo_api_client::TodoApiClient;
use reqwest;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use util::AppResult;
use util::Todo;
use log::debug;

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

impl TodoApiClient for TodoApiClientImpl {
    async fn list(&self) -> AppResult<Vec<Todo>> {
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
    async fn create(&self, _todo: Todo) -> AppResult<Todo> {
        // TODO ceate api call
        Err("todo call api")?
    }
    async fn update(&self, _todo: Todo) -> AppResult<Todo> {
        // TODO ceate api call
        Err("todo call api")?
    }
}
