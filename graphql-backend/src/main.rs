#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
extern crate jsonwebtoken as jwt;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod graphql;
mod db;
mod error;
mod models;
mod prelude;
mod utils;

use std::env;

fn main() {
    dotenv::dotenv().ok();

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "graphql_backend=debug,actix_web=info");
    }
    env_logger::init();
    let _ = graphql::server::start_server();
}
