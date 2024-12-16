use crate::usecases::update_todo_usecase::UpdateTodoUseCase;
use repository::TodoRepositoryImpl;
use std::sync::Arc;
use util::AppResult;
use util::Todo;

#[derive(Clone, Debug)]
pub struct UpdateTodoUseCaseImpl {
    todo_repository: Arc<TodoRepositoryImpl>,
}

impl UpdateTodoUseCaseImpl {
    pub fn new(todo_repository: TodoRepositoryImpl) -> Self {
        Self {
            todo_repository: Arc::new(todo_repository),
        }
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
