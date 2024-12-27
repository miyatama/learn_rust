mod usecase_impls;

use crate::layers_trait_mock::{RepositoryHandler, RepositoryHandlerImpl, UsecaseHandler};
pub use usecase_impls::get_todo2_usecase_impl::GetTodo2UsecaseImpl;

pub struct UsecaseHandlerImpl {
    get_todo2: GetTodo2UsecaseImpl,
}

impl UsecaseHandlerImpl {
    pub fn new() -> Self {
        let hadler = RepositoryHandlerImpl::new();
        let todo2_repository = hadler.get_todo2_repository();
        let get_todo2 = GetTodo2UsecaseImpl::new(todo2_repository.clone());
        Self {
            get_todo2: get_todo2,
        }
    }
}

impl UsecaseHandler for UsecaseHandlerImpl {
    type GetTodo2UsecaseType = GetTodo2UsecaseImpl;
    fn get_todo2_usecase(&self) -> &Self::GetTodo2UsecaseType {
        &self.get_todo2
    }
}
