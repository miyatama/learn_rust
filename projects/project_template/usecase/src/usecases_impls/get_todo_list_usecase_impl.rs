use crate::usecases::get_todo_list_usecase::GetTodoListUseCase;
use repository::{RepositoryHandler, TodoRepository};
use util::AppResult;
use util::Todo;

pub struct GetTodoListUseCaseImpl<'r, R: RepositoryHandler> {
    todo_repository: &'r R::Todo,
}

impl<'r, R: RepositoryHandler> GetTodoListUseCaseImpl<'r, R> {
    pub fn new(handler: &'r R) -> Self {
        Self {
            todo_repository: handler.todo_repository(),
        }
    }
}

impl<'r, R: RepositoryHandler> GetTodoListUseCase for GetTodoListUseCaseImpl<'r, R> {
    fn run(&self) -> AppResult<Vec<Todo>> {
        self.todo_repository.list()
    }
}
