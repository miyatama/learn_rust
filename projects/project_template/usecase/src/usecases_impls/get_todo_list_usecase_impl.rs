use crate::usecases::get_todo_list_usecase::GetTodoListUseCase;
use repository::{TodoRepository, TodoRepositoryImpl};
use util::AppResult;
use util::Todo;

#[derive(Clone, Debug)]
pub struct GetTodoListUseCaseImpl {
}

impl GetTodoListUseCaseImpl {
    pub fn new() -> Self {
        Self { }
    }
}

impl GetTodoListUseCase for GetTodoListUseCaseImpl {
    fn run(&self) -> AppResult<Vec<Todo>> {
        // TODO call repository
        Ok(vec![Todo {
            id: 0,
            text: "".to_string(),
        }])
    }
}
