use util::AppResult;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait DeleteTodoUseCase {
    fn run(&self, id: u32) -> AppResult<()>;
}
