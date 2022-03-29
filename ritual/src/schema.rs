table! {
    invitations (id) {
        id -> Uuid,
        sender_id -> Uuid,
        recipient_email -> Varchar,
        expires_at -> Timestamp,
    }
}

table! {
    ritual_moments (id) {
        id -> Uuid,
        ritual_id -> Uuid,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

table! {
    rituals (id) {
        id -> Uuid,
        user_id -> Uuid,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        email_verified -> Bool,
        hash -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(invitations -> users (sender_id));
joinable!(ritual_moments -> rituals (ritual_id));
joinable!(rituals -> users (user_id));

allow_tables_to_appear_in_same_query!(
    invitations,
    ritual_moments,
    rituals,
    users,
);
