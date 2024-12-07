mod app_logger;

pub use app_logger::AppLogger;

use std::error::Error;

pub type AppResult<T> = Result<T, Box<dyn Error>>;
