use actix_session::{storage, SessionMiddleware};
use actix_web::{cookie, middleware, web, App, HttpServer};

use dotenvy::dotenv;
use shortlink::{auth, errors::AppError, home, prelude::*, users};
use std::env;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

async fn run_db_migrations() -> Result<(), AppError> {
    let pool = create_pool(None);
    let mut conn = pool
        .get()
        .map_err(|err| AppError::MigrationError(err.to_string()))?;

    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|err| AppError::MigrationError(err.to_string()))?;

    Ok(())
}

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    dotenv().map_err(AppError::EnvLoadError)?;

    run_db_migrations().await?;

    HttpServer::new(|| {
        let key = env::var("SECRET_KEY").map_or_else(
            |_| cookie::Key::generate(),
            |x| cookie::Key::from(x.as_bytes()),
        );

        App::new()
            .app_data(web::Data::new(AppData::new(None)))
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::trim())
            .wrap(
                SessionMiddleware::builder(storage::CookieSessionStore::default(), key.clone())
                    .cookie_http_only(true)
                    .cookie_same_site(cookie::SameSite::Strict)
                    .cookie_secure(true)
                    .build(),
            )
            .configure(home::routes::routes_config)
            .configure(users::routes::routes_config)
            .configure(auth::routes::routes_config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
