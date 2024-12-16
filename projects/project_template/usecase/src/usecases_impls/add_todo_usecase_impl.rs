use crate::usecases::add_todo_usecase::AddTodoUseCase;
use repository::TodoRepositoryImpl;
use std::sync::Arc;
use util::AppResult;
use util::Todo;

#[derive(Clone, Debug)]
pub struct AddTodoUseCaseImpl {
    todo_repository: Arc<TodoRepositoryImpl>,
}

impl AddTodoUseCaseImpl {
    pub fn new(todo_repository: TodoRepositoryImpl) -> Self {
        Self {
            todo_repository: Arc::new(todo_repository),
        }
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
