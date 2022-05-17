
use std::fmt::Debug;
use library::utils::errors::ServiceError;
use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;
use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Bid,
    Ask,
}

#[derive(Debug)]
pub struct InvalidSideError;
impl From<InvalidSideError> for ServiceError {
    fn from(_: InvalidSideError) -> ServiceError {
        ServiceError::BadRequest("side must be 'ask' or 'bid'".to_string())
    }
}

impl OrderSide {
    pub fn from_string(side: &str) -> Result<OrderSide, InvalidSideError> {
        let lower = side.to_lowercase();
        match lower.as_str() {
            "bid" => Ok(OrderSide::Bid),
            "ask" => Ok(OrderSide::Ask),
            _ => Err(InvalidSideError),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            OrderSide::Bid => "bid".to_string(),
            OrderSide::Ask => "ask".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Order<Asset>
where
    Asset: Debug + Clone,
{
    pub order_id: Uuid,
    pub order_asset: Asset,
    pub price_asset: Asset,
    pub side: OrderSide,
    pub price: BigDecimal,
    pub qty: BigDecimal,
}


#[derive(Eq, PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Market,
    Limit,
}

impl OrderType {
    pub fn to_string(&self) -> String {
        match self {
            OrderType::Market => "market".to_string(),
            OrderType::Limit => "limit".to_string(),
        }
    }
}
