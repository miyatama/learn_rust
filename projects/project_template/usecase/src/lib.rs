mod usecases;
mod usecases_impls;

use repository::RepositoryHandler;
pub use usecases::add_todo_usecase::AddTodoUseCase;
pub use usecases::delete_todo_usecase::DeleteTodoUseCase;
pub use usecases::get_todo_list_usecase::GetTodoListUseCase;
pub use usecases::update_todo_usecase::UpdateTodoUseCase;
pub use usecases_impls::add_todo_usecase_impl::AddTodoUseCaseImpl;
pub use usecases_impls::delete_todo_usecase_impl::DeleteTodoUseCaseImpl;
pub use usecases_impls::get_todo_list_usecase_impl::GetTodoListUseCaseImpl;
pub use usecases_impls::update_todo_usecase_impl::UpdateTodoUseCaseImpl;

pub trait UseCases {
    type GetTodoList: GetTodoListUseCase;
    type AddTodo: AddTodoUseCase;
    type UpdateTodo: UpdateTodoUseCase;
    type DeleteTodo: DeleteTodoUseCase;
    fn get_todo_list(&self) -> &Self::GetTodoList;
    fn add_todo(&self) -> &Self::AddTodo;
    fn update_todo(&self) -> &Self::UpdateTodo;
    fn delete_todo(&self) -> &Self::DeleteTodo;
}

#[derive(Clone, Debug)]
pub struct UseCasesImpls<'r, R: RepositoryHandler> {
    get_todo_list_usecase: GetTodoListUseCaseImpl<'r, R>,
    add_todo_usecase: AddTodoUseCaseImpl<'r, R>,
    update_todo_usecase: UpdateTodoUseCaseImpl<'r, R>,
    delete_todo_usecase: DeleteTodoUseCaseImpl<'r, R>,
}

impl<'r, R: RepositoryHandler> UseCasesImpls<'r, R> {
    pub async fn new(handler: &'r R) -> Self {
        let get_todo_list_usecase = GetTodoListUseCaseImpl::new(handler);
        let add_todo_usecase = AddTodoUseCaseImpl::new(handler);
        let update_todo_usecase = UpdateTodoUseCaseImpl::new(handler);
        let delete_todo_usecase = DeleteTodoUseCaseImpl::new(handler);
        Self {
            get_todo_list_usecase: get_todo_list_usecase,
            add_todo_usecase: add_todo_usecase,
            update_todo_usecase: update_todo_usecase,
            delete_todo_usecase: delete_todo_usecase,
        }
    }
}

impl<'r, R: RepositoryHandler> UseCases for UseCasesImpls<'r, R> {
    type GetTodoList = GetTodoListUseCaseImpl<'r, R>;
    type AddTodo = AddTodoUseCaseImpl<'r, R>;
    type UpdateTodo = UpdateTodoUseCaseImpl<'r, R>;
    type DeleteTodo = DeleteTodoUseCaseImpl<'r, R>;

    fn get_todo_list(&self) -> &Self::GetTodoList {
        &self.get_todo_list_usecase
    }
    fn add_todo(&self) -> &Self::AddTodo {
        &self.add_todo_usecase
    }
    fn update_todo(&self) -> &Self::UpdateTodo {
        &self.update_todo_usecase
    }
    fn delete_todo(&self) -> &Self::DeleteTodo {
        &self.delete_todo_usecase
    }
}
