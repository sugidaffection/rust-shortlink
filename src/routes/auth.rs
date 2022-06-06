use crate::forms::{LoginForm, RegisterForm};
use crate::models::RegisterUser;
use crate::{
    auth_handlers::{login_user, register_user},
    redirect_to, AppData,
};
use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};
use tera::Context;

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
    if let Some(user) = login_user(pool, form.into()) {
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
    if let Some(user) = register_user(pool, RegisterUser::from(form)) {
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
