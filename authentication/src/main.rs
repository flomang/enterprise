extern crate diesel;

use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use time::Duration;

use authentication::handlers::{auth, invitation, register};
use authentication::models;
use kitchen::utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: models::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    // Start http server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(utils::auth::cookie_policy(domain.clone(), Duration::new(86400, 0)))
            .app_data(web::JsonConfig::default().limit(4096))
            // everything under '/api/' route
            .service(
                web::scope("/api")
                    .service(web::scope("/invitations").service(invitation::create_invitation))
                    .service(web::scope("/register").service(register::register_user))
                    .service(web::scope("/login").service(auth::login))
                    .service(web::scope("/logout").service(auth::logout)),
            )
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
