use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum QuizAppError {
    #[error("setting error: {}", .0)]
    SettingsError(String),
}
