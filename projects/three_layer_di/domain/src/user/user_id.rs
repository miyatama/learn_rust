use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct UserId(u64);

impl UserId {
    pub fn new(id: u64) -> UserId {
        Self(id)
    }
}
