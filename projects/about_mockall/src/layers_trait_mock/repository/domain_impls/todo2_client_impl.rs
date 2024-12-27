use crate::layers_trait_mock::{Todo2, Todo2Client};

#[derive(Debug, Clone)]
pub struct Todo2ClientImpl {}

impl Todo2ClientImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl Todo2Client for Todo2ClientImpl {
    fn get_todos(&self) -> Vec<Todo2> {
        vec![
            Todo2::new("test01".to_string()),
            Todo2::new("test02".to_string()),
            Todo2::new("test03".to_string()),
            Todo2::new("test04".to_string()),
            Todo2::new("test05".to_string()),
        ]
    }
}
