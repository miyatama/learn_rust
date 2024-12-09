use util::AppResult;
use util::Todo;

#[cfg_attr(feature = "mock", mockall::automock)]
pub trait GetTodoListUseCase {
    fn run(&self) -> AppResult<Vec<Todo>>;
}
