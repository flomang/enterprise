use super::chrono::NaiveDateTime;
use super::schema::*;
use bigdecimal::{BigDecimal, ToPrimitive};
use diesel::pg::data_types::PgNumeric;
use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{
    ser::{Serializer},
    Serialize,
};

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


fn serialize_bigdecimal_opt<S>(bg: &Option<BigDecimal>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match bg {
     Some(b) => serializer.serialize_f64(b.to_f64().unwrap()),
     None => serializer.serialize_none(),
    }
}

fn serialize_bigdecimal<S>(bg: &BigDecimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_f64(bg.to_f64().unwrap())
}

// impl Serialize for Order {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut s = serializer.serialize_struct("Order", 11)?;
//         s.serialize_field("id", &self.id)?;
//         s.serialize_field("user_id", &self.user_id)?;
//         s.serialize_field("order_asset", &self.order_asset)?;
//         s.serialize_field("price_asset", &self.price_asset)?;

//         if let Some(p) = &self.price {
//             s.serialize_field("price",  &p.to_f64().unwrap())?;
//         }

//         s.serialize_field("qty",  &self.quantity.to_f64())?;
//         s.serialize_field("order_type", &self.order_type)?;
//         s.serialize_field("side", &self.side)?;
//         s.serialize_field("status", &self.status)?;
//         s.serialize_field("created_at", &self.created_at)?;
//         s.serialize_field("updated_at", &self.updated_at)?;
//         s.end()
//     }
// }

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
