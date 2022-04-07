#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;

use engine::orderbook::Orderbook;
use exchange::engine;
use std::sync::Mutex;

pub mod routes;
pub mod models;
pub mod schema;

// please keep these organized while editing
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum BrokerAsset {
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

pub struct AppState {
    pub order_book: Mutex<Orderbook<BrokerAsset>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::domain::OrderSide;
    use engine::orderbook::Orderbook;
    use engine::orders;
    use engine::orders::OrderRequest;
    use exchange::engine;
    use std::collections::HashMap;
    use std::time::SystemTime;

    #[test]
    fn sketch() {
        let btc_asset = BrokerAsset::BTC;
        let usd_asset = BrokerAsset::USD;
        //let eth_asset = BrokerAsset::ETH;
        let btc_market = String::from("BTC-USD");
        let eth_market = String::from("ETH-USD");
        let btc_orderbook = Orderbook::new(BrokerAsset::BTC, BrokerAsset::USD);
        let eth_orderbook = Orderbook::new(BrokerAsset::ETH, BrokerAsset::USD);

        let mut markets: HashMap<String, Orderbook<BrokerAsset>> = HashMap::new();
        markets.insert(btc_market, btc_orderbook);
        markets.insert(eth_market, eth_orderbook);

        let btc_balance = 0.5;
        let usd_balance = 300.00;

        let request_list = vec![
            orders::new_limit_order_request(
                btc_asset,
                usd_asset,
                OrderSide::Bid,
                41711.76,
                0.15,
                SystemTime::now(),
            ),
            orders::new_limit_order_request(
                btc_asset,
                usd_asset,
                OrderSide::Ask,
                41712.6,
                1.0,
                SystemTime::now(),
            ),
            orders::amend_order_request(1, OrderSide::Bid, 40000.00, 0.16, SystemTime::now()),
            orders::new_limit_order_request(
                btc_asset,
                usd_asset,
                OrderSide::Bid,
                1.01,
                0.4,
                SystemTime::now(),
            ),
            orders::new_limit_order_request(
                btc_asset,
                usd_asset,
                OrderSide::Ask,
                1.03,
                0.5,
                SystemTime::now(),
            ),
            orders::new_market_order_request(
                btc_asset,
                usd_asset,
                OrderSide::Bid,
                0.90,
                SystemTime::now(),
            ),
            orders::new_limit_order_request(
                btc_asset,
                usd_asset,
                OrderSide::Ask,
                1.05,
                0.5,
                SystemTime::now(),
            ),
            orders::limit_order_cancel_request(4, OrderSide::Ask),
            orders::new_limit_order_request(
                btc_asset,
                usd_asset,
                OrderSide::Bid,
                1.06,
                0.6,
                SystemTime::now(),
            ),
        ];
        let btc_orderbook = markets.get_mut("BTC-USD").unwrap();

        // processing
        for order in request_list {
            let valid = match order {
                OrderRequest::NewLimitOrder {
                    order_asset: _,
                    price_asset,
                    side,
                    price,
                    qty,
                    ts: _,
                } => {
                    //let total = price * qty;
                    // if side == OrderSide::Bid {
                    //     println!("total cost: {} {:?}", total, price_asset)
                    // } else {
                    //     println!("total sale: {} {:?}", total, price_asset)
                    // }
                    true
                }
                OrderRequest::NewMarketOrder {
                    order_asset: _,
                    price_asset: _,
                    side: _,
                    qty: _,
                    ts: _,
                } => {
                    //println!("new market order");
                    true
                }
                OrderRequest::AmendOrder {
                    id: _,
                    side: _,
                    price: _,
                    qty: _,
                    ts: _,
                } => {
                    //println!("amend order");
                    true
                }
                OrderRequest::CancelOrder { id: _, side: _ } => {
                    //println!("cancel order");
                    true
                }
            };

            if valid {
                println!("Processing => {:?}", &order);
                let results = btc_orderbook.process_order(order);

                for result in results {
                    println!("\tResult => {:?}", result);
                }

                if let Some((bid, ask)) = btc_orderbook.current_spread() {
                    println!("Spread => bid: {}, ask: {}\n", bid, ask);
                }
            }
        }

        assert_eq!(true, true);
    }
}
