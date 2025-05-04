use thiserror::Error;

#[derive(Error, Debug)]
pub enum QuizAppError {
    #[error("setting error: {0}")]
    SettingsError,
}