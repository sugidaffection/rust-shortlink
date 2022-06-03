use crate::utils::{redirect_to, AppData};
use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};
use tera::Context;
use uuid::Uuid;

#[get("/profile")]
async fn profile(data: web::Data<AppData>, session: Session) -> impl Responder {
    let data = data.into_inner();
    let mut ctx = Context::new();
    ctx.insert("title", "Shortlink");
    if let Some(user_session) = session.get::<Uuid>("user_session").unwrap() {
        ctx.insert("user", &user_session);
        let render = data.tera.render("profile.html", &ctx).unwrap();
        return HttpResponse::Ok().body(render);
    }

    redirect_to("/auth/login")
}

#[get("/settings")]
async fn settings(data: web::Data<AppData>, session: Session) -> impl Responder {
    let data = data.into_inner();
    let mut ctx = Context::new();
    ctx.insert("title", "Shortlink");
    if let Some(user_session) = session.get::<Uuid>("user_session").unwrap() {
        ctx.insert("user", &user_session);
        let render = data.tera.render("profile.html", &ctx).unwrap();
        return HttpResponse::Ok().body(render);
    }

    redirect_to("/auth/login")
}

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").service(profile).service(settings));
}
