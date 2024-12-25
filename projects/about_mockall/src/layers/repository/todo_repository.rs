use crate::layers::Todo;

pub trait TodoRepository {
    fn get_todos(&self) -> Vec<Todo>;
}
