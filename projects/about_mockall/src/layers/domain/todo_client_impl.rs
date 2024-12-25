use crate::layers::{Todo, TodoClient};
#[cfg(test)]
use mockall::automock;

#[derive(Debug, Clone)]
pub struct TodoClientImpl {}

impl TodoClientImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg_attr(test, automock)]
impl TodoClient for TodoClientImpl {
    fn get_todos(&self) -> Vec<Todo> {
        vec![
            Todo::new("test01".to_string()),
            Todo::new("test02".to_string()),
            Todo::new("test03".to_string()),
            Todo::new("test04".to_string()),
            Todo::new("test05".to_string()),
        ]
    }
}
