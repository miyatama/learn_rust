use crate::usecases::update_todo_usecase::UpdateTodoUseCase;
use repository::{TodoRepository, TodoRepositoryImpl};
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
    fn run(&self, id: u32, text: String) -> AppResult<Todo> {
        self.todo_repository.update(Todo { id: id, text: text })
    }
}
