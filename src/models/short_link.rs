use crate::database::schema::short_link;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Queryable, Deserialize, Serialize)]
pub struct ShortLink {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub hash: String,
    pub long_url: String,
    pub is_private: Option<bool>,
    pub created_at: NaiveDateTime,
    pub uid: i32,
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "short_link"]
pub struct NewShortLink {
    pub long_url: String,
    pub owner_id: Uuid,
    pub hash: String,
}

#[derive(Clone, Deserialize)]
pub struct LongURL {
    pub url: String,
}
