use diesel::result::Error as DieselError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShortLinkError {
    #[error("Database error: {0}")]
    Diesel(#[from] DieselError), // Converts Diesel errors automatically

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

impl ShortLinkError {
    pub fn new_unexpected(error: impl Into<String>) -> Self {
        Self::Unexpected(error.into())
    }
}
