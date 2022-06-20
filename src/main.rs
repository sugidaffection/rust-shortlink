use actix_session::{storage, SessionMiddleware};
use actix_web::{cookie, middleware, web, App, HttpServer};

use dotenv::dotenv;
use shortlink::{auth, create_pool, home, users, AppData};
use std::env;

#[macro_use]
extern crate diesel_migrations;
use diesel_migrations::embed_migrations;
embed_migrations!();

async fn run_db_migrations() {
    let conn = create_pool(None).get().unwrap();
    embedded_migrations::run(&*conn).expect("failed to run migrations")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    run_db_migrations().await;
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
            .configure(home::routes::routes_config)
            .configure(users::routes::routes_config)
            .configure(auth::routes::routes_config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
