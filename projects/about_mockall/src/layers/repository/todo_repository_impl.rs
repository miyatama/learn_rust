use crate::layers::{Todo, TodoClientImpl, TodoRepository};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TodoRepositoryImpl {
    client: Arc<TodoClientImpl>,
}

impl TodoRepositoryImpl {
    pub fn new(client: TodoClientImpl) -> Self {
        Self { client: Arc::new(client) }
    }
}

impl TodoRepository for TodoRepositoryImpl {
    fn get_todos(&self) -> Vec<Todo> {
        vec![]
    }
}
