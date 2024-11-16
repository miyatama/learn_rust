pub mod error_type;

use std::{error::Error, fmt::Display};

use crate::MyErrorType;

pub type MyResult<T> = Result<T, MyError>;

#[derive(Debug)]
pub struct MyError {
    error_type: MyErrorType,
    desc: String,
}

impl MyError {
    pub fn new(error_type: MyErrorType, desc: impl ToString) -> Self {
        Self {
            error_type: error_type,
            desc: desc.to_string(),
        }
    }
}

impl Error for MyError {}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.error_type, self.desc)
    }
}
