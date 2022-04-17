use super::chrono::NaiveDateTime;
use super::schema::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use diesel::pg::data_types::PgNumeric;


pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

// https://stackoverflow.com/questions/38175300/how-to-insert-a-decimal-number-with-diesels-pgnumeric-type
#[derive(Debug, Insertable, Queryable)]
#[table_name = "orders"]
pub struct Order {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub order_asset: String,
    pub price_asset: String,
    pub price: Option<PgNumeric>,
    pub quantity: PgNumeric,
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
    pub price: Option<PgNumeric>,
    pub quantity: PgNumeric,
    pub status: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Queryable)]
#[table_name = "fills"]
pub struct Fill {
    pub id: uuid::Uuid,
    pub order_id: uuid::Uuid,
    pub price: PgNumeric,
    pub quantity: PgNumeric,
    pub order_type: String,
    pub side: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
