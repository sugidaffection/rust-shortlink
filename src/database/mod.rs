use std::env;

use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};

pub mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type PConn = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn create_pool(database_url: Option<&str>) -> DbPool {
    let database_url = database_url
        .map(ToOwned::to_owned)
        .unwrap_or_else(get_database_url);
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let max_size: u32 = env::var("DATABASE_POOL_MAX_SIZE")
        .unwrap_or_else(|_| "10".to_string()) // Default pool size = 10
        .parse()
        .expect("DATABASE_POOL_MAX_SIZE must be a valid number");

    DbPool::builder()
        .max_size(max_size)
        .build(manager)
        .expect("Failed to create pool.")
}
