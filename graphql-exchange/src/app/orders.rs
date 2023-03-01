use crate::models::Order;
use validator::Validate;

#[derive(async_graphql::InputObject, Debug, Validate, Deserialize)]
pub struct OrderRequest {
    #[validate(length(
        min = 3,
        max = 7,
        message = "fails validation - must be 3-7 characters long"
    ))]
    pub order_asset: String,
    #[validate(length(
        min = 3,
        max = 7,
        message = "fails validation - must be 3-7 characters long"
    ))]
    pub price_asset: String,
    pub side: String,
    pub price: Option<f64>,
    pub qty: f64,
}

#[derive(async_graphql::SimpleObject, Debug, Serialize)]
pub struct OrderResponse {
    pub order: Order,
}

impl From<Order> for OrderResponse {
    fn from(order: Order) -> Self {
        OrderResponse { order }
    }
}
