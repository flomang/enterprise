table! {
    orders (id) {
        id -> Uuid,
        user_id -> Uuid,
        price -> Nullable<Numeric>,
        qty -> Nullable<Numeric>,
        typ -> Varchar,
        side -> Varchar,
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
