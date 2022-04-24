use actix_identity::Identity;
use actix_web::{delete, get, patch, post, web, HttpResponse, Result};
use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::prelude::*;
use orderbook::guid::domain::InvalidSideError;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

use crate::models::*;
use crate::schema::fills::dsl as fill_schema;
use crate::schema::orders::dsl as order_schema;
use crate::{AppState, BrokerAsset};
use authentication::models::SlimUser;
use kitchen::utils::errors::ServiceError;
use kitchen::utils::pagination::PageInfo;
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
    let db_results = web::block(move || {
        let conn = pool.get().expect("couldn't get db connection from pool");
        store_results(&results, &conn, &user.id)
    })
    .await?;

    match db_results {
        Ok(_) => Ok(HttpResponse::Ok().json(json)),
        Err(err) => {
            log::error!("DbError: {}", err);
            Err(ServiceError::InternalServerError)
        }
    }
}

fn store_results(
    results: &OrderProcessingResult<BrokerAsset>,
    conn: &PgConnection,
    user_id: &Uuid,
) -> Result<(), DbError> {
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
                        user_id: *user_id,
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
                        .execute(conn)?;
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
                        .execute(conn)?;
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
                        .execute(conn)?;
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
                        .execute(conn)?;
                }
                Success::Cancelled { order_id, ts } => {
                    let timestamp = to_chrono(ts);
                    let order = order_schema::orders.filter(order_schema::id.eq(order_id));

                    diesel::update(order)
                        .set((
                            order_schema::status.eq("cancelled"),
                            order_schema::updated_at.eq(timestamp),
                        ))
                        .execute(conn)?;
                }
            }
        }
    }

    Ok(())
}

#[derive(Serialize)]
struct OrderPage {
    page: i64,
    page_size: i64,
    orders: Vec<Order>,
    total_pages: i64,
}

#[get("/orders")]
pub async fn get_orders(
    params: web::Query<PageInfo>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(str) = id.identity() {
        let result: Result<OrderPage, DbError> = web::block(move || {
            use crate::schema::orders::dsl::*;
            use kitchen::utils::pagination::*;

            let user: SlimUser = serde_json::from_str(&str).unwrap();
            let mut conn = pool.get().expect("couldn't get db connection from pool");
            let result = orders
                .filter(user_id.eq(user.id))
                .filter(status.ne("cancelled".to_string()))
                .order_by(created_at)
                .paginate(params.page)
                .per_page(params.page_size)
                .load_and_count_pages::<Order>(&mut conn)?;

            let (results, total_pages) = result;

            let page = OrderPage {
                page: params.page,
                page_size: params.page_size,
                orders: results,
                total_pages: total_pages,
            };
            Ok(page)
        })
        .await?;

        match result {
            Ok(page) => Ok(HttpResponse::Ok().json(page)),
            Err(err) => {
                log::error!("DbError: {}", err);
                Err(ServiceError::InternalServerError)
            }
        }
    } else {
        Err(ServiceError::Unauthorized)
    }
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
        let order_asset = BrokerAsset::from_string(&req.order_asset)?;
        let price_asset = BrokerAsset::from_string(&req.price_asset)?;
        let side = OrderSide::from_string(&req.side)?;
        let price_opt = req.price;
        let qty_opt: Option<BigDecimal> = FromPrimitive::from_f64(req.qty);

        let mut errors: Vec<String> = vec![];
        if qty_opt.is_none() {
            errors.push("qty must be a decimal".to_string());
        }

        if errors.len() > 0 {
            let value = serde_json::json!(errors);
            return Ok(HttpResponse::Ok().json(value));
        }

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

        process_results(results, pool, user).await
    } else {
        Err(ServiceError::Unauthorized)
    }
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
    if let Some(str) = id.identity() {
        let user: SlimUser = serde_json::from_str(&str).unwrap();
        let side = OrderSide::from_string(&req.side)?;
        let order_id = path.into_inner();

        if let Ok(id) = uuid::Uuid::parse_str(&order_id) {
            let price = FromPrimitive::from_f64(req.price).unwrap();
            let qty = FromPrimitive::from_f64(req.qty).unwrap();
            let order = orders::amend_order_request(id, side, price, qty, SystemTime::now());
            let mut book = state.order_book.lock().unwrap();
            let results = book.process_order(order);
            process_results(results, pool, user).await
        } else {
            Err(ServiceError::BadRequest("invalid order id".to_string()))
        }
    } else {
        Err(ServiceError::Unauthorized)
    }
}

#[delete("/orders/{id}")]
pub async fn delete_order(
    id: Identity,
    path: web::Path<String>,
    state: web::Data<AppState>,
    req: web::Json<CancelOrderRequest>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(str) = id.identity() {
        let user: SlimUser = serde_json::from_str(&str).unwrap();
        let side = OrderSide::from_string(&req.side)?;
        let order_id = path.into_inner();

        if let Ok(id) = uuid::Uuid::parse_str(&order_id) {
            let order = orders::limit_order_cancel_request(id, side);
            let mut book = state.order_book.lock().unwrap();
            let results = book.process_order(order);
            process_results(results, pool, user).await
        } else {
            Err(ServiceError::BadRequest("invalid order id".to_string()))
        }
    } else {
        Err(ServiceError::Unauthorized)
    }
}
