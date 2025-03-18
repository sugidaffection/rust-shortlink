use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateShortLinkDto {
    pub url: String,
}

#[derive(Serialize)]
pub struct ShortLinkDto {
    pub long_url: String,
    pub short_url: Option<String>,
    pub status: String,
    pub is_private: bool,
    pub created_at: chrono::NaiveDateTime,
}
