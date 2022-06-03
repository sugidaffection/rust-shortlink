use actix_web::http::header;
use actix_web::HttpResponse;

use tera::Tera;

use secrecy::Secret;
use serde::Deserialize;

#[derive(Clone)]
pub struct AppData {
    pub tera: Tera,
}

pub fn redirect_to(location: &'static str) -> HttpResponse {
    HttpResponse::Found()
        .append_header((header::LOCATION, location))
        .finish()
}

#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: Secret<String>,
}

#[derive(Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub email: String,
    pub password: Secret<String>,
}
