use crate::usecases::delete_todo_usecase::DeleteTodoUseCase;
use repository::{TodoRepository, TodoRepositoryImpl};
use std::future::Future;
use std::sync::Arc;
use util::AppResult;

#[derive(Clone, Debug)]
pub struct DeleteTodoUseCaseImpl {
    todo_repository: Arc<TodoRepositoryImpl>,
}

impl DeleteTodoUseCaseImpl {
    pub fn new(todo_repository: TodoRepositoryImpl) -> Self {
        Self {
            todo_repository: Arc::new(todo_repository),
        }
    }
}

impl DeleteTodoUseCase for DeleteTodoUseCaseImpl {
    fn run(&self, id: u32) -> impl Future<Output = AppResult<()>> + Send {
        self.todo_repository.delete(id)
    }
}
