use crate::repositories::TodoRepository;
use domain::{DomainHandler, TodoApiClient};
use util::AppResult;
use util::Todo;

#[derive(Debug, Clone)]
pub struct TodoRepositoryImpl<'r, R: DomainHandler> {
    todo_api_client: &'r R::TodoApi,
}

impl<'r, R: DomainHandler> TodoRepositoryImpl<'r, R> {
    pub fn new(handler: &'r R) -> Self {
        Self {
            todo_api_client: handler.todo_api_client(),
        }
    }
}

impl<'r, R: DomainHandler> TodoRepository for TodoRepositoryImpl<'r, R> {
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
