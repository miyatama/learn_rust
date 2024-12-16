use crate::usecases::get_todo_list_usecase::GetTodoListUseCase;
use repository::{TodoRepository, TodoRepositoryImpl};
use std::sync::Arc;
use util::AppResult;
use util::Todo;

#[derive(Clone, Debug)]
pub struct GetTodoListUseCaseImpl {
    todo_repository: Arc<TodoRepositoryImpl>,
}

impl GetTodoListUseCaseImpl {
    pub fn new(todo_repository: TodoRepositoryImpl) -> Self {
        Self {
            todo_repository: Arc::new(todo_repository),
        }
    }
}

impl GetTodoListUseCase for GetTodoListUseCaseImpl {
    async fn run(&self) -> AppResult<Vec<Todo>> {
        self.todo_repository.list().await
    }
}
