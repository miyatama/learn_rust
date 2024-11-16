use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct UserFirstName(String);

impl UserFirstName {
    pub fn new(first_name: impl ToString) -> UserFirstName {
        Self(first_name.to_string())
    }
}

impl Display for UserFirstName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
