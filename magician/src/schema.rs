table! {
    invitations (id) {
        id -> Uuid,
        email -> Varchar,
        expires_at -> Timestamp,
    }
}

table! {
    ritual_times (id) {
        id -> Uuid,
        ritual_id -> Uuid,
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
        hash -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(ritual_times -> rituals (ritual_id));
joinable!(rituals -> users (user_id));

allow_tables_to_appear_in_same_query!(
    invitations,
    ritual_times,
    rituals,
    users,
);
