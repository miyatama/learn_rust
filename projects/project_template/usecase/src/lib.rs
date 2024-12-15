mod usecases;
mod usecases_impls;

use repository::{Repositories, RepositoriesImpl};
pub use usecases::add_todo_usecase::AddTodoUseCase;
pub use usecases::get_todo_list_usecase::GetTodoListUseCase;
pub use usecases::update_todo_usecase::UpdateTodoUseCase;
pub use usecases_impls::add_todo_usecase_impl::AddTodoUseCaseImpl;
pub use usecases_impls::get_todo_list_usecase_impl::GetTodoListUseCaseImpl;
pub use usecases_impls::update_todo_usecase_impl::UpdateTodoUseCaseImpl;

pub trait UseCases {
    type GetTodoListUseCase: GetTodoListUseCase;
    type AddTodoUseCase: AddTodoUseCase;
    type UpdateTodoUseCase: UpdateTodoUseCase;
    fn get_todo_list_usecase(&self) -> Self::GetTodoListUseCase;
    fn add_todo_usecase(&self) -> Self::AddTodoUseCase;
    fn update_todo_usecase(&self) -> Self::UpdateTodoUseCase;
}

#[derive(Clone, Debug)]
pub struct UseCasesImpls<'r, R: Repositories> {
    repositories: &'r R,
}

impl<'r, R: Repositories> UseCasesImpls<'r, R> {
    pub fn new(repositories: &'r R) -> Self {
        Self {
            repositories: repositories,
        }
    }
}

impl<'u, R: Repositories> UseCases for UseCasesImpls<'u, R> {
    type AddTodoUseCase = AddTodoUseCaseImpl;
    type UpdateTodoUseCase = UpdateTodoUseCaseImpl;
    type GetTodoListUseCase = GetTodoListUseCaseImpl<'u>;

    fn add_todo_usecase(&self) -> Self::AddTodoUseCase {
        AddTodoUseCaseImpl::new()
    }
    fn update_todo_usecase(&self) -> Self::UpdateTodoUseCase {
        UpdateTodoUseCaseImpl::new()
    }
    fn get_todo_list_usecase(&self) -> Self::GetTodoListUseCase {
        GetTodoListUseCaseImpl::new(self.repositories.todo_repository())
    }
}
