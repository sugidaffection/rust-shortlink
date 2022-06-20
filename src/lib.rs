#![allow(dead_code, unused_variables)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate secrecy;

use actix_web::http::header;
use actix_web::HttpResponse;
use bcrypt::{hash, verify};
use diesel::{r2d2, PgConnection};
use secrecy::{ExposeSecret, Secret};
use serde::de;
use std::env;
use tera::Tera;

pub mod auth;
pub mod home;
pub mod users;

pub mod database;
pub mod errors;
pub mod forms;
pub mod models;
pub mod services;

pub type Pool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;
pub type PConn = r2d2::PooledConnection<r2d2::ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct AppData {
    pub tera: Tera,
    pub pool: Pool,
}

impl AppData {
    pub fn new() -> Self {
        let tera = Tera::new("templates/**/*").expect("Template folder not found");
        let pool: Pool = create_pool(None);

        Self {
            tera: tera,
            pool: pool,
        }
    }
}

pub fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn create_connection_manager(database_url: String) -> r2d2::ConnectionManager<PgConnection> {
    r2d2::ConnectionManager::<PgConnection>::new(database_url)
}

pub fn create_pool(database_url: Option<String>) -> Pool {
    let database_url = database_url.unwrap_or(get_database_url());
    let manager = create_connection_manager(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn redirect_to(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .append_header((header::LOCATION, location))
        .finish()
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

pub fn hash_password(password: Secret<String>) -> Option<String> {
    hash(password.expose_secret(), 12).ok()
}

pub fn verify_password(password: Secret<String>, hash: Secret<String>) -> Option<bool> {
    verify(
        password.expose_secret().to_owned(),
        hash.expose_secret().as_str(),
    )
    .ok()
}

fn deserialize_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
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
