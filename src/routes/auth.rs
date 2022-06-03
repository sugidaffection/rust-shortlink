use actix_web::{get, web, HttpResponse, Responder};

#[get("/login")]
pub async fn login() -> impl Responder {
    HttpResponse::Ok().body("Login")
}

#[get("/register")]
pub async fn register() -> impl Responder {
    HttpResponse::Ok().body("Register")
}

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(login).service(register));
}
