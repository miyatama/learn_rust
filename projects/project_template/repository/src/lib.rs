mod repositories;
mod repositories_impls;

use domain::DomainHandler;
pub use repositories::TodoRepository;
pub use repositories_impls::TodoRepositoryImpl;

use std::fmt::Debug;
use std::clone::Clone;

pub trait RepositoryHandler: Debug + Clone  {
    type Todo: TodoRepository;
    fn todo_repository(&self) -> &Self::Todo;
}

#[derive(Debug, Clone)]
pub struct RepositoryHandlerImpl<'d, D: DomainHandler> {
    todo_repository: TodoRepositoryImpl<'d, D>,
}

impl<'d, D: DomainHandler> RepositoryHandlerImpl<'d, D> {
    pub fn new(handler: &'d D) -> Self {
        let todo_repository = TodoRepositoryImpl::new(handler);
        Self {
            todo_repository: todo_repository,
        }
    }
}

impl<'d, D: DomainHandler> RepositoryHandler for RepositoryHandlerImpl<'d, D> {
    type Todo = TodoRepositoryImpl<'d, D>;
    fn todo_repository(&self) -> &Self::Todo {
        &self.todo_repository
    }
}
