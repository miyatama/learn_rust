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
    type Repositories: Repositories;
    fn get_todo_list(&self) -> &GetTodoListUseCaseImpl;
    fn add_todo(&self) -> &AddTodoUseCaseImpl;
    fn update_todo(&self) -> &UpdateTodoUseCaseImpl;
}

#[derive(Clone, Debug)]
pub struct UseCasesImpls {
    get_todo_list_usecase: GetTodoListUseCaseImpl,
    add_todo_usecase: AddTodoUseCaseImpl,
    update_todo_usecase: UpdateTodoUseCaseImpl,
}

impl UseCasesImpls {
    pub async fn new() -> UseCasesImpls {
        let repositories = RepositoriesImpl::new();
        let todo_repository = repositories.todo_repository();
        let get_todo_list_usecase = GetTodoListUseCaseImpl::new(todo_repository.clone());
        let add_todo_usecase = AddTodoUseCaseImpl::new(todo_repository.clone());
        let update_todo_usecase = UpdateTodoUseCaseImpl::new(todo_repository.clone());
        Self {
            get_todo_list_usecase: get_todo_list_usecase,
            add_todo_usecase: add_todo_usecase,
            update_todo_usecase: update_todo_usecase,
        }
    }
}

impl UseCases for UseCasesImpls {
    type Repositories = RepositoriesImpl;

    fn get_todo_list(&self) -> &GetTodoListUseCaseImpl {
        &self.get_todo_list_usecase
    }
    fn add_todo(&self) -> &AddTodoUseCaseImpl {
        &self.add_todo_usecase
    }
    fn update_todo(&self) -> &UpdateTodoUseCaseImpl {
        &self.update_todo_usecase
    }
}
