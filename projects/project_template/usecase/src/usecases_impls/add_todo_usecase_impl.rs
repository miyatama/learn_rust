use crate::usecases::add_todo_usecase::AddTodoUseCase;
use repository::{TodoRepository, TodoRepositoryImpl};
use std::sync::Arc;
use util::AppResult;
use util::Todo;

#[derive(Clone, Debug)]
pub struct AddTodoUseCaseImpl {
    todo_repository: Arc<TodoRepositoryImpl>,
}

impl AddTodoUseCaseImpl {
    pub fn new(todo_repository: TodoRepositoryImpl) -> Self {
        Self {
            todo_repository: Arc::new(todo_repository),
        }
    }
}

impl AddTodoUseCase for AddTodoUseCaseImpl {
    fn run(&self, text: String) -> AppResult<Todo> {
        self.todo_repository.create(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate;
    use repository::MockTodoRepository;

    #[tokio::test]
    async fn add_todo_usecase_success() {
        let text = "test_parameter".to_string();
        let expect = Todo {
            id: 100,
            text: "test message".to_string(),
        };
        let mock_todo_repository = MockTodoRepository::new();
        mock_todo_repository
            .expect_create()
            .with(predicate::eq(text))
            .times(1)
            .return_const(Ok(expect.clone()));
        let usecase = AddTodoUseCaseImpl::new(mock_todo_repository);
        let result = usecase.run(text).await;
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        assert_eq!(expect.id, result.id);
        assert_eq!(expect.text, result.text);
    }
}
