use crate::repositories::TodoRepository;
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

#[cfg_attr(feature = "mock", mockall::automock)]
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
