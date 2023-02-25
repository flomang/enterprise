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
    followers (user_id, follower_id) {
        user_id -> Uuid,
        follower_id -> Uuid,
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

table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        email -> Varchar,
        password -> Text,
        bio -> Nullable<Text>,
        image -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(fills -> orders (order_id));
joinable!(orders -> users (user_id));

allow_tables_to_appear_in_same_query!(
    fills,
    followers,
    orders,
    users,
);
