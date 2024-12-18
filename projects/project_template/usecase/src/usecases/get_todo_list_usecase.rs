use std::future::Future;
use util::AppResult;
use util::Todo;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait GetTodoListUseCase {
    fn run(&self) -> impl Future<Output = AppResult<Vec<Todo>>> + Send;
}
