use crate::layers_trait_mock::{GetTodo2Usecase, Todo2, Todo2Repository, Todo2RepositoryImpl};
use std::sync::Arc;

pub struct GetTodo2UsecaseImpl {
    repository: Arc<Todo2RepositoryImpl>,
}

impl GetTodo2UsecaseImpl {
    pub fn new(repository: Todo2RepositoryImpl) -> Self {
        Self {
            repository: Arc::new(repository),
        }
    }
}

impl GetTodo2Usecase for GetTodo2UsecaseImpl {
    fn run(&self) -> Vec<Todo2> {
        self.repository.get_todos()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_todos() {
        /*
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
         */
    }
}
