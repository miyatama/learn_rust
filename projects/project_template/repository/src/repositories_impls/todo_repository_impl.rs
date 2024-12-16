use crate::repositories::todo_repository::TodoRepository;
use domain::{TodoApiClient, TodoApiClientImpl};
use std::sync::Arc;
use util::AppResult;
use util::Todo;

#[derive(Debug, Clone)]
pub struct TodoRepositoryImpl {
    todo_api_client: Arc<TodoApiClientImpl>,
}

impl TodoRepositoryImpl {
    pub fn new(todo_api_client: TodoApiClientImpl) -> Self {
        Self {
            todo_api_client: Arc::new(todo_api_client),
        }
    }
}

impl TodoRepository for TodoRepositoryImpl {
    async fn create(&self) -> AppResult<Todo> {
        // TODO call domain
        Ok(Todo {
            id: 0,
            text: "".to_string(),
        })
    }
    async fn update(&self, _todo: Todo) -> AppResult<Todo> {
        // TODO call domain
        Ok(Todo {
            id: 0,
            text: "".to_string(),
        })
    }
    async fn list(&self) -> AppResult<Vec<Todo>> {
        self.todo_api_client.list().await
    }
}
