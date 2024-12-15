use crate::usecases::update_todo_usecase::UpdateTodoUseCase;
use util::AppResult;
use util::Todo;

#[derive(Clone, Debug)]
pub struct UpdateTodoUseCaseImpl {}

impl UpdateTodoUseCaseImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl UpdateTodoUseCase for UpdateTodoUseCaseImpl {
    fn run(&self) -> AppResult<Todo> {
        // TODO call repository
        Ok(Todo {
            id: 0,
            text: "".to_string(),
        })
    }
}
