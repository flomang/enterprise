use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use actix_web::cookie::time::Duration;
use std::sync::Mutex;

use engine::orderbook::Orderbook;
use exchange::engine;

use paper_exchange::routes::orders as paper;
use paper_exchange::AppState;
use paper_exchange::BrokerAsset;
use ritual::utils;
use kitchen::utils::auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var(
        "RUST_LOG",
        "simple-auth-server=debug,actix_web=info,actix_server=info",
    );
    env_logger::init();

    let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    let order_book = Orderbook::new(BrokerAsset::BTC, BrokerAsset::USD);
    let data = web::Data::new(AppState {
        order_book: Mutex::new(order_book),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(auth::cookie_policy(domain, Duration::new(86400, 0)))
            .app_data(data.clone())
            .app_data(web::JsonConfig::default().limit(4096))
            .service(
                web::scope("/api")
                    .service(paper::post_order)
                    .service(paper::patch_order)
                    .service(paper::delete_order),
            )
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}
