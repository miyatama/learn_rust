use crate::usecases::get_todo_list_usecase::GetTodoListUseCase;
use repository::{TodoRepository, TodoRepositoryImpl};
use util::AppResult;
use util::Todo;

#[derive(Clone)]
pub struct GetTodoListUseCaseImpl<'r> {
    todo_repository: &'r dyn TodoRepository,
}

impl<'r> GetTodoListUseCaseImpl<'r> {
    pub fn new(repository: &'r dyn TodoRepository) -> Self {
        Self {
            todo_repository: repository,
        }
    }
}

impl<'r> GetTodoListUseCase for GetTodoListUseCaseImpl<'r> {
    fn run(&self) -> AppResult<Vec<Todo>> {
        self.todo_repository.list()
    }
}
