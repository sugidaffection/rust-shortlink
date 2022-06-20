use actix_web::web;

use super::handlers::{generate_shortlink, index, redirect};

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(generate_shortlink)
        .service(redirect);
}
