use std::env;

use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};

pub mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn create_pool(database_url: Option<&str>) -> DbPool {
    let database_url = database_url
        .map(ToOwned::to_owned)
        .unwrap_or_else(get_database_url);
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let max_size = env::var("DATABASE_POOL_MAX_SIZE")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(10); // Default to 10

    DbPool::builder()
        .max_size(max_size)
        .build(manager)
        .expect("Failed to create pool.")
}

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::errors::AppError;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub async fn run_db_migrations() -> Result<(), AppError> {
    let pool = create_pool(None);
    let mut conn = pool
        .get()
        .map_err(|err| AppError::MigrationError(err.to_string()))?;

    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|err| AppError::MigrationError(err.to_string()))?;

    Ok(())
}
