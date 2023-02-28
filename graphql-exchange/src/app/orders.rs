use crate::models::Order;
use validator::Validate;

#[derive(async_graphql::InputObject, Debug, Validate, Deserialize)]
pub struct OrderRequest {
    #[validate(length(
        min = 3,
        max = 7,
        message = "fails validation - must be 3-7 characters long"
    ))]
    order_asset: String,
    #[validate(length(
        min = 3,
        max = 7,
        message = "fails validation - must be 3-7 characters long"
    ))]
    price_asset: String,
    side: String,
    price: Option<f64>,
    qty: f64,
}

#[derive(async_graphql::SimpleObject, Debug, Serialize)]
pub struct OrderResponse {
    pub order: Order,
}

#[derive(async_graphql::SimpleObject, Debug, Serialize)]
pub struct UserResponseInner {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl From<Order> for OrderResponse {
    fn from(order: Order) -> Self {
        OrderResponse { order }
    }
}
