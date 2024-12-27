mod domain_impls;
mod todo2_repository;

pub use domain_impls::todo2_client_impl::Todo2ClientImpl;
pub use todo2_repository::Todo2Repository;

use crate::layers_trait_mock::DomainHandler;

pub trait RepositoryHandler {
    type Todo2RepositoryType: Todo2Repository;
    // Copyトレイトが必要となる為、参照で保持する。
    fn get_todo2_repository(&self) -> &Self::Todo2RepositoryType;
}

pub struct DomainHandlerImpl {
    todo2_client: Todo2ClientImpl,
}

impl DomainHandlerImpl {
    pub fn new() -> Self {
        let todo2_client = Todo2ClientImpl::new();
        Self {
            todo2_client: todo2_client,
        }
    }
}

impl DomainHandler for DomainHandlerImpl {
    type Todo2ClientType = Todo2ClientImpl;
    fn get_todo2_client(&self) -> &Self::Todo2ClientType {
        &self.todo2_client
    }
}
