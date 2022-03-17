use actix_web::{delete, patch, post, web, App, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use engine::domain::OrderSide;
use engine::orderbook::Orderbook;
use engine::orders;
use exchange::engine;
use std::time::SystemTime;

// please keep these organized while editing
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum BrokerAsset {
    ADA,
    BTC,
    DOT,
    ETH,
    GRIN,
    USD,
}

impl BrokerAsset {
    pub fn from_string(asset: &str) -> Option<BrokerAsset> {
        let upper = asset.to_uppercase();
        match upper.as_str() {
            "ADA" => Some(BrokerAsset::ADA),
            "BTC" => Some(BrokerAsset::BTC),
            "DOT" => Some(BrokerAsset::DOT),
            "ETH" => Some(BrokerAsset::ETH),
            "GRIN" => Some(BrokerAsset::GRIN),
            "USD" => Some(BrokerAsset::USD),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct OrderRequest {
    order_asset: String,
    price_asset: String,
    side: String,
    price: Option<f64>,
    qty: f64,
}

#[derive(Serialize, Deserialize)]
struct AmendOrderRequest {
    id: u64,
    side: String,
    price: f64,
    qty: f64,
}

#[derive(Serialize, Deserialize)]
struct CancelOrderRequest {
    id: u64,
    side: String,
}

struct AppState {
    order_book: Mutex<Orderbook<BrokerAsset>>,
}

#[post("/orders")]
async fn post_order(
    state: web::Data<AppState>,
    req: web::Json<OrderRequest>,
) -> Result<impl Responder> {
    let order_asset_opt = BrokerAsset::from_string(&req.order_asset);
    let price_asset_opt = BrokerAsset::from_string(&req.price_asset);
    let side_opt = OrderSide::from_string(&req.side);
    let price_opt = req.price;

    let mut errors: Vec<String> = vec![];
    if order_asset_opt.is_none() {
        errors.push("bad order asset".to_string());
    }
    if price_asset_opt.is_none() {
        errors.push("bad price asset".to_string());
    }
    if side_opt.is_none() {
        errors.push("side must be bid or ask".to_string());
    }

    let order = match (order_asset_opt, price_asset_opt, side_opt, price_opt) {
        (Some(order_asset), Some(price_asset), Some(side), Some(price)) => {
            Some(orders::new_limit_order_request(
                order_asset,
                price_asset,
                side,
                price,
                req.qty,
                SystemTime::now(),
            ))
        }
        (Some(order_asset), Some(price_asset), Some(side), None) => {
            Some(orders::new_market_order_request(
                order_asset,
                price_asset,
                side,
                req.qty,
                SystemTime::now(),
            ))
        }
        _ => None,
    };

    if let Some(o) = order {
        let mut book = state.order_book.lock().unwrap();
        let res = book.process_order(o);
        let value = serde_json::json!(res);
        Ok(web::Json(value))
    } else {
        let value = serde_json::json!(errors);
        Ok(web::Json(value))
    }
}

#[patch("/orders/{id}")]
async fn patch_order(
    path: web::Path<u64>,
    state: web::Data<AppState>,
    req: web::Json<AmendOrderRequest>,
) -> Result<impl Responder> {
    let side_opt = OrderSide::from_string(&req.side);
    let id = path.into_inner();

    match side_opt {
        Some(side) => {
            let order =
                orders::amend_order_request(id, side, req.price, req.qty, SystemTime::now());
            let mut book = state.order_book.lock().unwrap();
            let res = book.process_order(order);
            Ok(web::Json(format!("{:?}", res)))
        }
        None => Ok(web::Json("side must be 'bid' or 'ask'".to_string())),
    }
}

#[delete("/orders/{id}")]
async fn delete_order(
    path: web::Path<u64>,
    state: web::Data<AppState>,
    req: web::Json<CancelOrderRequest>,
) -> Result<impl Responder> {
    let side_opt = OrderSide::from_string(&req.side);
    let id = path.into_inner();

    match side_opt {
        Some(side) => {
            let order = orders::limit_order_cancel_request(id, side);
            let mut book = state.order_book.lock().unwrap();
            let res = book.process_order(order);
            println!("{:?}", res);
            Ok(web::Json("what now".to_string()))
        }
        None => Ok(web::Json("side must be 'bid' or 'ask'".to_string())),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let order_book = Orderbook::new(BrokerAsset::BTC, BrokerAsset::USD);
    let data = web::Data::new(AppState {
        order_book: Mutex::new(order_book),
    });

    HttpServer::new(move || {
        App::new().app_data(data.clone()).service(
            web::scope("/api")
                .service(post_order)
                .service(patch_order)
                .service(delete_order),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
