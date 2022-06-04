use actix_session::{storage, SessionMiddleware};
use actix_web::{cookie, middleware, web, App, HttpServer};

use dotenv::dotenv;
use shortlink::AppData;
use shortlink::{auth, home, user};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        let key = env::var("SECRET_KEY").map_or(cookie::Key::generate(), |x| {
            cookie::Key::derive_from(x.as_bytes())
        });
        App::new()
            .app_data(web::Data::new(AppData::new()))
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::trim())
            .wrap(SessionMiddleware::new(
                storage::CookieSessionStore::default(),
                key.clone(),
            ))
            .service(home::index)
            .service(home::redirect)
            .configure(user::routes_config)
            .configure(auth::routes_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
