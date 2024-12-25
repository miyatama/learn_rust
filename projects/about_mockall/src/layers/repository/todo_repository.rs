use crate::layers::Todo;

#[cfg_attr(test, allow(dead_code))]
pub trait TodoRepository {
    fn get_todos(&self) -> Vec<Todo>;
}
