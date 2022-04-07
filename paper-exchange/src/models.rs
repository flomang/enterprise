use super::chrono::NaiveDateTime;
use super::schema::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use diesel::pg::data_types::PgNumeric;


pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Insertable, Queryable)]
#[table_name = "orders"]
pub struct Order {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub price: Option<PgNumeric>,
    pub qty: Option<PgNumeric>,
    pub typ: String,
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
    pub qty: Option<PgNumeric>,
    pub status: String,
    pub updated_at: NaiveDateTime,
}
