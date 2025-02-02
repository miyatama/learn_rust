use crate::models::todo::Todo;

#[derive(Default)]
pub struct TodoServiceData {
    pub todos: Vec<Todo>,
}

impl TodoServiceData {
    #[allow(clippy::new_without_default, dead_code)]
    pub fn new() -> Self {
        Self { todos: vec![] }
    }

    pub fn todos(&self) -> Vec<Todo> {
        self.todos.clone()
    }
}
