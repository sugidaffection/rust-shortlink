use crate::utils::AppData;
use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};
use tera::Context;
use uuid::Uuid;

#[get("/")]
async fn index(data: web::Data<AppData>, session: Session) -> impl Responder {
    let data = data.into_inner();
    let mut ctx = Context::new();
    ctx.insert("title", "Shortlink");
    if let Some(user_session) = session.get::<Uuid>("user_session").unwrap() {
        ctx.insert("user", &user_session);
    }
    let render = data.tera.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(render)
}

#[get("/{code}")]
async fn redirect(
    path: web::Path<String>,
    data: web::Data<AppData>,
    session: Session,
) -> impl Responder {
    let path = path.into_inner();
    let data = data.into_inner();
    let mut ctx = Context::new();
    ctx.insert("title", "Shortlink");
    if let Some(user_session) = session.get::<Uuid>("user_session").unwrap() {
        ctx.insert("user", &user_session);
    }
    let render = data.tera.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(render)
}
