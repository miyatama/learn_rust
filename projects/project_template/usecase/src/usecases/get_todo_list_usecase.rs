use util::AppResult;
use util::Todo;

#[cfg_attr(feature = "mock", mockall::automock)]
pub trait GetTodoListUseCase {
    async fn run(&self) -> AppResult<Vec<Todo>>;
}
