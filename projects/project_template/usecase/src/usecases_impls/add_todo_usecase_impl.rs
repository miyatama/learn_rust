use crate::usecases::add_todo_usecase::AddTodoUseCase;
use util::AppResult;
use util::Todo;

#[derive(Clone, Debug)]
pub struct AddTodoUseCaseImpl {}

impl Default for AddTodoUseCaseImpl {
    fn default() -> Self {
        Self {}
    }
}

impl AddTodoUseCase for AddTodoUseCaseImpl {
    fn run(&self) -> AppResult<Todo> {
        // TODO call repository
        Ok(Todo {
            id: 0,
            text: "".to_string(),
        })
    }
}
