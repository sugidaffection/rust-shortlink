use actix_web::web;

use super::handlers::{handle_login, handle_register, login, logout, register};

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(login)
            .service(handle_login)
            .service(register)
            .service(handle_register)
            .service(logout),
    );
}
