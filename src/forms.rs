use secrecy::Secret;
use serde::{de, Deserialize};

use crate::models::LoginUser;

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

fn deserialize_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: de::Deserializer<'de>,
{
    if let Ok(s) = de::Deserialize::deserialize(deserializer) {
        return Ok(Some(
            ["on", "yes", "true", "1", "checked", "y"].contains(&s),
        ));
    }
    Ok(None)
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
