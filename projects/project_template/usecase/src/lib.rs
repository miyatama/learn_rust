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
    fn get_todo_list_usecase(&self) -> &Self::GetTodoListUseCase;
    fn add_todo_usecase(&self) -> &Self::AddTodoUseCase;
    fn update_todo_usecase(&self) -> &Self::UpdateTodoUseCase;
}

#[derive(Clone, Debug)]
pub struct UseCasesImpls<'r, R: Repositories> {
    repositories: &'r R,
    add_todo_usecase: AddTodoUseCaseImpl,
    update_todo_usecase: UpdateTodoUseCaseImpl,
    get_todo_list_usecase: GetTodoListUseCaseImpl,
}

impl<'r, R: Repositories> UseCasesImpls<'r, R> {
    pub fn new(repositories: &'r R) -> Self {
        Self {
            repositories: repositories,
            add_todo_usecase: AddTodoUseCaseImpl::new(),
            update_todo_usecase: UpdateTodoUseCaseImpl::new(),
            get_todo_list_usecase: GetTodoListUseCaseImpl::new(),
        }
    }
}

impl<'u, R: Repositories> UseCases for UseCasesImpls<'u, R> {
    type AddTodoUseCase = AddTodoUseCaseImpl;
    type UpdateTodoUseCase = UpdateTodoUseCaseImpl;
    type GetTodoListUseCase = GetTodoListUseCaseImpl;

    fn add_todo_usecase(&self) -> &Self::AddTodoUseCase {
        &self.add_todo_usecase
    }
    fn update_todo_usecase(&self) -> &Self::UpdateTodoUseCase {
        &self.update_todo_usecase
    }
    fn get_todo_list_usecase(&self) -> &Self::GetTodoListUseCase {
        &self.get_todo_list_usecase
    }
}
