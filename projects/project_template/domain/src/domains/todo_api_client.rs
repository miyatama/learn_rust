use std::future::Future;
use util::AppResult;
use util::Todo;

#[cfg_attr(feature = "mock", mockall::automock)]
pub trait TodoApiClient {
    fn list(&self) -> impl Future<Output = AppResult<Vec<Todo>>> + Sync;
    fn create(&self, todo: Todo) -> impl Future<Output = AppResult<Todo>> + Sync;
    fn update(&self, todo: Todo) -> impl Future<Output = AppResult<Todo>> + Sync;
    fn delete(&self, todo: Todo) -> impl Future<Output = AppResult<()>> + Sync;
}
