mod app_logger;
mod data;

pub use app_logger::AppLogger;

pub use data::todo::Todo;

use std::error::Error;

pub type AppResult<T> = Result<T, Box<dyn Error>>;
