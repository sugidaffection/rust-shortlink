use crate::deserialize_bool;
use crate::models::LoginUser;
use secrecy::Secret;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: Secret<String>,
    #[serde(default, deserialize_with = "deserialize_bool")]
    pub remember: Option<bool>,
}

impl Into<LoginUser> for LoginForm {
    fn into(self) -> LoginUser {
        LoginUser {
            email: self.email,
            password: self.password,
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub email: String,
    pub password: Secret<String>,
}

#[derive(Clone, Deserialize)]
pub struct LongURLForm {
    pub url: String,
}
