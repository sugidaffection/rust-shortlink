use actix_web::{get, web, HttpResponse, Responder};

#[get("/profile")]
async fn profile() -> impl Responder {
    HttpResponse::Ok().body("User Profile")
}

#[get("/settings")]
async fn settings() -> impl Responder {
    HttpResponse::Ok().body("User Settings")
}

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").service(profile).service(settings));
}
