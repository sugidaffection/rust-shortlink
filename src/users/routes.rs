use actix_web::web;

use super::handlers::{profile, settings};

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").service(profile).service(settings));
}
