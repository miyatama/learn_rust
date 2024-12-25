use crate::layers::{Todo, TodoClient};

#[derive(Debug, Clone)]
pub struct TodoClientImpl {}

impl TodoClientImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl TodoClient for TodoClientImpl {
    fn get_todos(&self) -> Vec<Todo> {
        vec![]
    }
}
