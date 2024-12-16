use util::AppResult;
use util::Todo;

#[cfg_attr(feature = "mock", mockall::automock)]
pub trait TodoApiClient {
    async fn list(&self) -> AppResult<Vec<Todo>>;
    async fn create(&self, todo: Todo) -> AppResult<Todo>;
    async fn update(&self, todo: Todo) -> AppResult<Todo>;
}
