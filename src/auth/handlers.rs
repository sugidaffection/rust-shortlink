use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};
use tera::Context;

use crate::{
    forms::{LoginForm, RegisterForm},
    redirect_to,
    services::auth::AuthService,
    AppData,
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
    let conn = &pool.get().ok().expect("Pool Connection doesnt exists");

    if let Some(user) = AuthService::login(conn, form).ok() {
        session.insert("user_session", user.email).unwrap();
        return redirect_to("/");
    }
    let mut ctx = Context::new();
    ctx.insert("title", "Shortlink | Sign In");
    ctx.insert("error", "Wrong username or password");
    let render = data.tera.render("login.html", &ctx).unwrap();
    HttpResponse::Ok().body(render)
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
    let conn = &pool.get().ok().expect("Pool Connection doesnt exists");

    if let Some(user) = AuthService::register(conn, form).ok() {
        session.insert("user_session", user.email).unwrap();
        return redirect_to("/");
    }
    let mut ctx = Context::new();
    ctx.insert("title", "Shortlink | Register");
    ctx.insert("error", "Failed to create user");
    let render = data.tera.render("register.html", &ctx).unwrap();
    HttpResponse::Ok().body(render)
}

#[get("/logout")]
pub async fn logout(session: Session) -> impl Responder {
    session.remove("user_session");
    redirect_to("/auth/login")
}
