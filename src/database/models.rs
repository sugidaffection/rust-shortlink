use crate::{hash_password, RegisterForm};

use super::schema::{short_link, users};
use chrono::NaiveDateTime;

use secrecy::Secret;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "short_link"]
pub struct ShortLink {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub hash: String,
    pub long_url: String,
    pub is_private: Option<bool>,
    pub created_at: NaiveDateTime,
    pub uid: i32,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub email: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: Secret<String>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

impl RegisterUser {
    pub fn from(form: RegisterForm) -> Self {
        RegisterUser {
            username: form.username,
            email: form.email,
            password_hash: hash_password(form.password).expect("Failed to hash password"),
        }
    }
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "short_link"]
pub struct NewShortLink {
    pub long_url: String,
    pub owner_id: Uuid,
    pub hash: String,
}
