use crate::usecases::add_todo_usecase::AddTodoUseCase;

use repository::{RepositoryHandler, TodoRepository};
use util::AppResult;
use util::Todo;

pub struct AddTodoUseCaseImpl<'r, R: RepositoryHandler> {
    todo_repository: &'r R::Todo,
}

impl<'r, R: RepositoryHandler> AddTodoUseCaseImpl<'r, R> {
    pub fn new(handler: &'r R) -> Self {
        Self {
            todo_repository: handler.todo_repository(),
        }
    }
}

impl<'r, R: RepositoryHandler> AddTodoUseCase for AddTodoUseCaseImpl<'r, R> {
    fn run(&self, text: String) -> AppResult<Todo> {
        self.todo_repository.create(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate;

    #[tokio::test]
    async fn add_todo_usecase_success() {
        let text = "test_parameter".to_string();
        let expect = Todo {
            id: 100,
            text: "test message".to_string(),
        };
        let mock_todo_repository = TodoRepository::new();
        mock_todo_repository
            .expect_create()
            .with(predicate::eq(text))
            .times(1)
            .return_const(Ok(expect.clone()));
        let mock_repository_handler = RepositoryHandler::new();
        mock_repository_handler
            .expect_todo_repository()
            .times(1)
            .return_const(mock_todo_repository);
        let usecase = AddTodoUseCaseImpl::new(mock_repository_handler);
        let result = usecase.run(text);
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        assert_eq!(expect.id, result.id);
        assert_eq!(expect.text, result.text);
    }
}
