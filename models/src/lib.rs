use core::panic;
use diesel::{pg::PgConnection, Connection};
use dotenvy::dotenv;
use std::env;

pub mod post;
pub mod schema;
pub mod tag;
pub mod user;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
