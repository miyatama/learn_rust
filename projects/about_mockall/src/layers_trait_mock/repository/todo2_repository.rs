use crate::layers_trait_mock::Todo2;

#[cfg_attr(test, allow(dead_code))]
pub trait Todo2Repository {
    fn get_todos(&self) -> Vec<Todo2>;
}
