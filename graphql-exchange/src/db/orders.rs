use actix::prelude::*;
use diesel::prelude::*;
use libreauth::pass::HashBuilder;

use super::DbExecutor;
//use crate::app::users::{LoginUser, RegisterUser, UpdateUserOuter, UserResponse};
use crate::app::orders::{OrderRequest, OrderResponse};
//use crate::models::{NewUser, User, UserChange};
use crate::models::{NewOrder, Order, OrderChange};
use crate::prelude::*;
use crate::utils::{HASHER, PWD_SCHEME_VERSION};

// message handler implementations ↓

impl Message for OrderRequest {
    type Result = Result<OrderResponse>;
}

impl Handler<OrderRequest> for DbExecutor {
    type Result = Result<OrderResponse>;

    fn handle(&mut self, msg: OrderRequest, _: &mut Self::Context) -> Self::Result {
        use crate::schema::orders::dsl::*;

        let new_order = NewOrder {
            id: msg.id.clone(),
            user_id: msg.user_id.clone(),
            order_asset: msg.order_asset.clone(),
            price_asset: msg.price_asset.clone(),
            price: msg.price.clone(),
            quantity: msg.quantity.clone(),
            order_type: msg.order_type.clone(),
            side: msg.side.clone(),
            status: msg.status.clone(),
            created_at: msg.created_at.clone(),
            updated_at: msg.updated_at.clone(),
        }; 

        let conn = &mut self.0.get()?;

        match diesel::insert_into(orders)
            .values(new_order)
            .get_result::<Order>(conn)
        {
            Ok(o) => Ok(o.into()),
            Err(e) => Err(e.into()),
        }
    }
}