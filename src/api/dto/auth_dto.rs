use secrecy::SecretString;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub password: SecretString,
}

#[derive(Deserialize)]
pub struct RegisterDto {
    pub email: String,
    pub password: SecretString,
    pub username: String,
}
