use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn make_connection() {
    dotenv().ok();
}
