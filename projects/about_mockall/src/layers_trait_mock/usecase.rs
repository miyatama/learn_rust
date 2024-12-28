mod get_todo2_usecase;
mod repository_impls;

pub use get_todo2_usecase::GetTodo2Usecase;
pub use repository_impls::todo2_repository_impl::Todo2RepositoryImpl;

use crate::layers_trait_mock::{DomainHandler, DomainHandlerImpl, RepositoryHandler};

#[cfg_attr(test, allow(dead_code))]
pub trait UsecaseHandler {
    type GetTodo2UsecaseType: GetTodo2Usecase;
    fn get_todo2_usecase(&self) -> &Self::GetTodo2UsecaseType;
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
    type Todo2Repository = Todo2RepositoryImpl;
    fn get_todo2_repository(&self) -> &Self::Todo2Repository {
        &self.todo2_repository
    }
}
