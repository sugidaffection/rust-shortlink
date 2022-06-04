use secrecy::Secret;
use serde::{de, Deserialize};

#[derive(Clone, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_bool")]
    pub remember: bool,
}

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer).unwrap_or("n");

    Ok(["on", "yes", "true", "1", "checked", "y"].contains(&s))
}

#[derive(Clone, Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub email: String,
    pub password: Secret<String>,
}
