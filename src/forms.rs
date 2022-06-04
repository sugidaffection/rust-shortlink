use secrecy::Secret;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: Secret<String>,
    pub remember: bool,
}

#[derive(Clone, Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub email: String,
    pub password: Secret<String>,
}
