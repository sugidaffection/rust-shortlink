use super::schema::{short_link, users};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "short_link"]
pub struct ShortLink {
    pub id: Uuid,
    pub hash: String,
    pub long_url: String,
    pub created_at: NaiveDateTime,
    pub owner_id: Uuid,
    pub is_private: bool,
}

#[derive(Insertable, Queryable, Serialize, Deserialize, Debug)]
#[table_name = "users"]
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
