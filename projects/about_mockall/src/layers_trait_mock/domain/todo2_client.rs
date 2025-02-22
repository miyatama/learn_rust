use crate::layers_trait_mock::Todo2;
use mockall::automock;

#[automock]
pub trait Todo2Client {
    #[cfg_attr(test, allow(dead_code))]
    fn get_todos(&self) -> Vec<Todo2>;
}
