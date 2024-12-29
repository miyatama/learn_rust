use util::AppResult;
use util::Todo;

#[cfg_attr(feature = "mock", mockall::automock)]
pub trait TodoApiClient {
    fn list(&self) -> AppResult<Vec<Todo>>;
    fn create(&self, todo: Todo) -> AppResult<Todo>;
    fn update(&self, todo: Todo) -> AppResult<Todo>;
    fn delete(&self, todo: Todo) -> AppResult<()>;
}
