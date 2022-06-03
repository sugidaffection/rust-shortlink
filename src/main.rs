mod routes;

use routes::{auth, home, user};

use actix_web::{middleware, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Compress::default())
            .service(home::index)
            .service(home::redirect)
            .configure(user::routes_config)
            .configure(auth::routes_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
