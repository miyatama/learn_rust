/**
 * ディレクトリ分割
 */
#[derive(Debug)]
pub struct ID2 {
    value: String,
}
impl ID2 {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}