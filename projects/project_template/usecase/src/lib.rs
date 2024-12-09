mod usecases;

pub use usecases::add_todo_usecase::AddTodoUseCase;
pub use usecases::get_todo_list_usecase::GetTodoListUseCase;
pub use usecases::update_todo_usecase::UpdateTodoUseCase;

pub trait UseCases {
    type GetTodoListUseCase: GetTodoListUseCase;
    type AddTodoUseCase: AddTodoUseCase;
    type UpdateTodoUseCase: UpdateTodoUseCase;
    fn get_todo_list_usecase(&self) -> &Self::GetTodoListUseCase;
    fn add_todo_usecase(&self) -> &Self::AddTodoUseCase;
    fn update_todo_usecase(&self) -> &Self::UpdateTodoUseCase;
}
