#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;

use diesel::prelude::*;
use kitchen::utils::errors::ServiceError;
use orderbook::guid::{
    domain::OrderSide,
    orderbook::Orderbook,
    orders::{new_limit_order_request, new_market_order_request, OrderRequest},
};
use serde::Serialize;
use std::fmt;
use std::sync::Mutex;
use std::time::SystemTime;

use crate::models::Order;
use crate::models::Pool;

pub mod models;
pub mod routes;
pub mod schema;

#[derive(Debug)]
pub struct AssetError {
    msg: String,
}
impl fmt::Display for AssetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

// please keep these organized while editing
#[derive(PartialEq, Eq, Debug, Copy, Clone, Serialize)]
pub enum BrokerAsset {
    ADA,
    BTC,
    DOT,
    ETH,
    GRIN,
    USD,
}

impl BrokerAsset {
    // pub fn from_string(asset: &str) -> Option<BrokerAsset> {
    //     let upper = asset.to_uppercase();
    //     match upper.as_str() {
    //         "ADA" => Some(BrokerAsset::ADA),
    //         "BTC" => Some(BrokerAsset::BTC),
    //         "DOT" => Some(BrokerAsset::DOT),
    //         "ETH" => Some(BrokerAsset::ETH),
    //         "GRIN" => Some(BrokerAsset::GRIN),
    //         "USD" => Some(BrokerAsset::USD),
    //         _ => None,
    //     }
    // }

    pub fn from_string(asset: &str) -> Result<BrokerAsset, AssetError> {
        let upper = asset.to_uppercase();
        match upper.as_str() {
            "ADA" => Ok(BrokerAsset::ADA),
            "BTC" => Ok(BrokerAsset::BTC),
            "DOT" => Ok(BrokerAsset::DOT),
            "ETH" => Ok(BrokerAsset::ETH),
            "GRIN" => Ok(BrokerAsset::GRIN),
            "USD" => Ok(BrokerAsset::USD),
            _ => Err(AssetError {
                msg: format!("invalid asset: {}", asset),
            }),
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

impl From<AssetError> for ServiceError {
    fn from(asset: AssetError) -> ServiceError {
        ServiceError::BadRequest(asset.msg)
    }
}


pub struct AppState {
    pub order_book: Mutex<Orderbook<BrokerAsset>>,
}

pub fn database_orders(pool: Pool) -> Vec<OrderRequest<BrokerAsset>> {
    use crate::schema::orders::dsl::*;
    use kitchen::utils::pagination::*;

    let mut order_requests = vec![];
    let mut page = 1;
    let mut total_pages = 1;
    let page_size = 1000;
    let mut conn = pool.get().unwrap();

    loop {
        let results = orders
            .filter(status.ne("cancelled".to_string()))
            .order_by(created_at)
            .paginate(page)
            .per_page(page_size)
            .load_and_count_pages::<Order>(&mut conn);

        if let Ok((ords, total)) = results {
            total_pages = total;

            for o in ords.iter() {
                let request = match o.order_type.as_str() {
                    "limit" => new_limit_order_request(
                        BrokerAsset::from_string(&o.order_asset).expect("invalid order asset"),
                        BrokerAsset::from_string(&o.price_asset).expect("invalid price asset"),
                        OrderSide::from_string(&o.side).expect("this should be a valid side"),
                        o.price.clone().unwrap(),
                        o.quantity.clone(),
                        SystemTime::now(),
                    ),
                    _ => new_market_order_request(
                        BrokerAsset::from_string(&o.order_asset).expect("invalid order asset"),
                        BrokerAsset::from_string(&o.price_asset).expect("invalid price asset"),
                        OrderSide::from_string(&o.side).unwrap(),
                        o.quantity.clone(),
                        SystemTime::now(),
                    ),
                };
                order_requests.push(request);
            }
        }

        if page >= total_pages {
            break;
        }
        page += 1;
    }

    order_requests
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
