
mod engine;

use std::collections::HashMap;
use std::time::SystemTime;
//use engine::orderbook::{Orderbook, OrderSide, orders};
pub use engine::domain::OrderSide;
pub use engine::orderbook::{Orderbook, OrderProcessingResult, Success, Failed};
pub use engine::orders;
//use orders::OrderRequest;


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


// fn parse_asset(asset: &str) -> Option<BrokerAsset> {
//     // please keep these organized while editing
//     match asset {
//         // sorted alpha
//         "ADA" => Some(BrokerAsset::ADA),
//         "BTC" => Some(BrokerAsset::BTC),
//         "DOT" => Some(BrokerAsset::DOT),
//         "ETH" => Some(BrokerAsset::ETH),
//         "EUR" => Some(BrokerAsset::EUR),
//         "GRIN" => Some(BrokerAsset::GRIN),
//         "UNI" => Some(BrokerAsset::UNI),
//         "USD" => Some(BrokerAsset::USD),
//         _ => None,
//     }
// }


fn main() {
    let btc_asset = BrokerAsset::BTC;
    let usd_asset = BrokerAsset::USD;
    //let eth_asset = BrokerAsset::ETH;

    let mut markets: HashMap<String, Orderbook<BrokerAsset>> = HashMap::new();
    markets.insert(String::from("BTC-USD"), Orderbook::new(BrokerAsset::BTC, BrokerAsset::USD));
    markets.insert(String::from("ETH-USD"), Orderbook::new(BrokerAsset::ETH, BrokerAsset::USD));
    markets.insert(String::from("ETH-BTC"), Orderbook::new(BrokerAsset::ETH, BrokerAsset::BTC));


    /* create order requests
       a client request can be:
         * new limit order
         * cancel limit order
         * amend order 
         * new market order
    */
    let request_list =
        vec![
            orders::new_limit_order_request(
                btc_asset,
                usd_asset,
                OrderSide::Bid,
                0.98,
                5.0,
                SystemTime::now()
            ),

            orders::new_limit_order_request(
                btc_asset,
                usd_asset,
                OrderSide::Ask,
                1.02,
                1.0,
                SystemTime::now()
            ),

            orders::amend_order_request(1, OrderSide::Bid, 0.99, 4.0, SystemTime::now()),

            orders::new_limit_order_request(
                btc_asset,
                usd_asset,
                OrderSide::Bid,
                1.01,
                0.4,
                SystemTime::now()
            ),

            orders::new_limit_order_request(
                btc_asset,
                usd_asset,
                OrderSide::Ask,
                1.03,
                0.5,
                SystemTime::now()
            ),

            orders::new_market_order_request(btc_asset, usd_asset, OrderSide::Bid, 0.90, SystemTime::now()),

            orders::new_limit_order_request(
                btc_asset,
                usd_asset,
                OrderSide::Ask,
                1.05,
                0.5,
                SystemTime::now()
            ),

            orders::limit_order_cancel_request(4, OrderSide::Ask),

            orders::new_limit_order_request(
                btc_asset,
                usd_asset,
                OrderSide::Bid,
                1.06,
                0.6,
                SystemTime::now()
            ),
        ];

    // processing
    for order in request_list {
        //match order {
        //    OrderRequest::NewMarketOrder {
        //        order_asset,
        //        price_asset,
        //        side,
        //        qty,
        //        ts: _ts,
        //    } => {
        //        let market = format!("{:?}-{:?}", order_asset, price_asset);

        //        println!("NEW market order: {}, {:?}, {}", market, side, qty)
        //    }

        //    OrderRequest::NewLimitOrder {
        //        order_asset,
        //        price_asset,
        //        side: _side,
        //        price: _price,
        //        qty: _qty,
        //        ts: _ts,
        //    } => {
        //        let market = format!("{:?}-{:?}", order_asset, price_asset);

        //        println!("New limit order: {}", market)
        //    }

        //    OrderRequest::AmendOrder {
        //        id: _id,
        //        side: _side,
        //        price: _price,
        //        qty: _qty,
        //        ts: _ts,
        //    } => {
        //        println!("Amend order")
        //    }

        //    OrderRequest::CancelOrder { 
        //        id: _id, 
        //        side: _side 
        //    } => {
        //        println!("Cancel order")
        //    }
        //}
        println!("Order => {:?}", &order);

        let btc_orderbook = markets.get_mut("BTC-USD").unwrap();

        let res = btc_orderbook.process_order(order);
        println!("Processing => {:?}\n", res);

        if let Some((bid, ask)) = btc_orderbook.current_spread() {
            println!("Spread => bid: {}, ask: {}\n", bid, ask);
        } 
    }
}
