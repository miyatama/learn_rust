use crate::layers::{GetTodoUsecase, Todo, TodoRepositoryImpl};
use std::sync::Arc;

pub struct GetTodoUsecaseImpl {
    repository: Arc<TodoRepositoryImpl>,
}

impl GetTodoUsecaseImpl {
    pub fn new(repository: TodoRepositoryImpl) -> Self {
        Self {
            repository: Arc::new(repository),
        }
    }
}

impl GetTodoUsecase for GetTodoUsecaseImpl {
    fn run(&self) -> Vec<Todo> {
        vec![]
    }
}
