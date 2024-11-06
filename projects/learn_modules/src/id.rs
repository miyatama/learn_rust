// 単純にファイル分割

#[derive(Debug)]
pub struct ID {
    value: String,
}
impl ID {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}