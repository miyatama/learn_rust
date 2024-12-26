mod todo2_repository;
mod todo2_repository_impl;

pub use todo2_repository::Todo2Repository;
pub use todo2_repository_impl::Todo2RepositoryImpl;

use crate::layers_trait_mock::{DomainHandler, DomainHandlerImpl};

pub trait RepositoryHandler {
    type Todo2RepositoryType: Todo2Repository;
    // Copyトレイトが必要となる為、参照で保持する。
    fn get_todo2_repository(&self) -> &Self::Todo2RepositoryType;
}

pub struct RepositoryHandlerImpl {
    todo2_repository: Todo2RepositoryImpl,
}

impl RepositoryHandlerImpl {
    pub fn new() -> Self {
        let handler = DomainHandlerImpl::new();
        let todo2_client = handler.get_todo2_client();
        let todo2_repository = Todo2RepositoryImpl::new(todo2_client.clone());
        Self {
            todo2_repository: todo2_repository,
        }
    }
}

impl RepositoryHandler for RepositoryHandlerImpl {
    type Todo2RepositoryType = Todo2RepositoryImpl;
    fn get_todo2_repository(&self) -> &Self::Todo2RepositoryType {
        &self.todo2_repository
    }
}
