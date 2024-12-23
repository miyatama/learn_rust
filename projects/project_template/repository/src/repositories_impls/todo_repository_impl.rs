use crate::repositories::todo_repository::TodoRepository;
use domain::{TodoApiClient, TodoApiClientImpl};
#[cfg(test)]
use mockall::automock;
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

#[cfg_attr(test, automock)]
impl TodoRepository for TodoRepositoryImpl {
    fn create(&self, text: String) -> AppResult<Todo> {
        self.todo_api_client.create(Todo { id: 0, text: text })
    }

    fn update(&self, todo: Todo) -> AppResult<Todo> {
        self.todo_api_client.update(todo)
    }

    fn list(&self) -> AppResult<Vec<Todo>> {
        self.todo_api_client.list()
    }

    fn delete(&self, id: u32) -> AppResult<()> {
        self.todo_api_client.delete(Todo {
            id: id,
            text: "".to_string(),
        })
    }
}
