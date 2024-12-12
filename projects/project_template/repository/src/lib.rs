mod repositories;
mod repositories_impls;

pub use repositories::todo_repository::TodoRepository;
pub use repositories_impls::todo_repository_impl::TodoRepositoryImpl;

pub trait Repositories {
    type TodoRepository: TodoRepository;
    fn todo_repository(&self) -> &Self::TodoRepository;
}

#[derive(Clone, Debug, Default)]
pub struct RepositoriesImpl {
    todo_repository: TodoRepositoryImpl,
}

impl Repositories for RepositoriesImpl {
    type TodoRepository = TodoRepositoryImpl;
    fn todo_repository(&self) -> &Self::TodoRepository {
        &self.todo_repository
    }
}
