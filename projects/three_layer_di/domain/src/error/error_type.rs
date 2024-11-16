#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MyErrorType {
    NotFound,
    Duplicate,
}
