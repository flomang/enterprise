table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Uuid,
        role_id -> Int4,
        username -> Text,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        email_verified -> Bool,
        hash -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(users -> roles (role_id));

allow_tables_to_appear_in_same_query!(
    roles,
    users,
);
