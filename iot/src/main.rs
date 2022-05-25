#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use clap::{AppSettings, ArgMatches, SubCommand, Parser};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod api;
mod database;
mod errors;
mod config;
mod middleware;
mod models;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, default_value_t = 3010)]
    port: i32,
}

// Tokio-based single-threaded async runtime for the Actix ecosystem.
// To achieve similar performance to multi-threaded, work-stealing runtimes, applications using actix-rt will create multiple, mostly disconnected, single-threaded runtimes.
// This approach has good performance characteristics for workloads where the majority of tasks have similar runtime expense.
// https://docs.rs/actix-rt/latest/actix_rt/index.html
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let cli = Cli::parse();

    // Start http server
    HttpServer::new(move || {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let allowed_origin: String =
            std::env::var("ALLOWED_ORIGIN").unwrap_or_else(|_| "localhost:3001".to_string());

        // create db connection pool
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: library::db::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let cors = Cors::default()
            .allowed_origin_fn(move |origin, _req_head| {
                origin.as_bytes().ends_with(allowed_origin.as_bytes())
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::ACCEPT, http::header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool))
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            .wrap(middleware::timer::ResponseTime)
            .configure(config::config_services)
            .app_data(web::JsonConfig::default().limit(4096))
    })
    .bind(format!("127.0.0.1:{}", cli.port))?
    .run()
    .await
}
