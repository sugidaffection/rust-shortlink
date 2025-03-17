use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};
use tera::Context;

use crate::{
    errors::{SignInError, SignUpError},
    forms::{LoginForm, RegisterForm},
    prelude::*,
    services::auth::AuthService,
};

#[get("/login")]
pub async fn login(data: web::Data<AppData>) -> impl Responder {
    let data = data.into_inner();
    let mut ctx = Context::new();
    ctx.insert("title", "Shortlink | Sign In");
    let render = data.tera.render("login.html", &ctx).unwrap();
    HttpResponse::Ok().body(render)
}

#[post("/login")]
pub async fn handle_login(
    data: web::Data<AppData>,
    form: web::Form<LoginForm>,
    session: Session,
) -> impl Responder {
    let data = data.into_inner();
    let form = form.into_inner();
    let pool = data.pool.clone();
    let conn = &mut pool.get().ok().expect("Pool Connection doesnt exists");

    match AuthService::login(conn, form) {
        Ok(user) => {
            session.insert("user_session", user.id).unwrap();
            return redirect_to("/");
        }
        Err(err) => {
            let mut ctx = Context::new();
            ctx.insert("title", "Shortlink | Sign In");
            ctx.insert("error", &err.to_string());
            let render = data.tera.render("login.html", &ctx).unwrap();
            HttpResponse::Ok().body(render)
        }
    }
}

#[get("/register")]
pub async fn register(data: web::Data<AppData>) -> impl Responder {
    let data = data.into_inner();
    let mut ctx = Context::new();
    ctx.insert("title", "Shortlink | Register");
    let render = data.tera.render("register.html", &ctx).unwrap();
    HttpResponse::Ok().body(render)
}

#[post("/register")]
pub async fn handle_register(
    data: web::Data<AppData>,
    form: web::Form<RegisterForm>,
    session: Session,
) -> impl Responder {
    let data = data.into_inner();
    let form = form.into_inner();
    let pool = data.pool.clone();
    let conn = &mut pool.get().ok().expect("Pool Connection doesnt exists");

    match AuthService::register(conn, form) {
        Ok(user) => {
            session.insert("user_session", user.id).unwrap();
            return redirect_to("/");
        }
        Err(err) => {
            let mut ctx = Context::new();
            ctx.insert("title", "Shortlink | Register");
            ctx.insert("error", &err.to_string());
            let render = data.tera.render("register.html", &ctx).unwrap();
            HttpResponse::Ok().body(render)
        }
    }
}

#[get("/logout")]
pub async fn logout(session: Session) -> impl Responder {
    session.remove("user_session");
    session.purge();
    redirect_to("/auth/login")
}
