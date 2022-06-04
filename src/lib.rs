#![allow(dead_code, unused_variables)]

#[macro_use]
extern crate diesel;

use actix_web::http::header;
use actix_web::HttpResponse;
use diesel::{r2d2, PgConnection};
use std::env;
use tera::Tera;

pub mod database;
pub mod forms;
pub mod handlers;
pub mod routes;

pub use {database::*, forms::*, handlers::*, routes::*};

pub type Pool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

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

pub fn redirect_to(location: &'static str) -> HttpResponse {
    HttpResponse::Found()
        .append_header((header::LOCATION, location))
        .finish()
}
