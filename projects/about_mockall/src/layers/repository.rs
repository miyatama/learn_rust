mod todo_repository;
mod todo_repository_impl;

use mockall_double::double;
pub use todo_repository::TodoRepository;

#[double]
pub use todo_repository_impl::TodoRepositoryImpl;

use crate::layers::{DomainHandler, DomainHandlerImpl};

pub trait RepositoryHandler {
    type TodoRepositoryType: TodoRepository;
    // Copyトレイトが必要となる為、参照で保持する。
    fn get_todo_repository(&self) -> &Self::TodoRepositoryType;
}

pub struct RepositoryHandlerImpl {
    todo_repository: TodoRepositoryImpl,
}

impl RepositoryHandlerImpl {
    pub fn new() -> Self {
        let handler = DomainHandlerImpl::new();
        let todo_client = handler.get_todo_client();

        #[cfg(test)]
        let todo_repository = TodoRepositoryImpl::new();

        #[cfg(not(test))]
        let todo_repository = TodoRepositoryImpl::new(todo_client.clone());
        Self {
            todo_repository: todo_repository,
        }
    }
}

impl RepositoryHandler for RepositoryHandlerImpl {
    type TodoRepositoryType = TodoRepositoryImpl;
    fn get_todo_repository(&self) -> &Self::TodoRepositoryType {
        &self.todo_repository
    }
}
