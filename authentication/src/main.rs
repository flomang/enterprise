extern crate diesel;

use actix_cors::Cors;
use actix_web::{http, middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use authentication::models;
use authentication::config;

const KEY: [u8; 16] = *include_bytes!("secret.key");
const IGNORE_ROUTES: [&str; 3] = ["/api/ping", "/api/login", "/api/register"];

// Tokio-based single-threaded async runtime for the Actix ecosystem.
// To achieve similar performance to multi-threaded, work-stealing runtimes, applications using actix-rt will create multiple, mostly disconnected, single-threaded runtimes.
// This approach has good performance characteristics for workloads where the majority of tasks have similar runtime expense.
// https://docs.rs/actix-rt/latest/actix_rt/index.html
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // Start http server
    HttpServer::new(move || {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        //let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());
        let allowed_origin: String =
            std::env::var("ALLOWED_ORIGIN").unwrap_or_else(|_| "localhost:3001".to_string());

        // create db connection pool
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: models::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let cors = Cors::default()
            .allowed_origin_fn(move |origin, _req_head| {
                origin.as_bytes().ends_with(allowed_origin.as_bytes())
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::ACCEPT, http::header::CONTENT_TYPE])
            .max_age(3600);

        //let auth = HttpAuthentication::bearer(library::auth::bearer_auth_validator);

        App::new()
            .app_data(web::Data::new(pool))
            .wrap(cors)
            .wrap(middleware::Logger::default())
            // routes here need auth
            .wrap(authentication::middleware::auth_middleware::Authentication::new(&KEY, &IGNORE_ROUTES)) // Comment this line of code if you want to integrate with yew-address-book-frontend
            .configure(config::app::config_services)
            //.wrap(auth)
            //.wrap(utils::auth::cookie_policy(domain, Duration::new(86400, 0)))
            .app_data(web::JsonConfig::default().limit(4096))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
