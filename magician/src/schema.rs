table! {
    invitations (id) {
        id -> Uuid,
        email -> Varchar,
        expires_at -> Timestamp,
    }
}

table! {
    ritual_times (id) {
        id -> Int4,
        ritual_id -> Int4,
        created_on -> Timestamp,
    }
}

table! {
    rituals (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        created_on -> Timestamp,
        updated_on -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        hash -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(ritual_times -> rituals (ritual_id));

allow_tables_to_appear_in_same_query!(
    invitations,
    ritual_times,
    rituals,
    users,
);
