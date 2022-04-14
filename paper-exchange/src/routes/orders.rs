use actix_identity::Identity;
use actix_web::{delete, patch, post, web, HttpResponse, Result};
use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use diesel::pg::data_types::PgNumeric;

use crate::models::*;
use crate::{AppState, BrokerAsset};
use authentication::models::SlimUser;
use kitchen::utils::errors::ServiceError;
use orderbook::guid::orderbook::Success;
use orderbook::guid::{domain::OrderSide, orders};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderRequest {
    order_asset: String,
    price_asset: String,
    side: String,
    price: Option<f64>,
    qty: f64,
}

#[derive(Serialize, Deserialize)]
pub struct AmendOrderRequest {
    id: u64,
    side: String,
    price: f64,
    qty: f64,
}

#[derive(Serialize, Deserialize)]
pub struct CancelOrderRequest {
    id: u64,
    side: String,
}

#[post("/orders")]
pub async fn post_order(
    id: Identity,
    state: web::Data<AppState>,
    req: web::Json<OrderRequest>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    // access request identity
    if let Some(str) = id.identity() {
        // access request identity
        let user: SlimUser = serde_json::from_str(&str).unwrap();
        let order_asset_opt = BrokerAsset::from_string(&req.order_asset);
        let price_asset_opt = BrokerAsset::from_string(&req.price_asset);
        let side_opt = OrderSide::from_string(&req.side);
        let price_opt = req.price;
        let qty_opt: Option<BigDecimal> = FromPrimitive::from_f64(req.qty);

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
        if qty_opt.is_none() {
            errors.push("qty must be a decimal".to_string());
        }

        if errors.len() > 0 {
            let value = serde_json::json!(errors);
            return Ok(HttpResponse::Ok().json(value));
        }

        let order_asset = order_asset_opt.unwrap();
        let price_asset = price_asset_opt.unwrap();
        let side = side_opt.unwrap();
        let qty = qty_opt.unwrap();

        let order = match price_opt {
            Some(price) => orders::new_limit_order_request(
                order_asset,
                price_asset,
                side,
                FromPrimitive::from_f64(price).unwrap(),
                FromPrimitive::from_f64(req.qty).unwrap(),
                SystemTime::now(),
            ),
            None => orders::new_market_order_request(
                order_asset,
                price_asset,
                side,
                FromPrimitive::from_f64(req.qty).unwrap(),
                SystemTime::now(),
            ),
        };

        let mut book = state.order_book.lock().unwrap();
        let results = book.process_order(order);

        for result in results.iter() {
            if let Ok(success) = result {
                match success {
                    Success::Accepted { id, order_type, ts } => {
                        let price = match price_opt {
                            Some(pr) => {
                                let bigdec: BigDecimal = FromPrimitive::from_f64(pr).unwrap();
                                let pgnum = PgNumeric::from(bigdec);
                                Some(pgnum)
                            }
                            None => None,
                        };

                        let duration = ts.duration_since(SystemTime::UNIX_EPOCH).unwrap();
                        let timestamp = chrono::NaiveDateTime::from_timestamp(duration.as_secs() as i64, 0);
                        let order = Order {
                            id: *id,
                            user_id: user.id,
                            order_asset: order_asset.to_string(),
                            price_asset: price_asset.to_string(),
                            price,
                            quantity: PgNumeric::from(qty.clone()),
                            order_type: order_type.to_string(),
                            side: side.to_string(),
                            status: "accepted".to_string(),
                            created_at: timestamp,
                            updated_at: timestamp,
                        };

                        let conn: &PgConnection = &pool.get().unwrap();
                        let result = diesel::insert_into(crate::schema::orders::dsl::orders).values(order).execute(conn);
                        println!("{:?}", result);
                    }
                    Success::Filled {
                        order_id: _,
                        side: _,
                        order_type: _,
                        price: _,
                        qty: _,
                        ts: _,
                    } => {
                        println!("todo");
                    }
                    Success::PartiallyFilled {
                        order_id: _,
                        side: _,
                        order_type: _,
                        price: _,
                        qty: _,
                        ts: _,
                    } => {
                        println!("todo");
                    }
                    Success::Amended {
                        id: _,
                        price: _,
                        qty: _,
                        ts: _,
                    } => {
                        println!("todo");
                    }
                    Success::Cancelled { id: _, ts: _ } => {
                        println!("todo");
                    }
                }
            }
        }

        let value = serde_json::json!(results);
        Ok(HttpResponse::Ok().json(value))
    } else {
        Err(ServiceError::Unauthorized)
    }
}

#[patch("/orders/{id}")]
pub async fn patch_order(
    path: web::Path<String>,
    state: web::Data<AppState>,
    req: web::Json<AmendOrderRequest>,
) -> Result<HttpResponse, ServiceError> {
    let side_opt = OrderSide::from_string(&req.side);
    let order_id = path.into_inner();

    if let Ok(id) = uuid::Uuid::parse_str(&order_id) {
        match side_opt {
            Some(side) => {
                let price = FromPrimitive::from_f64(req.price).unwrap();
                let qty = FromPrimitive::from_f64(req.qty).unwrap();
                let order = orders::amend_order_request(id, side, price, qty, SystemTime::now());
                let mut book = state.order_book.lock().unwrap();
                let res = book.process_order(order);
                Ok(HttpResponse::Ok().json(format!("{:?}", res)))
            }
            None => Ok(HttpResponse::Ok().json("side must be 'bid' or 'ask'".to_string())),
        }
    } else {
        Err(ServiceError::BadRequest("invalid order id".to_string()))
    }
}

#[delete("/orders/{id}")]
pub async fn delete_order(
    path: web::Path<String>,
    state: web::Data<AppState>,
    req: web::Json<CancelOrderRequest>,
) -> Result<HttpResponse, ServiceError> {
    let side_opt = OrderSide::from_string(&req.side);

    let order_id = path.into_inner();

    if let Ok(id) = uuid::Uuid::parse_str(&order_id) {
        match side_opt {
            Some(side) => {
                let order = orders::limit_order_cancel_request(id, side);
                let mut book = state.order_book.lock().unwrap();
                let res = book.process_order(order);
                println!("{:?}", res);
                Ok(HttpResponse::Ok().json(format!("{:?}", res)))
            }
            None => Ok(HttpResponse::Ok().json("side must be 'bid' or 'ask'".to_string())),
        }
    } else {
        Err(ServiceError::BadRequest("invalid order id".to_string()))
    }
}
