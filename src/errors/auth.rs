use thiserror::Error;

#[derive(Debug, Error)]
pub enum SignInError {
    #[error("Invalid username or password")]
    InvalidCredentials,

    #[error("Unexpected authentication error: {0}")]
    Unexpected(String),
}

impl SignInError {
    pub fn new_unexpected(error: impl Into<String>) -> Self {
        Self::Unexpected(error.into())
    }
}

#[derive(Debug, Error)]
pub enum SignUpError {
    #[error("Email already in use.")]
    DuplicateEmail,

    #[error("Failed to hash password.")]
    HashingFailed,

    #[error("Failed to register new user.")]
    RegistrationFailed,

    #[error("Unexpected authentication error: {0}")]
    Unexpected(String),
}

impl SignUpError {
    pub fn new_unexpected(error: impl Into<String>) -> Self {
        Self::Unexpected(error.into())
    }
}
