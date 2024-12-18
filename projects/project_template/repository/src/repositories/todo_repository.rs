use std::future::Future;
use util::AppResult;
use util::Todo;

#[cfg_attr(feature="mock", mockall::automock)]
pub trait TodoRepository {
    fn create(&self, text: String) -> impl Future<Output = AppResult<Todo>> + Send;
    fn update(&self, todo: Todo) -> impl Future<Output = AppResult<Todo>> + Send;
    fn delete(&self, id: u32) -> impl Future<Output = AppResult<()>> + Send;
    fn list(&self) -> impl Future<Output = AppResult<Vec<Todo>>> + Send;
}
