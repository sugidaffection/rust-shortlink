use crate::forms::RegisterForm;
use actix_web::web;

pub fn register_user(form: web::Form<RegisterForm>) {}
