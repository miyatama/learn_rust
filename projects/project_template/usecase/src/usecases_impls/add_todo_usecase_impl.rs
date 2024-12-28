use crate::usecases::add_todo_usecase::AddTodoUseCase;

use repository::{RepositoryHandler, TodoRepository};
use util::AppResult;
use util::Todo;

#[derive(Clone, Debug)]
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
        let mock_todo_repository = TodoRepositoryImpl::new();
        mock_todo_repository
            .expect_create()
            .with(predicate::eq(text))
            .times(1)
            .return_const(Ok(expect.clone()));
        let usecase = AddTodoUseCaseImpl::new(mock_todo_repository);
        let result = usecase.run(text);
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        assert_eq!(expect.id, result.id);
        assert_eq!(expect.text, result.text);
    }
}
