table! {
    comments (id) {
        id -> Int4,
        body -> Text,
        media_item_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    media_datas (id) {
        id -> Uuid,
        name -> Varchar,
        note -> Nullable<Varchar>,
        size -> Int4,
        published -> Bool,
        location -> Varchar,
        device_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        media_type -> Nullable<Media_enum>,
        location_type -> Location_enum,
        media_audience_type -> Nullable<Array<Media_audience_enum>>,
    }
}

joinable!(comments -> media_datas (media_item_id));

allow_tables_to_appear_in_same_query!(
    comments,
    media_datas,
);
