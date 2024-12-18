use std::future::Future;
use util::AppResult;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait DeleteTodoUseCase {
    fn run(&self, id: u32) -> impl Future<Output = AppResult<()>> + Send;
}
