use crate::layers::{GetTodoUsecase, Todo, TodoRepository, TodoRepositoryImpl};
use mockall::automock;
use std::sync::Arc;

pub struct GetTodoUsecaseImpl {
    repository: Arc<TodoRepositoryImpl>,
}

impl GetTodoUsecaseImpl {
    pub fn new(repository: TodoRepositoryImpl) -> Self {
        Self {
            repository: Arc::new(repository),
        }
    }
}

#[automock]
impl GetTodoUsecase for GetTodoUsecaseImpl {
    fn run(&self) -> Vec<Todo> {
        self.repository.get_todos()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_todos() {
        let text = "test_parameter".to_string();
        let expect = vec![Todo::new(text)];
        let mut mock_todo_repository = TodoRepositoryImpl::new();
        mock_todo_repository
            .expect_get_todos()
            .times(1)
            .return_const(expect.clone());
        let usecase = GetTodoUsecaseImpl::new(mock_todo_repository);
        let result = usecase.run();

        assert_eq!(expect[0], result[0]);
    }
}
