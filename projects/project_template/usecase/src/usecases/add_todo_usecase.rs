use std::future::Future;
use util::AppResult;
use util::Todo;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait AddTodoUseCase {
    fn run(&self, text: String) -> impl Future<Output = AppResult<Todo>> + Send;
}
