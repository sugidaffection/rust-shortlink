use serde::Serialize;

#[derive(Serialize)]
pub struct ShortURL {
    hash: String,
    long_url: String,
    created_at: String,
}
