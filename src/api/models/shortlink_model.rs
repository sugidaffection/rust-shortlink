use diesel::prelude::*;

use crate::database::schema::short_links;

#[derive(Insertable)]
#[diesel(table_name = short_links)]
pub struct NewShortLink {
    pub owner_id: Option<uuid::Uuid>,
    pub anonymous_owner_id: Option<uuid::Uuid>,
    pub long_url: String,
    pub status: String,
    pub is_private: bool,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = short_links)]
pub struct ShortLink {
    pub id: uuid::Uuid,
    pub owner_id: Option<uuid::Uuid>,
    pub anonymous_owner_id: Option<uuid::Uuid>,
    pub serial_id: i64,
    pub long_url: String,
    pub hash: Option<String>,
    pub status: String,
    pub is_private: bool,
    pub created_at: chrono::NaiveDateTime,
}
