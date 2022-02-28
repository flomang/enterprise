
use exchange::engine;

use std::collections::HashMap;
use std::time::SystemTime;
pub use engine::domain::OrderSide;
pub use engine::orderbook::{Orderbook, OrderProcessingResult, Success, Failed};
pub use engine::orders;

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
    let btc_market = String::from("BTC-USD");
    let eth_market = String::from("ETH-USD");

    let mut markets: HashMap<String, Orderbook<BrokerAsset>> = HashMap::new();
    markets.insert(btc_market, Orderbook::new(BrokerAsset::BTC, BrokerAsset::USD));
    markets.insert(eth_market, Orderbook::new(BrokerAsset::ETH, BrokerAsset::USD));


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

    let btc_orderbook = markets.get_mut("BTC-USD").unwrap();

    // processing
    for order in request_list {
        println!("Processing => {:?}", &order);
        let res = btc_orderbook.process_order(order);
        println!("Results => {:?}", res);

        if let Some((bid, ask)) = btc_orderbook.current_spread() {
            println!("Spread => bid: {}, ask: {}\n", bid, ask);
        } 
    }
}
