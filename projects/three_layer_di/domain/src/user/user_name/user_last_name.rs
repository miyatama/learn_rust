use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct UserLastName(String);

impl UserLastName {
    pub fn new(last_name: impl ToString) -> UserLastName {
        Self(last_name.to_string())
    }
}

impl Display for UserLastName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
