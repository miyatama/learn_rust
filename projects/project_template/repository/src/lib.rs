mod repositories;
mod repositories_impls;

use domain::{Domains, DomainsImpl};
pub use repositories::todo_repository::TodoRepository;
pub use repositories_impls::todo_repository_impl::TodoRepositoryImpl;

pub trait Repositories {
    type TodoRepository: TodoRepository;
    fn todo_repository(&self) -> &Self::TodoRepository;
}

#[derive(Clone, Debug)]
pub struct RepositoriesImpl {
    todo_repository: TodoRepositoryImpl,
}

impl RepositoriesImpl {
    pub fn new() -> Self {
        let domains = DomainsImpl::new();
        let todo_api_client = domains.todo_api_client();
        let todo_repository = TodoRepositoryImpl::new(todo_api_client.clone());
        Self {
            todo_repository: todo_repository,
        }
    }
}

impl Repositories for RepositoriesImpl {
    type TodoRepository = TodoRepositoryImpl;
    fn todo_repository(&self) -> &Self::TodoRepository {
        &self.todo_repository
    }
}
