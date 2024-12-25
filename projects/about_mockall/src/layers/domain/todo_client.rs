use crate::layers::Todo;

pub trait TodoClient {
    fn get_todos(&self) -> Vec<Todo>;
}
