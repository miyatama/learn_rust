mod usecases;
mod usecases_impls;

use repository::{RepositoryHandler, RepositoryHandlerImpl };
pub use usecases::add_todo_usecase::AddTodoUseCase;
pub use usecases::delete_todo_usecase::DeleteTodoUseCase;
pub use usecases::get_todo_list_usecase::GetTodoListUseCase;
pub use usecases::update_todo_usecase::UpdateTodoUseCase;
pub use usecases_impls::add_todo_usecase_impl::AddTodoUseCaseImpl;
pub use usecases_impls::delete_todo_usecase_impl::DeleteTodoUseCaseImpl;
pub use usecases_impls::get_todo_list_usecase_impl::GetTodoListUseCaseImpl;
pub use usecases_impls::update_todo_usecase_impl::UpdateTodoUseCaseImpl;

pub trait UseCases {
    type Handler: RepositoryHandler;
    fn get_todo_list(&self) -> &GetTodoListUseCaseImpl;
    fn add_todo(&self) -> &AddTodoUseCaseImpl;
    fn update_todo(&self) -> &UpdateTodoUseCaseImpl;
    fn delete_todo(&self) -> &DeleteTodoUseCaseImpl;
}

#[derive(Clone, Debug)]
pub struct UseCasesImpls {
    get_todo_list_usecase: GetTodoListUseCaseImpl,
    add_todo_usecase: AddTodoUseCaseImpl,
    update_todo_usecase: UpdateTodoUseCaseImpl,
    delete_todo_usecase: DeleteTodoUseCaseImpl,
}

impl UseCasesImpls {
    pub async fn new() -> UseCasesImpls {
        let repositories = RepositoryHandlerImpl::new();
        let todo_repository = repositories.todo_repository();
        let get_todo_list_usecase = GetTodoListUseCaseImpl::new(todo_repository.clone());
        let add_todo_usecase = AddTodoUseCaseImpl::new(todo_repository.clone());
        let update_todo_usecase = UpdateTodoUseCaseImpl::new(todo_repository.clone());
        let delete_todo_usecase = DeleteTodoUseCaseImpl::new(todo_repository.clone());
        Self {
            get_todo_list_usecase: get_todo_list_usecase,
            add_todo_usecase: add_todo_usecase,
            update_todo_usecase: update_todo_usecase,
            delete_todo_usecase: delete_todo_usecase,
        }
    }
}

impl UseCases for UseCasesImpls {
    type Handler = RepositoryHandlerImpl;

    fn get_todo_list(&self) -> &GetTodoListUseCaseImpl {
        &self.get_todo_list_usecase
    }
    fn add_todo(&self) -> &AddTodoUseCaseImpl {
        &self.add_todo_usecase
    }
    fn update_todo(&self) -> &UpdateTodoUseCaseImpl {
        &self.update_todo_usecase
    }
    fn delete_todo(&self) -> &DeleteTodoUseCaseImpl {
        &self.delete_todo_usecase
    }
}
