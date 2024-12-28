mod repositories;
mod repositories_impls;

use domain::{DomainHandler, DomainHandlerImpl};
pub use repositories::TodoRepository;

#[cfg(feature = "mock")]
use mockall_double::double;

#[cfg_attr(feature = "mock", double)]
pub use repositories_impls::TodoRepositoryImpl;

pub trait RepositoryHandler {
    type TodoRepository: TodoRepository;
    fn todo_repository(&self) -> &Self::TodoRepository;
}

pub struct RepositoryHandlerImpl {
    todo_repository: TodoRepositoryImpl,
}

impl RepositoryHandlerImpl {
    pub fn new() -> Self {
        let domains = DomainHandlerImpl::new();
        let todo_api_client = domains.todo_api_client();

        #[cfg(test)]
        let todo_repository = TodoRepositoryImpl::new();
        #[cfg(not(test))]
        let todo_repository = TodoRepositoryImpl::new(todo_api_client.clone());

        Self {
            todo_repository: todo_repository,
        }
    }
}

impl RepositoryHandler for RepositoryHandlerImpl {
    type TodoRepository = TodoRepositoryImpl;
    fn todo_repository(&self) -> &Self::TodoRepository {
        &self.todo_repository
    }
}
