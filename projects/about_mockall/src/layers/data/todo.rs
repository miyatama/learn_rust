#[derive(Debug, Clone, PartialEq)]
pub struct Todo {
    pub title: String,
}

impl Todo {
    pub fn new(title: String) -> Self {
        Self {
            title: title.clone(),
        }
    }
}
