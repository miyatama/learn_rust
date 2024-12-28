use crate::usecases::delete_todo_usecase::DeleteTodoUseCase;
use repository::{RepositoryHandler, TodoRepository};
use util::AppResult;

#[derive(Clone, Debug)]
pub struct DeleteTodoUseCaseImpl<'r, R: RepositoryHandler> {
    todo_repository: &'r R::Todo,
}

impl<'r, R: RepositoryHandler> DeleteTodoUseCaseImpl<'r, R> {
    pub fn new(handler: &'r R) -> Self {
        Self {
            todo_repository: handler.todo_repository(),
        }
    }
}

impl<'r, R: RepositoryHandler> DeleteTodoUseCase for DeleteTodoUseCaseImpl<'r, R> {
    fn run(&self, id: u32) -> AppResult<()> {
        self.todo_repository.delete(id)
    }
}
