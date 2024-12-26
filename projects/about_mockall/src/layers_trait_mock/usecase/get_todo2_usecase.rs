use crate::layers_trait_mock::Todo2;

pub trait GetTodo2Usecase {
    fn run(&self) -> Vec<Todo2>;
}
