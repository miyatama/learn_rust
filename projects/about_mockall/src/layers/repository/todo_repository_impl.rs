use crate::layers::{Todo, TodoClient, TodoClientImpl, TodoRepository};
#[cfg(test)]
use mockall::automock;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TodoRepositoryImpl {
    #[cfg_attr(test, allow(dead_code))]
    client: Arc<TodoClientImpl>,
}

impl TodoRepositoryImpl {
    pub fn new(client: TodoClientImpl) -> Self {
        Self {
            client: Arc::new(client),
        }
    }
}

#[cfg_attr(test, automock)]
impl TodoRepository for TodoRepositoryImpl {
    fn get_todos(&self) -> Vec<Todo> {
        self.client.get_todos()
    }
}
