use crate::usecases::update_todo_usecase::UpdateTodoUseCase;
use repository::{RepositoryHandler, TodoRepository};
use util::AppResult;
use util::Todo;

#[derive(Clone, Debug)]
pub struct UpdateTodoUseCaseImpl<'r, R: RepositoryHandler> {
    todo_repository: &'r R::Todo,
}

impl<'r, R: RepositoryHandler> UpdateTodoUseCaseImpl<'r, R> {
    pub fn new(handler: &'r R) -> Self {
        Self {
            todo_repository: handler.todo_repository(),
        }
    }
}

impl<'r, R: RepositoryHandler> UpdateTodoUseCase for UpdateTodoUseCaseImpl<'r, R> {
    fn run(&self, id: u32, text: String) -> AppResult<Todo> {
        self.todo_repository.update(Todo { id: id, text: text })
    }
}
