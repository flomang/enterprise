use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

use exchange::engine;
use engine::orderbook::Orderbook;

use sandbox_exchange::BrokerAsset;
use sandbox_exchange::routes::orders as paper;
use sandbox_exchange::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let order_book = Orderbook::new(BrokerAsset::BTC, BrokerAsset::USD);
    let data = web::Data::new(AppState {
        order_book: Mutex::new(order_book),
    });

    HttpServer::new(move || {
        App::new().app_data(data.clone()).service(
            web::scope("/api")
                .service(paper::post_order)
                .service(paper::patch_order)
                .service(paper::delete_order),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
