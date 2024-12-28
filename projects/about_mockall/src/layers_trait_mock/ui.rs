mod usecase_impls;

use crate::layers_trait_mock::{RepositoryHandler, UsecaseHandler};
pub use usecase_impls::get_todo2_usecase_impl::GetTodo2UsecaseImpl;

pub struct UsecaseHandlerImpl<'r, R: RepositoryHandler> {
    get_todo2: GetTodo2UsecaseImpl<'r, R>,
}

impl<'r, R: RepositoryHandler> UsecaseHandlerImpl<'r, R> {
    pub fn new(handler: &'r R) -> Self {
        // let handler = RepositoryHandlerImpl::new();
        let get_todo2 = GetTodo2UsecaseImpl::new(handler);
        Self {
            get_todo2: get_todo2,
        }
    }
}

impl<'r, R: RepositoryHandler> UsecaseHandler for UsecaseHandlerImpl<'r, R> {
    type GetTodo2UsecaseType = GetTodo2UsecaseImpl<'r, R>;
    fn get_todo2_usecase(&self) -> &Self::GetTodo2UsecaseType {
        &self.get_todo2
    }
}
