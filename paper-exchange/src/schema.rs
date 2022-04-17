table! {
    fills (id) {
        id -> Uuid,
        order_id -> Uuid,
        price -> Numeric,
        quantity -> Numeric,
        order_type -> Varchar,
        side -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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

joinable!(fills -> orders (order_id));

allow_tables_to_appear_in_same_query!(
    fills,
    orders,
);
