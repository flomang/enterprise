use actix_identity::Identity;
use actix_web::{delete, patch, post, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use bigdecimal::FromPrimitive;

use crate::{AppState, BrokerAsset};
use kitchen::utils::errors::ServiceError;
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
) -> Result<HttpResponse, ServiceError> {
    // access request identity
    if let Some(_id) = id.identity() {
        // access request identity
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
                    FromPrimitive::from_f64(price).unwrap(),
                    FromPrimitive::from_f64(req.qty).unwrap(),
                    SystemTime::now(),
                ))
            }
            (Some(order_asset), Some(price_asset), Some(side), None) => {
                Some(orders::new_market_order_request(
                    order_asset,
                    price_asset,
                    side,
                    FromPrimitive::from_f64(req.qty).unwrap(),
                    SystemTime::now(),
                ))
            }
            _ => None,
        };

        if let Some(o) = order {
            let mut book = state.order_book.lock().unwrap();
            let res = book.process_order(o);
            let value = serde_json::json!(res);
            Ok(HttpResponse::Ok().json(value))
        } else {
            let value = serde_json::json!(errors);
            Ok(HttpResponse::Ok().json(value))
        }
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
