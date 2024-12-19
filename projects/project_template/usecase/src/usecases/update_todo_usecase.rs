use util::AppResult;
use util::Todo;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait UpdateTodoUseCase {
    fn run(&self, id: u32, text: String) -> AppResult<Todo>;
}
