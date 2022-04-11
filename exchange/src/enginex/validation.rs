
use std::fmt::Debug;
use bigdecimal::{BigDecimal, Zero};
use uuid::Uuid;

use super::orders::OrderRequest;


/// Validation errors
const ERR_BAD_ORDER_ASSET: &str = "bad order asset";
const ERR_BAD_PRICE_ASSET: &str = "bad price asset";
const ERR_BAD_PRICE_VALUE: &str = "price must be non-negative";
const ERR_BAD_QUANTITY_VALUE: &str = "quantity must be non-negative";
const ERR_BAD_SEQ_ID: &str = "order ID out of range";


/* Validators */

pub struct OrderRequestValidator<Asset> {
    orderbook_order_asset: Asset,
    orderbook_price_asset: Asset,
    //min_sequence_id: u64,
    //max_sequence_id: u64,
}

impl<Asset> OrderRequestValidator<Asset>
where
    Asset: Debug + Clone + Copy + Eq,
{
    pub fn new(
        orderbook_order_asset: Asset,
        orderbook_price_asset: Asset,
        //min_sequence_id: u64,
        //max_sequence_id: u64,
    ) -> Self {
        OrderRequestValidator {
            orderbook_order_asset,
            orderbook_price_asset,
            //min_sequence_id,
            //max_sequence_id,
        }
    }


    pub fn validate(&self, request: &OrderRequest<Asset>) -> Result<(), &str> {
        match request {
            OrderRequest::NewMarketOrder {
                order_asset,
                price_asset,
                side: _side,
                qty,
                ts: _ts,
            } => self.validate_market(*order_asset, *price_asset, qty.clone()),

            OrderRequest::NewLimitOrder {
                order_asset,
                price_asset,
                side: _side,
                price,
                qty,
                ts: _ts,
            } => self.validate_limit(*order_asset, *price_asset, price.clone(), qty.clone()),

            OrderRequest::AmendOrder {
                id,
                price,
                side: _side,
                qty,
                ts: _ts,
            } => self.validate_amend(*id, price.clone(), qty.clone()),

            OrderRequest::CancelOrder { id, side: _side } => self.validate_cancel(*id),
        }
    }

    /* Internal validators */

    fn validate_market(
        &self,
        order_asset: Asset,
        price_asset: Asset,
        qty: BigDecimal,
    ) -> Result<(), &str> {

        if self.orderbook_order_asset != order_asset {
            return Err(ERR_BAD_ORDER_ASSET);
        }

        if self.orderbook_price_asset != price_asset {
            return Err(ERR_BAD_PRICE_ASSET);
        }

        if qty <= BigDecimal::zero() {
            return Err(ERR_BAD_QUANTITY_VALUE);
        }

        Ok(())
    }


    fn validate_limit(
        &self,
        order_asset: Asset,
        price_asset: Asset,
        price: BigDecimal,
        qty: BigDecimal,
    ) -> Result<(), &str> {

        if self.orderbook_order_asset != order_asset {
            return Err(ERR_BAD_ORDER_ASSET);
        }

        if self.orderbook_price_asset != price_asset {
            return Err(ERR_BAD_PRICE_ASSET);
        }

        if price <= BigDecimal::zero() {
            return Err(ERR_BAD_PRICE_VALUE);
        }

        if qty <= BigDecimal::zero() {
            return Err(ERR_BAD_QUANTITY_VALUE);
        }

        Ok(())
    }


    fn validate_amend(&self, id: Uuid, price: BigDecimal, qty: BigDecimal) -> Result<(), &str> {
        //if self.min_sequence_id > id || self.max_sequence_id < id {
        //    return Err(ERR_BAD_SEQ_ID);
        //}
        if id == Uuid::nil() {
            return Err(ERR_BAD_SEQ_ID);
        }

        if price <= BigDecimal::zero() {
            return Err(ERR_BAD_PRICE_VALUE);
        }

        if qty <= BigDecimal::zero() {
            return Err(ERR_BAD_QUANTITY_VALUE);
        }

        Ok(())
    }


    fn validate_cancel(&self, id: Uuid) -> Result<(), &str> {
        // if self.min_sequence_id > id || self.max_sequence_id < id {
        //     return Err(ERR_BAD_SEQ_ID);
        // }
        if id == Uuid::nil() {
            return Err(ERR_BAD_SEQ_ID);
        }

        Ok(())
    }
}
