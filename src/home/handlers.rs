use crate::models::LongURL;
use crate::prelude::*;
use crate::services::short_link::ShortLinkService;
use actix_session::Session;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use tera::Context;
use uuid::Uuid;

#[get("/")]
async fn home(data: web::Data<AppData>, session: Session, req: HttpRequest) -> impl Responder {
    let data = data.into_inner();
    let mut ctx = Context::new();
    ctx.insert("title", "Shortlink");
    if let Ok(_) = session.get::<String>("user_session") {
        ctx.insert("is_logged_in", &true);
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

    if let Ok(Some(user_session)) = session.get::<Uuid>("user_session") {
        ctx.insert("is_logged_in", &true);
        let pool = data.pool.clone();
        let conn = &mut pool.get().ok().expect("Pool Connection doesnt exists");
        match ShortLinkService::create(conn, form, user_session) {
            Ok(short_link) => {
                let url = req
                    .url_for("redirect", &[short_link.hash.unwrap()])
                    .ok()
                    .unwrap()
                    .to_string();
                ctx.insert("url", &url);
                let render = data.tera.render("index.html", &ctx).unwrap();

                return HttpResponse::Ok().body(render);
            }
            Err(_) => {
                ctx.insert("error", "Failed to generate short url");
                let render = data.tera.render("index.html", &ctx).unwrap();

                return HttpResponse::Ok().body(render);
            }
        }
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
    let conn = &mut pool.get().ok().expect("Pool Connection doesnt exists");
    let long_url = ShortLinkService::get(conn, path);
    if let Some(url) = long_url {
        return redirect_to(url.as_str());
    }

    redirect_to("/")
}
