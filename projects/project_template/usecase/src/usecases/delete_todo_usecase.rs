use util::AppResult;
use util::Todo;

#[cfg_attr(feature = "mock", mockall::automock)]
pub trait DeleteTodoUseCase {
    async fn run(&self, id: u32) -> AppResult<()>;
}
