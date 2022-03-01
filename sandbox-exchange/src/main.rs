use actix_web::{error, post, put, delete, web, App, HttpResponse, HttpServer, Responder};
use futures::StreamExt;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

//use std::time::SystemTime;
use exchange::engine;
use engine::domain::OrderSide;
use engine::orderbook::{Orderbook, OrderProcessingResult, Success, Failed};
use engine::orders;

// please keep these organized while editing
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum BrokerAsset {
    ADA,
    BTC,
    DOT,
    ETH,
    EUR,
    GRIN,
    UNI,
    USD,
}

#[derive(Serialize, Deserialize)]
struct LimitOrder {
    order_asset: String,
    price_asset: String,
    side: String,
    price: f64,
    qty: f64,
}

struct AppState {
    order_book: Mutex<Orderbook<BrokerAsset>>,
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

#[post("/order")]
async fn post_order(state: web::Data<AppState>, mut payload: web::Payload) -> impl Responder {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    let obj = serde_json::from_slice::<LimitOrder>(&body)?;

    //let btc_asset = BrokerAsset::BTC;
    //let usd_asset = BrokerAsset::USD;
    //let no =  orders::new_limit_order_request(
    //    btc_asset,
    //    usd_asset,
    //    OrderSide::Bid,
    //    0.98,
    //    5.0,
    //    SystemTime::now());

    //let mut book = state.order_book.lock().unwrap();
    //let res = book.process_order(no);
    //println!("Results => {:?}", res);
    Ok(HttpResponse::Ok().json(obj)) //
}

#[put("/order")]
async fn put_order(state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body("put order")
}

#[delete("/order")]
async fn delete_order(state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body("delete order")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let order_book = Orderbook::new(BrokerAsset::BTC, BrokerAsset::USD);
    let data = web::Data::new(AppState {order_book: Mutex::new(order_book)});

    HttpServer::new( move || {
        App::new()
            .app_data(data.clone())
            .service(post_order)
            .service(put_order)
            .service(delete_order)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// fn main() {
//     let btc_asset = BrokerAsset::BTC;
//     let usd_asset = BrokerAsset::USD;
//     let eth_asset = BrokerAsset::ETH;
//     let btc_market = String::from("BTC-USD");
//     let eth_market = String::from("ETH-USD");

//     let mut markets: HashMap<String, Orderbook<BrokerAsset>> = HashMap::new();
//     markets.insert(btc_market, Orderbook::new(BrokerAsset::BTC, BrokerAsset::USD));
//     markets.insert(eth_market, Orderbook::new(BrokerAsset::ETH, BrokerAsset::USD));

//     let request_list =
//     vec![
//         orders::new_limit_order_request(
//             btc_asset,
//             usd_asset,
//             OrderSide::Bid,
//             0.98,
//             5.0,
//             SystemTime::now()
//         ),

//         orders::new_limit_order_request(
//             btc_asset,
//             usd_asset,
//             OrderSide::Ask,
//             1.02,
//             1.0,
//             SystemTime::now()
//         ),

//         orders::amend_order_request(1, OrderSide::Bid, 0.99, 4.0, SystemTime::now()),

//         orders::new_limit_order_request(
//             btc_asset,
//             usd_asset,
//             OrderSide::Bid,
//             1.01,
//             0.4,
//             SystemTime::now()
//         ),

//         orders::new_limit_order_request(
//             btc_asset,
//             usd_asset,
//             OrderSide::Ask,
//             1.03,
//             0.5,
//             SystemTime::now()
//         ),

//         orders::new_market_order_request(btc_asset, usd_asset, OrderSide::Bid, 0.90, SystemTime::now()),

//         orders::new_limit_order_request(
//             btc_asset,
//             usd_asset,
//             OrderSide::Ask,
//             1.05,
//             0.5,
//             SystemTime::now()
//         ),

//         orders::limit_order_cancel_request(4, OrderSide::Ask),

//         orders::new_limit_order_request(
//             btc_asset,
//             usd_asset,
//             OrderSide::Bid,
//             1.06,
//             0.6,
//             SystemTime::now()
//         ),
//     ];

// let btc_orderbook = markets.get_mut("BTC-USD").unwrap();

// // processing
// for order in request_list {
//     println!("Processing => {:?}", &order);
//     let res = btc_orderbook.process_order(order);
//     println!("Results => {:?}", res);
    
//     if let Some((bid, ask)) = btc_orderbook.current_spread() {
//         println!("Spread => bid: {}, ask: {}\n", bid, ask);
//     } 
// }

//     /* create order requests
//        a client request can be:
//          * new limit order
//          * cancel limit order
//          * amend order 
//          * new market order
//     */
//     let request_list =
//         vec![
//             orders::new_limit_order_request(
//                 btc_asset,
//                 usd_asset,
//                 OrderSide::Bid,
//                 0.98,
//                 5.0,
//                 SystemTime::now()
//             ),

//             orders::new_limit_order_request(
//                 btc_asset,
//                 usd_asset,
//                 OrderSide::Ask,
//                 1.02,
//                 1.0,
//                 SystemTime::now()
//             ),

//             orders::amend_order_request(1, OrderSide::Bid, 0.99, 4.0, SystemTime::now()),

//             orders::new_limit_order_request(
//                 btc_asset,
//                 usd_asset,
//                 OrderSide::Bid,
//                 1.01,
//                 0.4,
//                 SystemTime::now()
//             ),

//             orders::new_limit_order_request(
//                 btc_asset,
//                 usd_asset,
//                 OrderSide::Ask,
//                 1.03,
//                 0.5,
//                 SystemTime::now()
//             ),

//             orders::new_market_order_request(btc_asset, usd_asset, OrderSide::Bid, 0.90, SystemTime::now()),

//             orders::new_limit_order_request(
//                 btc_asset,
//                 usd_asset,
//                 OrderSide::Ask,
//                 1.05,
//                 0.5,
//                 SystemTime::now()
//             ),

//             orders::limit_order_cancel_request(4, OrderSide::Ask),

//             orders::new_limit_order_request(
//                 btc_asset,
//                 usd_asset,
//                 OrderSide::Bid,
//                 1.06,
//                 0.6,
//                 SystemTime::now()
//             ),
//         ];

//     let btc_orderbook = markets.get_mut("BTC-USD").unwrap();

//     // processing
//     for order in request_list {
//         println!("Processing => {:?}", &order);
//         let res = btc_orderbook.process_order(order);
//         println!("Results => {:?}", res);
        
//         if let Some((bid, ask)) = btc_orderbook.current_spread() {
//             println!("Spread => bid: {}, ask: {}\n", bid, ask);
//         } 
//     }
// }
