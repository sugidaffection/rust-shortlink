use crate::database::schema::short_links;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = short_links)]
#[diesel(primary_key(id))]
pub struct ShortLink {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub serial_id: i64,
    pub hash: Option<String>,
    pub long_url: String,
    pub is_private: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub expires_at: Option<NaiveDateTime>,
    pub click_count: i64,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: String,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = short_links)]
pub struct NewShortLink {
    pub owner_id: Uuid,
    pub long_url: String,
    pub is_private: bool,
    pub status: String,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct LongURL {
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_private: Option<bool>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = short_links)]
pub struct IdAndSerialId {
    pub id: Uuid,
    pub serial_id: i64,
}
