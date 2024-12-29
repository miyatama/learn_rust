use crate::repositories::TodoRepository;
use domain::{DomainHandler, TodoApiClient};
use util::AppResult;
use util::Todo;

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

#[cfg(test)]
mod tests {
    #[test]
    fn test_create() {
        let expect_todo = Todo {
            id: 100,
            text: "test2".to_string(),
        };
        let mock_todo_api_client = TodoApiClient::new();
        mock_todo_api_client
            .expect_create()
            .times(1)
            .return_const(expect_todo);
        let mock_domain_handler = DomainHandler::new();
        mock_domain_handler
            .expect_todo_api_client()
            .times(1)
            .return_const(mock_todo_api_client);
        let repository = TodoRepositoryImpl::new(mock_domain_handler);
        let result = repository.create("test".to_string());
        assert_eq!(expect, result);
    }
}
