use actix_web::{
    error::{Error as ActixError, JsonPayloadError},
    http::StatusCode,
    HttpRequest, HttpResponse, ResponseError,
};
use bcrypt::BcryptError;
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DieselError},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// A common error response format
#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

// All API errors derive from this enum
#[derive(Debug, Error)]
pub enum ApiError {
    // Auth errors
    #[error("Invalid username or password")]
    InvalidCredentials,
    #[error("Username already exists")]
    DuplicateUsername,
    #[error("Email already exists")]
    DuplicateEmail,

    // User errors
    #[error("User not found")]
    UserNotFound,

    // Database errors
    #[error("Database connection failed")]
    DbConnectionError(#[from] PoolError),
    #[error("Database error")]
    DatabaseError(String),

    // Validation errors
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Invalid request payload")]
    InvalidPayload(#[source] JsonPayloadError),

    // Authentication/authorization
    #[error("Authentication required")]
    Unauthorized,
    #[error("Insufficient permissions")]
    Forbidden,

    // Other errors
    #[error("Request not found")]
    NotFound,
    #[error("Internal server error")]
    InternalError,
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

impl ApiError {
    pub fn name(&self) -> String {
        match self {
            Self::InvalidCredentials => "INVALID_CREDENTIALS",
            Self::DuplicateUsername => "DUPLICATE_USERNAME",
            Self::DuplicateEmail => "DUPLICATE_EMAIL",
            Self::UserNotFound => "USER_NOT_FOUND",
            Self::DbConnectionError(_) => "DB_CONNECTION_ERROR",
            Self::DatabaseError(_) => "DATABASE_ERROR",
            Self::ValidationError(_) => "VALIDATION_ERROR",
            Self::InvalidPayload(_) => "INVALID_PAYLOAD",
            Self::Unauthorized => "UNAUTHORIZED",
            Self::Forbidden => "FORBIDDEN",
            Self::NotFound => "NOT_FOUND",
            Self::InternalError => "INTERNAL_ERROR",
            Self::Unexpected(_) => "UNEXPECTED_ERROR",
        }
        .to_string()
    }

    pub fn with_details(self, details: impl ToString) -> Self {
        match self {
            Self::Unexpected(_) => Self::Unexpected(details.to_string()),
            Self::ValidationError(_) => Self::ValidationError(details.to_string()),
            _ => self,
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::DuplicateUsername | Self::DuplicateEmail => StatusCode::CONFLICT,
            Self::UserNotFound | Self::NotFound => StatusCode::NOT_FOUND,
            Self::DbConnectionError(_) | Self::DatabaseError(_) | Self::InternalError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::ValidationError(_) | Self::InvalidPayload(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();

        // For internal errors, don't expose details in production
        let details = match self {
            Self::DbConnectionError(_) | Self::DatabaseError(_) | Self::InternalError => None,

            Self::ValidationError(details) | Self::Unexpected(details) => Some(details.clone()),

            Self::InvalidPayload(err) => Some(err.to_string()),

            _ => None,
        };

        HttpResponse::build(status).json(ErrorResponse {
            error: self.name(),
            message: self.to_string(),
            details,
        })
    }
}

// Convert from specialized errors to ApiError
impl From<DieselError> for ApiError {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => ApiError::UserNotFound,
            DieselError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let msg = info.message().to_lowercase();
                    if msg.contains("username") {
                        ApiError::DuplicateUsername
                    } else if msg.contains("email") {
                        ApiError::DuplicateEmail
                    } else {
                        ApiError::Unexpected(info.message().to_string())
                    }
                } else {
                    ApiError::DatabaseError(info.message().to_string())
                }
            }
            _ => ApiError::DatabaseError(err.to_string()),
        }
    }
}

impl From<BcryptError> for ApiError {
    fn from(_: BcryptError) -> Self {
        ApiError::InternalError
    }
}

impl From<JsonPayloadError> for ApiError {
    fn from(err: JsonPayloadError) -> Self {
        ApiError::InvalidPayload(err)
    }
}

// Custom error handler for JSON payload errors
pub fn json_error_handler(err: JsonPayloadError, _req: &HttpRequest) -> ActixError {
    ApiError::from(err).into()
}

// You can define legacy error types that convert to ApiError
#[derive(Debug, Error, PartialEq)]
pub enum AuthError {
    #[error("Invalid username or password")]
    InvalidCredentials,
    #[error("Username already exists")]
    DuplicateUsername,
    #[error("Email already exists")]
    DuplicateEmail,
    #[error("Unexpected authentication error: {0}")]
    Unexpected(String),
    #[error("Internal authentication error")]
    InternalError,
}

impl From<AuthError> for ApiError {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::InvalidCredentials => ApiError::InvalidCredentials,
            AuthError::DuplicateUsername => ApiError::DuplicateUsername,
            AuthError::DuplicateEmail => ApiError::DuplicateEmail,
            AuthError::Unexpected(msg) => ApiError::Unexpected(msg),
            AuthError::InternalError => ApiError::InternalError,
        }
    }
}

// Common result type for API functions
pub type Result<T> = std::result::Result<T, ApiError>;
