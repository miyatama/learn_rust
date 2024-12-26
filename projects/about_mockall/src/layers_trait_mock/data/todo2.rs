#[derive(Debug, Clone, PartialEq)]
pub struct Todo2 {
    pub title: String,
}

impl Todo2 {
    pub fn new(title: String) -> Self {
        Self {
            title: title.clone(),
        }
    }
}
