use actix_web::http::header;
use actix_web::HttpResponse;
use bcrypt::{hash, verify};
use secrecy::{ExposeSecret, SecretString};
use serde::de;

pub fn hash_password(password: SecretString) -> Option<String> {
    hash(password.expose_secret(), 12).ok()
}

pub fn verify_password(password: SecretString, hash: SecretString) -> Option<bool> {
    verify(password.expose_secret().to_owned(), hash.expose_secret()).ok()
}

pub fn deserialize_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: de::Deserializer<'de>,
{
    if let Ok(s) = de::Deserialize::deserialize(deserializer) {
        return Ok(Some(
            ["on", "yes", "true", "1", "checked", "y"].contains(&s),
        ));
    }
    Ok(None)
}

pub fn b62encode(mut n: usize) -> Option<String> {
    let base = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let base_length = base.len();
    let mut result = String::new();
    while n > 0 {
        result.insert(0, base.chars().nth(n % base_length).unwrap());
        n /= base_length;
    }

    return Some(result);
}

pub fn b62decode(string: String) -> Option<usize> {
    let base = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let base_length = base.len();

    let mut result: usize = 0;
    for (i, c) in string.chars().enumerate() {
        let pos = base.chars().position(|x| x == c);
        if let None = pos {
            return None;
        };
        let n = string.len() - (i + 1);
        result += pos.unwrap() * base_length.pow(n as u32);
    }

    return Some(result);
}

pub fn redirect_to(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .append_header((header::LOCATION, location))
        .finish()
}
