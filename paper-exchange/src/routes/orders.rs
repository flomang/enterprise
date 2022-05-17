use actix_identity::Identity;
use actix_web::{delete, get, patch, post, web, HttpResponse, Result};
use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

use crate::models::*;
use crate::schema::fills::dsl as fill_schema;
use crate::schema::orders::dsl as order_schema;
use crate::{AppState, BrokerAsset};
use authentication::models::SlimUser;
use library::utils::errors::ServiceError;
use library::utils::pagination::PageInfo;
use orderbook::guid::orderbook::{OrderProcessingResult, Success};
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
    order_id: Uuid,
    side: String,
    price: f64,
    qty: f64,
}

#[derive(Serialize, Deserialize)]
pub struct CancelOrderRequest {
    order_id: Uuid,
    side: String,
}

type DbError = Box<dyn std::error::Error + Send + Sync>;

fn to_chrono(ts: &SystemTime) -> chrono::NaiveDateTime {
    let duration = ts.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    chrono::NaiveDateTime::from_timestamp(duration.as_secs() as i64, 0)
}

async fn process_results(
    results: OrderProcessingResult<BrokerAsset>,
    pool: web::Data<Pool>,
    user: SlimUser,
) -> Result<HttpResponse, ServiceError> {
    let json = serde_json::json!(results);
    let db_results = web::block(move || store_results(results, pool, user.id)).await?;

    match db_results {
        Ok(_) => Ok(HttpResponse::Ok().json(json)),
        Err(err) => {
            log::error!("DbError: {}", err);
            Err(ServiceError::InternalServerError)
        }
    }
}

fn store_results(
    results: OrderProcessingResult<BrokerAsset>,
    pool: web::Data<Pool>,
    user_id: Uuid,
) -> Result<(), DbError> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    for result in results.iter() {
        if let Ok(success) = result {
            match success {
                Success::Accepted {
                    order_id,
                    order_asset,
                    price_asset,
                    price,
                    side,
                    qty,
                    order_type,
                    ts,
                } => {
                    let timestamp = to_chrono(ts);
                    let order = Order {
                        id: *order_id,
                        user_id,
                        order_asset: order_asset.to_string(),
                        price_asset: price_asset.to_string(),
                        price: price.clone(),
                        quantity: qty.clone(),
                        order_type: order_type.to_string(),
                        side: side.to_string(),
                        status: "open".to_string(),
                        created_at: timestamp,
                        updated_at: timestamp,
                    };

                    diesel::insert_into(order_schema::orders)
                        .values(order)
                        .execute(&conn)?;
                }
                Success::Filled {
                    order_id,
                    side,
                    order_type,
                    price,
                    qty,
                    ts,
                } => {
                    let timestamp = to_chrono(ts);

                    let fill = Fill {
                        id: uuid::Uuid::new_v4(),
                        order_id: *order_id,
                        price: price.clone(),
                        quantity: qty.clone(),
                        order_type: order_type.to_string(),
                        side: side.to_string(),
                        created_at: timestamp,
                        updated_at: timestamp,
                    };

                    diesel::insert_into(fill_schema::fills)
                        .values(fill)
                        .execute(&conn)?;
                }
                Success::PartiallyFilled {
                    order_id,
                    side,
                    order_type,
                    price,
                    qty,
                    ts,
                } => {
                    let timestamp = to_chrono(ts);
                    let fill = Fill {
                        id: uuid::Uuid::new_v4(),
                        order_id: *order_id,
                        price: price.clone(),
                        quantity: qty.clone(),
                        order_type: order_type.to_string(),
                        side: side.to_string(),
                        created_at: timestamp,
                        updated_at: timestamp,
                    };

                    diesel::insert_into(fill_schema::fills)
                        .values(fill)
                        .execute(&conn)?;
                }
                Success::Amended {
                    order_id,
                    price: pricee,
                    qty,
                    ts,
                } => {
                    let timestamp = to_chrono(ts);
                    let order = order_schema::orders.filter(order_schema::id.eq(order_id));

                    diesel::update(order)
                        .set((
                            order_schema::price.eq(pricee),
                            order_schema::quantity.eq(qty),
                            order_schema::updated_at.eq(timestamp),
                        ))
                        .execute(&conn)?;
                }
                Success::Cancelled { order_id, ts } => {
                    let timestamp = to_chrono(ts);
                    let order = order_schema::orders.filter(order_schema::id.eq(order_id));

                    diesel::update(order)
                        .set((
                            order_schema::status.eq("cancelled"),
                            order_schema::updated_at.eq(timestamp),
                        ))
                        .execute(&conn)?;
                }
            }
        }
    }

    Ok(())
}

#[derive(Serialize)]
struct OrderPage {
    page: u32,
    page_size: u32,
    orders: Vec<Order>,
    total_pages: i64,
}

#[get("/orders")]
pub async fn get_orders(
    params: web::Query<PageInfo>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let json_str = id.identity().ok_or(ServiceError::Unauthorized)?;
    let user: SlimUser = serde_json::from_str(&json_str).map_err(|err| {
        log::debug!("slim user deserialization: {}", err);
        ServiceError::Unauthorized
    })?;

    let result: Result<OrderPage, DbError> = web::block(move || {
        use crate::schema::orders::dsl::*;
        use library::utils::pagination::*;

        let page: u32 = if params.page > 0 { params.page } else { 1 };
        let page_size: u32 = if params.page_size > 0 {
            params.page_size
        } else {
            1
        };

        let mut conn = pool.get().expect("couldn't get db connection from pool");
        let (results, total_pages) = orders
            .filter(user_id.eq(user.id))
            .filter(status.ne("cancelled".to_string()))
            .order_by(created_at)
            .paginate(page as i64)
            .per_page(page_size as i64)
            .load_and_count_pages::<Order>(&mut conn)?;

        Ok(OrderPage {
            page,
            page_size,
            total_pages,
            orders: results,
        })
    })
    .await?;

    match result {
        Ok(page) => Ok(HttpResponse::Ok().json(page)),
        Err(err) => {
            log::error!("DbError: {}", err);
            Err(ServiceError::InternalServerError)
        }
    }
}

#[post("/orders")]
pub async fn post_order(
    id: Identity,
    state: web::Data<AppState>,
    req: web::Json<OrderRequest>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let json_str = id.identity().ok_or(ServiceError::Unauthorized)?;
    let user: SlimUser = serde_json::from_str(&json_str).map_err(|err| {
        log::debug!("slim user deserialization: {}", err);
        ServiceError::Unauthorized
    })?;
    let order_asset = BrokerAsset::from_string(&req.order_asset)?;
    let price_asset = BrokerAsset::from_string(&req.price_asset)?;
    let side = OrderSide::from_string(&req.side)?;
    let qty: BigDecimal = FromPrimitive::from_f64(req.qty).ok_or(ServiceError::BadRequest(
        "qty cannot be converted to BigDecimal".to_string(),
    ))?;

    let order = match req.price {
        Some(price) => {
            let price: BigDecimal = FromPrimitive::from_f64(price).ok_or(
                ServiceError::BadRequest("price cannot be converted to BigDecimal".to_string()),
            )?;

            orders::new_limit_order_request(
                order_asset,
                price_asset,
                side,
                price,
                qty,
                SystemTime::now(),
            )
        }
        None => {
            orders::new_market_order_request(order_asset, price_asset, side, qty, SystemTime::now())
        }
    };

    let mut book = state.order_book.lock().unwrap();
    let results = book.process_order(order);

    process_results(results, pool, user).await
}

/**
 * Leaving this here for now. In production, users should never be able to change the qty of their original order.
 */
#[patch("/orders/{id}")]
pub async fn patch_order(
    id: Identity,
    path: web::Path<String>,
    state: web::Data<AppState>,
    req: web::Json<AmendOrderRequest>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let json_str = id.identity().ok_or(ServiceError::Unauthorized)?;
    let user: SlimUser = serde_json::from_str(&json_str).map_err(|err| {
        log::debug!("slim user deserialization: {}", err);
        ServiceError::Unauthorized
    })?;

    let side = OrderSide::from_string(&req.side)?;
    let order_id = path.into_inner();
    let id = uuid::Uuid::parse_str(&order_id).or(Err(ServiceError::BadRequest(
        "expected uuid for order id".to_string(),
    )))?;

    let price = FromPrimitive::from_f64(req.price).unwrap();
    let qty = FromPrimitive::from_f64(req.qty).unwrap();
    let order = orders::amend_order_request(id, side, price, qty, SystemTime::now());
    let mut book = state.order_book.lock().unwrap();
    let results = book.process_order(order);
    process_results(results, pool, user).await
}

#[delete("/orders/{id}")]
pub async fn delete_order(
    id: Identity,
    path: web::Path<String>,
    state: web::Data<AppState>,
    req: web::Json<CancelOrderRequest>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let json_str = id.identity().ok_or(ServiceError::Unauthorized)?;
    let user: SlimUser = serde_json::from_str(&json_str).map_err(|err| {
        log::debug!("slim user deserialization: {}", err);
        ServiceError::Unauthorized
    })?;

    let side = OrderSide::from_string(&req.side)?;
    let order_id = path.into_inner();
    let id = uuid::Uuid::parse_str(&order_id).or(Err(ServiceError::BadRequest(
        "expected uuid for order id".to_string(),
    )))?;

    let order = orders::limit_order_cancel_request(id, side);
    let mut book = state.order_book.lock().unwrap();
    let results = book.process_order(order);
    process_results(results, pool, user).await
}
