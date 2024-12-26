use crate::layers_trait_mock::{Todo2, Todo2Client, Todo2ClientImpl, Todo2Repository};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Todo2RepositoryImpl {
    #[cfg_attr(test, allow(dead_code))]
    client: Arc<Todo2ClientImpl>,
}

impl Todo2RepositoryImpl {
    pub fn new(client: Todo2ClientImpl) -> Self {
        Self {
            client: Arc::new(client),
        }
    }
}

impl Todo2Repository for Todo2RepositoryImpl {
    fn get_todos(&self) -> Vec<Todo2> {
        self.client.get_todos()
    }
}
