use crate::models::LongURL;
use crate::services::short_link::ShortLinkService;
use crate::{redirect_to, AppData};
use actix_session::Session;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use tera::Context;

#[get("/")]
async fn index(data: web::Data<AppData>, session: Session, req: HttpRequest) -> impl Responder {
    let data = data.into_inner();
    let mut ctx = Context::new();
    ctx.insert("title", "Shortlink");
    if let Some(user_session) = session.get::<String>("user_session").unwrap() {
        ctx.insert("user", &user_session);
        let url = req.url_for("index", &["/"]).ok().unwrap().to_string();
        ctx.insert("url", url.as_str());
    }
    let render = data.tera.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(render)
}

#[post("/")]
async fn generate_shortlink(
    data: web::Data<AppData>,
    form: web::Form<LongURL>,
    req: HttpRequest,
    session: Session,
) -> impl Responder {
    let data = data.into_inner();
    let form = form.into_inner();
    let mut ctx = Context::new();
    ctx.insert("title", "Shortlink");
    if let Some(user_session) = session.get::<String>("user_session").unwrap() {
        ctx.insert("user", &user_session);
        let pool = data.pool.clone();
        let conn = &pool.get().ok().expect("Pool Connection doesnt exists");
        let short_code = ShortLinkService::create(conn, form, user_session).unwrap();
        let url = req.url_for("index", &["/"]).ok().expect("Cant get url");
        ctx.insert("url", url.as_str());
        ctx.insert("code", short_code.as_str());
        let render = data.tera.render("index.html", &ctx).unwrap();
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
    let pool = data.pool.clone();
    let conn = &pool.get().ok().expect("Pool Connection doesnt exists");
    let long_url = ShortLinkService::get(conn, path);
    if let Some(url) = long_url {
        return redirect_to(url.as_str());
    }

    redirect_to("/")
}
