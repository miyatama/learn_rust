use util::AppResult;
use util::Todo;

#[cfg_attr(feature = "mock", mockall::automock)]
pub trait TodoRepository {
    async fn create(&self) -> AppResult<Todo>;
    async fn update(&self, todo: Todo) -> AppResult<Todo>;
    async fn list(&self) -> AppResult<Vec<Todo>>;
}
