use crate::layers_trait_mock::Todo2;
use mockall::automock;

#[automock]
pub trait Todo2Repository {
    fn get_todos(&self) -> Vec<Todo2>;
}
