use crate::{b62encode, redirect_to, AppData, LongURLForm};
use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono;
use tera::Context;

#[get("/")]
async fn index(data: web::Data<AppData>, session: Session) -> impl Responder {
    let data = data.into_inner();
    let mut ctx = Context::new();
    ctx.insert("title", "Shortlink");
    if let Some(user_session) = session.get::<String>("user_session").unwrap() {
        ctx.insert("user", &user_session);
    }
    let render = data.tera.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(render)
}

#[post("/")]
async fn generate_shortlink(
    data: web::Data<AppData>,
    form: web::Form<LongURLForm>,
    session: Session,
) -> impl Responder {
    let data = data.into_inner();
    let form = form.into_inner();
    let mut ctx = Context::new();
    ctx.insert("title", "Shortlink");
    if let Some(user_session) = session.get::<String>("user_session").unwrap() {
        ctx.insert("user", &user_session);
        let short_code =
            b62encode(form.url.len() + (chrono::offset::Local::now().timestamp() as usize))
                .unwrap();
        ctx.insert("code", short_code.as_str());
        let render = data.tera.render("generate.html", &ctx).unwrap();
        return HttpResponse::Ok().body(render);
    }

    redirect_to("/auth/login")
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
    if let Some(user_session) = session.get::<String>("user_session").unwrap() {
        ctx.insert("user", &user_session);
    }
    let render = data.tera.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(render)
}

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(index)
        .service(generate_shortlink)
        .service(redirect);
}
