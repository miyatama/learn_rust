use util::AppResult;
use util::todo::ToDo;

#[cfg_attr(feature = "mock", mockall::automock)]
pub trait TodoApiRepository {
    fn list(&self) -> Vec<Todo>;
    fn create(&self, todo: Todo) -> MyResult<()>;
    fn update(&self, todo: Todo) -> MyResult<()>;
}
