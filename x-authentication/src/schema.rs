table! {
    invitations (id) {
        id -> Uuid,
        sender_id -> Uuid,
        recipient_email -> Varchar,
        expires_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        email_verified -> Bool,
        username -> Varchar,
        avatar_url -> Nullable<Varchar>,
        hash -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(invitations -> users (sender_id));

allow_tables_to_appear_in_same_query!(
    invitations,
    users,
);
