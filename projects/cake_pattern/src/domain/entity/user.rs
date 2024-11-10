use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub effective: bool,
}
