use super::chrono::NaiveDateTime;
use super::schema::*;
use bigdecimal::BigDecimal;
use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::Serialize;
use kitchen::utils::{serialize_bigdecimal, serialize_bigdecimal_opt};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Insertable, Queryable, Serialize)]
#[table_name = "orders"]
pub struct Order {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub order_asset: String,
    pub price_asset: String,
    #[serde(serialize_with = "serialize_bigdecimal_opt")]
    pub price: Option<BigDecimal>,
    #[serde(serialize_with = "serialize_bigdecimal")]
    pub quantity: BigDecimal,
    pub order_type: String,
    pub side: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[table_name = "orders"]
pub struct UpdateOrder {
    pub id: uuid::Uuid,
    pub price: Option<BigDecimal>,
    pub quantity: BigDecimal,
    pub status: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Queryable)]
#[table_name = "fills"]
pub struct Fill {
    pub id: uuid::Uuid,
    pub order_id: uuid::Uuid,
    pub price: BigDecimal,
    pub quantity: BigDecimal,
    pub order_type: String,
    pub side: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
