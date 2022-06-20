use crate::{
    redirect_to,
    services::{short_link::ShortLinkService, user::UserService},
    AppData,
};
use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};
use tera::Context;

#[get("/profile")]
async fn profile(data: web::Data<AppData>, session: Session) -> impl Responder {
    let data = data.into_inner();
    let mut ctx = Context::new();
    ctx.insert("title", "Shortlink");
    if let Some(user_session) = session.get::<String>("user_session").unwrap() {
        let pool = data.pool.clone();
        let conn = &pool.get().ok().expect("Pool Connection doesnt exists");

        let user = UserService::get_one_by_email(conn, user_session.clone());
        if let Some(user) = user.ok() {
            ctx.insert("user", &user);
            let shortlinks = ShortLinkService::get_all(conn, user.id);
            if let Some(shortlinks) = shortlinks.ok() {
                ctx.insert("shortlinks", &shortlinks);
            }
        }

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
    if let Some(user_session) = session.get::<String>("user_session").unwrap() {
        ctx.insert("user", &user_session);
        let render = data.tera.render("profile.html", &ctx).unwrap();
        return HttpResponse::Ok().body(render);
    }

    redirect_to("/auth/login")
}
