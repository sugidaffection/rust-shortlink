use actix_web::web;

use super::errors::json_error_handler;
use super::handlers::{create_shortlink, get_shortlinks, login, profile, redirect, register};

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .app_data(web::JsonConfig::default().error_handler(json_error_handler))
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(login))
                    .route("/register", web::post().to(register)),
            )
            .service(web::scope("/user").route("/me", web::get().to(profile)))
            .service(
                web::scope("/short")
                    .route("", web::post().to(create_shortlink))
                    .route("me", web::get().to(get_shortlinks)),
            ),
    )
    .service(
        web::resource("/{code}")
            .name("redirect")
            .route(web::get().to(redirect)),
    );
}
