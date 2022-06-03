use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize)]
pub struct ShortURL {
    pub id: Uuid,
    pub hash: String,
    pub long_url: String,
    pub created_at: NaiveDateTime,
    pub owner_id: String,
    pub is_private: bool,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}
