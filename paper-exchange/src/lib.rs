#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;

use orderbook::guid::orderbook::Orderbook;
use std::sync::Mutex;

pub mod models;
pub mod routes;
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

    pub fn to_string(&self) -> String {
        match self {
            BrokerAsset::ADA => "ADA".to_string(),
            BrokerAsset::BTC => "BTC".to_string(),
            BrokerAsset::DOT => "DOT".to_string(),
            BrokerAsset::ETH => "ETH".to_string(),
            BrokerAsset::GRIN => "GRIN".to_string(),
            BrokerAsset::USD => "USD".to_string(),
        }
    }
}

pub struct AppState {
    pub order_book: Mutex<Orderbook<BrokerAsset>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use bigdecimal::BigDecimal;

    #[test]
    fn insert_dummy_order() {
        use super::schema::orders::dsl::*;
        use diesel::pg::data_types::PgNumeric;
        use diesel::prelude::*;
        use diesel::r2d2::{self, ConnectionManager};
        use std::str::FromStr;

        let pric = "50000.0001";
        let qantity = "0.00005678";

        let bigdecimal_price = BigDecimal::from_str(&pric).unwrap();
        let bigdecimal_quantity = BigDecimal::from_str(&qantity).unwrap();

        let pricee = PgNumeric::from(bigdecimal_price);
        let qtyee = PgNumeric::from(bigdecimal_quantity);

        let database_url = "postgres://bishop@localhost/paper_dev".to_string();

        // create db connection pool
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: models::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let now = chrono::Local::now().naive_local();
        let order = super::models::Order {
            id: uuid::Uuid::new_v4(),
            user_id: uuid::Uuid::from_str("da8cc5a0-bddc-4ee8-a6d2-6e3a92b71600").unwrap(),
            order_asset: "BTC".to_string(),
            price_asset: "USD".to_string(),
            price: Some(pricee),
            quantity: qtyee,
            order_type: "limit".to_string(),
            side: "bid".to_string(),
            status: "open".to_string(),
            created_at: now,
            updated_at: now,
        };

        let conn: &PgConnection = &pool.get().unwrap();
        let result = diesel::insert_into(orders).values(order).execute(conn);
        println!("{:?}", result);
    }
}
