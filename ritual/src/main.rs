extern crate diesel;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use ritual::handlers;
use ritual::models;
use ritual::utils;
use handlers::{auth, invitation, register, ritual as rite, moment};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var(
        "RUST_LOG",
        "simple-auth-server=debug,actix_web=info,actix_server=info",
    );
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
            .data(pool.clone())
            // enable logger
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(domain.as_str())
                    .max_age(86400) // one day in seconds
                    .secure(false), // this can only be true if you have https
            ))
            .data(web::JsonConfig::default().limit(4096))
            // everything under '/api/' route
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/invitations").service(invitation::create_invitation),
                    )
                    .service(web::scope("/register").service(register::register_user))
                    .service(web::scope("/login").service(auth::login))
                    .service(web::scope("/logout").service(auth::logout))
                    .service(
                        web::scope("/rituals")
                            .service(rite::create_ritual)
                            .service(rite::list_rituals)
                            .service(rite::delete_ritual)
                            .service(rite::get_ritual)
                            .service(rite::patch_ritual),
                    )
                    .service(
                        web::scope("/moments")
                            .service(moment::create_ritual_moment)
                            .service(moment::list_ritual_moments)
                            .service(moment::delete_ritual_moment),
                    ),
            )
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
