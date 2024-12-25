use crate::layers::Todo;

pub trait GetTodoUsecase {
    fn run(&self) -> Vec<Todo>;
}
