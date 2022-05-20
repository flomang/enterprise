#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;

pub mod email_service;
pub mod models;
pub mod schema;
pub mod config;
pub mod api;
pub mod constants;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}