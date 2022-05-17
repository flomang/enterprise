use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Instant,
};

use actix::*;
use actix_files::{Files, NamedFile};
use actix_identity::Identity;
use actix_web::{
    cookie::time::Duration, middleware::Logger, web, App, Error, HttpRequest, HttpResponse,
    HttpServer, Responder,
};
use actix_web_actors::ws;
use authentication::models::SlimUser;
use library::utils;
use library::utils::errors::ServiceError;

mod server;
mod session;

async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

/// Entry point for our websocket route
async fn chat_route(
    id: Identity,
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    let json_str = id.identity().ok_or(ServiceError::Unauthorized)?;
    let user: SlimUser = serde_json::from_str(&json_str).map_err(|err| {
        log::debug!("slim user deserialization: {}", err);
        ServiceError::Unauthorized
    })?;

    ws::start(
        session::WsChatSession {
            id: 0,
            user,
            hb: Instant::now(),
            room: "Main".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

/// Displays state
async fn get_count(count: web::Data<AtomicUsize>) -> impl Responder {
    let current_count = count.load(Ordering::SeqCst);
    format!("Visitors: {}", current_count)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    //env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8080");
    HttpServer::new(move || {
        let app_state = Arc::new(AtomicUsize::new(0));
        let server = server::ChatServer::new(app_state.clone()).start();
        let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

        App::new()
            .wrap(utils::auth::cookie_policy(domain, Duration::new(86400, 0)))
            .wrap(Logger::default())
            .app_data(web::Data::from(app_state))
            .app_data(web::Data::new(server))
            .service(web::resource("/").to(index))
            .route("/count", web::get().to(get_count))
            .route("/ws", web::get().to(chat_route))
            .service(Files::new("/static", "./static"))
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
