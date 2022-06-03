mod routes;
use routes::{auth, home, user};

use actix_session::{storage, SessionMiddleware};
use actix_web::{cookie, middleware, web, App, HttpServer};
use tera::Tera;

mod utils;
use utils::AppData;

use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap();
        let key = env::var("SECRET_KEY")
            .map_or(cookie::Key::generate(), |x| cookie::Key::from(x.as_bytes()));
        App::new()
            .app_data(web::Data::new(AppData { tera: tera }))
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
