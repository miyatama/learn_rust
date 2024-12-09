use util::AppResult;
use util::Todo;

#[cfg_attr(feature = "mock", mockall::automock)]
pub trait TodoApiClient {
    fn list(&self) -> Vec<Todo>;
    fn create(&self, todo: Todo) -> AppResult<()>;
    fn update(&self, todo: Todo) -> AppResult<()>;
}
