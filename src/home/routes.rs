use actix_web::web;

use super::handlers::{generate_shortlink, home, redirect};

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(home)
        .service(generate_shortlink)
        .service(redirect);
}
