mod get_todo_usecase;
mod get_todo_usecase_impl;

pub use get_todo_usecase::GetTodoUsecase;
use mockall_double::double;

#[double]
pub use get_todo_usecase_impl::GetTodoUsecaseImpl;

use crate::layers::{RepositoryHandler, RepositoryHandlerImpl};

#[cfg_attr(test, allow(dead_code))]
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
        #[cfg_attr(test, allow(unused_variables))]
        let todo_repository = hadler.get_todo_repository();
        #[cfg(test)]
        let get_todo = GetTodoUsecaseImpl::new();

        #[cfg(not(test))]
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
