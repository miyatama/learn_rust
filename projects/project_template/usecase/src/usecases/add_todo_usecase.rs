use util::AppResult;
use util::Todo;

#[cfg_attr(feature = "mock", mockall::automock)]
pub trait AddTodoUseCase {
    fn run(&self) -> AppResult<Todo>;
}
