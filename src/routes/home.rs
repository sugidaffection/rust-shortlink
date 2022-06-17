use crate::{redirect_to, shortlink_handler, AppData, LongURLForm};
use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};
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
            shortlink_handler::create_short_link(data.pool.clone(), form, user_session).unwrap();
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
    let long_url = shortlink_handler::get_short_link(data.pool.clone(), path);
    if let Some(url) = long_url {
        return redirect_to(url.as_str());
    }

    redirect_to("/")
}

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(index)
        .service(generate_shortlink)
        .service(redirect);
}
