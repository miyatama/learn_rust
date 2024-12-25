use crate::layers::Todo;

pub trait TodoClient {
    #[cfg_attr(test, allow(dead_code))]
    fn get_todos(&self) -> Vec<Todo>;
}
