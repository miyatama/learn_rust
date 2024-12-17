use util::AppResult;
use util::Todo;

#[cfg_attr(feature = "mock", mockall::automock)]
pub trait TodoRepository {
    async fn create(&self, text: String) -> AppResult<Todo>;
    async fn update(&self, todo: Todo) -> AppResult<Todo>;
    async fn delete(&self, id: u32) -> AppResult<()>;
    async fn list(&self) -> AppResult<Vec<Todo>>;
}
