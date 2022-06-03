use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/{code}")]
async fn redirect(path: web::Path<String>) -> impl Responder {
    let code = path.into_inner();
    HttpResponse::Ok().body(format!("Hello world! {}", code))
}
