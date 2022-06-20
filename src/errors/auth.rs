pub struct AuthError {
    message: String,
    error: String,
}

impl AuthError {
    pub fn new(message: String, error: String) -> Self {
        Self {
            message: message,
            error: error,
        }
    }
}
