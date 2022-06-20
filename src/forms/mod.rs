use crate::deserialize_bool;
use secrecy::Secret;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: Secret<String>,
    #[serde(default, deserialize_with = "deserialize_bool")]
    pub remember: Option<bool>,
}

#[derive(Clone, Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub email: String,
    pub password: Secret<String>,
}
