use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: String,
    pub effective: bool,
}
