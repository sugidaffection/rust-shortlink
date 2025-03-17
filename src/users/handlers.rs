use crate::prelude::*;
use crate::services::{short_link::ShortLinkService, user::UserService};
use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};
use tera::Context;
use uuid::Uuid;

#[get("/profile")]
async fn profile(data: web::Data<AppData>, session: Session) -> impl Responder {
    let data = data.into_inner();
    let mut ctx = Context::new();
    ctx.insert("title", "Shortlink");
    if let Ok(Some(user_session)) = session.get::<Uuid>("user_session") {
        let pool = data.pool.clone();
        let conn = &mut pool.get().ok().expect("Pool Connection doesnt exists");

        let user = UserService::get_user_by_id(conn, user_session);
        if let Ok(user) = user {
            ctx.insert("is_logged_in", &true);
            ctx.insert("user", &user);
            let shortlinks = ShortLinkService::get_all(conn, user.id);
            if let Some(shortlinks) = shortlinks.ok() {
                ctx.insert("shortlinks", &shortlinks);
            }
            let render = data.tera.render("profile.html", &ctx).unwrap();
            return HttpResponse::Ok().body(render);
        }
    }

    redirect_to("/auth/login")
}

#[get("/settings")]
async fn settings(data: web::Data<AppData>, session: Session) -> impl Responder {
    let data = data.into_inner();
    let pool = data.pool.clone();

    let conn = &mut pool.get().ok().expect("Pool Connection doesnt exists");
    let mut ctx = Context::new();
    ctx.insert("title", "Shortlink");
    if let Ok(Some(user_session)) = session.get::<Uuid>("user_session") {
        if let Ok(user) = UserService::get_user_by_id(conn, user_session) {
            ctx.insert("is_logged_in", &true);
            ctx.insert("user", &user);
            let render = data.tera.render("profile.html", &ctx).unwrap();
            return HttpResponse::Ok().body(render);
        }
    }

    redirect_to("/auth/login")
}
