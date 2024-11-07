/**
 * ディレクトリ分割
 */
use crate::domain::value_object::id2::ID2;

#[derive(Debug)]
pub struct Node2 {
    id: ID2,
    label: String,
}
impl Node2 {
    pub fn new(id: ID2, label: &str) -> Self {
        Self {
            id,
            label: label.to_string(),
        }
    }
}