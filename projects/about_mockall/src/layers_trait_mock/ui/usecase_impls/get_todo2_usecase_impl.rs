use crate::layers_trait_mock::{GetTodo2Usecase, RepositoryHandler, Todo2, Todo2Repository};

pub struct GetTodo2UsecaseImpl<'r, R: RepositoryHandler> {
    repository: &'r R::Todo2Repository,
}

impl<'r, R: RepositoryHandler> GetTodo2UsecaseImpl<'r, R> {
    pub fn new(handler: &'r R) -> Self {
        Self {
            repository: handler.get_todo2_repository(),
        }
    }
}

impl<'r, R: RepositoryHandler> GetTodo2Usecase for GetTodo2UsecaseImpl<'r, R> {
    fn run(&self) -> Vec<Todo2> {
        self.repository.get_todos()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

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
