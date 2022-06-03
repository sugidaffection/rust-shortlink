use crate::utils::{redirect_to, AppData, LoginForm, RegisterForm};
use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};
use tera::Context;
use uuid::Uuid;

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
    if !form.email.is_empty() {
        session.insert("user_session", Uuid::new_v4()).unwrap();
        return redirect_to("/");
    }

    let mut ctx = Context::new();
    ctx.insert("title", "Shortlink | Sign In");
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
    let mut ctx = Context::new();
    ctx.insert("title", "Shortlink | Register");
    let render = data.tera.render("register.html", &ctx).unwrap();
    HttpResponse::Ok().body(render)
}

#[get("/logout")]
pub async fn logout(session: Session) -> impl Responder {
    session.remove("user_session");
    redirect_to("/auth/login")
}

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(login)
            .service(handle_login)
            .service(register)
            .service(handle_register)
            .service(logout),
    );
}
