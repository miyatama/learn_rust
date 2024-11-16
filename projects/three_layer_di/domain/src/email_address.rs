use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EmailAddress {
    user: String,
    domain: String,
}

impl EmailAddress {
    pub fn new(s: impl ToString) -> Self {
        let s = s.to_string();
        let mut sp = s.split('@');
        Self {
            user: sp.next().expect("wrong email user format").to_string(),
            domain: sp.next().expect("wrong email domain format").to_string(),
        }
    }
}

impl Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.user, self.domain)
    }
}
