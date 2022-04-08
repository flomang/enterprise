table! {
    orders (id) {
        id -> Uuid,
        user_id -> Uuid,
        order_asset -> Varchar,
        price_asset -> Varchar,
        price -> Nullable<Numeric>,
        quantity -> Numeric,
        order_type -> Varchar,
        side -> Varchar,
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
