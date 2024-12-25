mod get_todo_usecase;
mod get_todo_usecase_impl;

pub use get_todo_usecase::GetTodoUsecase;
pub use get_todo_usecase_impl::GetTodoUsecaseImpl;

use crate::layers::{RepositoryHandler, RepositoryHandlerImpl};

pub trait UsecaseHandler {
    type GetTodoUsecaseType: GetTodoUsecase;
    fn get_todo_usecase(&self) -> &Self::GetTodoUsecaseType;
}

pub struct UsecaseHandlerImpl {
    get_todo: GetTodoUsecaseImpl,
}

impl UsecaseHandlerImpl {
    pub fn new() -> Self {
        let hadler = RepositoryHandlerImpl::new();
        let todo_repository = hadler.get_todo_repository();
        let get_todo = GetTodoUsecaseImpl::new(todo_repository.clone());
        Self { get_todo: get_todo }
    }
}

impl UsecaseHandler for UsecaseHandlerImpl {
    type GetTodoUsecaseType = GetTodoUsecaseImpl;
    fn get_todo_usecase(&self) -> &Self::GetTodoUsecaseType {
        &self.get_todo
    }
}
