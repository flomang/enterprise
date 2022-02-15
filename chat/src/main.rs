use std::sync::{
    atomic::{AtomicUsize},
    Arc,
};
use std::env;

use actix::Actor;
use actix_cors::Cors;
use actix_web::{http, web, middleware::Logger, App, HttpServer};
use dotenv::dotenv;

mod asteroid;
mod routes;

use asteroid::server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // log info 
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // App state
    // We are keeping a count of the number of visitors
    let app_state = Arc::new(AtomicUsize::new(0));

    // Start chat server actor
    let server = server::ChatServer::new(app_state.clone()).start();

    // Create Http server with websocket support
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&env::var("CLIENT_HOST").unwrap())
            .allow_any_method()
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .data(app_state.clone())
            .data(server.clone())
            // websocket
            .service(web::resource("/ws/").to(routes::chat_route))
            // routes
            .service(routes::bangs)
            .service(routes::moonbang)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}