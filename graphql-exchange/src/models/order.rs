use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::orders;
use crate::utils::bigdecimal::{serialize_bigdecimal, serialize_bigdecimal_opt};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Cancelled,
    Filled,
    PartiallyFilled,
    Opened,
}

#[derive(Debug, Insertable, Queryable, Serialize)]
#[diesel(table_name = orders)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
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
#[diesel(table_name = orders)]
pub struct UpdateOrder {
    pub id: uuid::Uuid,
    pub price: Option<BigDecimal>,
    pub quantity: BigDecimal,
    pub status: String,
    pub updated_at: NaiveDateTime,
}

// implement OutputType for diesel models
#[async_graphql::Object]
impl Order {
    async fn id(&self) -> String {
        self.id.to_string()
    }

    async fn use_id(&self) -> String {
        self.user_id.to_string()
    }

    async fn price(&self) -> Option<String> {
        self.price.clone().map(|price| price.to_string())
    }

    async fn quantity(&self) -> String {
        self.quantity.to_string()
    }

    async fn created_at(&self) -> String {
        self.created_at.to_string()
    }

    async fn updated_at(&self) -> String {
        self.updated_at.to_string()
    }
}
