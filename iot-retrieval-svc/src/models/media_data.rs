use chrono::NaiveDateTime;
use serde::Deserialize;
use uuid::Uuid;
use diesel::PgConnection;

//use crate::models::metadata::{Image,Video};
use crate::database::schema::media_datas::dsl::*;
use crate::database::schema::media_datas;
use crate::database::{MediaEnum, LocationEnum};

// NewMediaData has to have Deserialize/Clone to work with bodyparser
// #[derive(Debug, Deserialize, Clone)]
#[derive(Insertable, Debug, Deserialize, Clone)]
#[table_name="media_datas"]
pub struct NewMediaData{
    pub id: Uuid,
    pub name: String,
    pub note: Option<String>,
    pub media_type: MediaEnum,
    pub location: String,
    pub location_type: LocationEnum,
    pub size: i32,
    pub device_id: Uuid
}


//#[derive(GraphQLObject)]
//#[graphql(description = "Media objects for the application")]
#[derive(Queryable, Identifiable, Debug, Eq, PartialEq)]
pub struct MediaData {
    pub id: Uuid,
    pub name: String,
    pub note: Option<String>,
    pub media_type: MediaEnum,
    pub location: String,
    pub location_type: LocationEnum,
    pub device_id: Uuid,
    pub size: i32,
    pub published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


impl MediaData { }

impl NewMediaData {
    // adding the self: &Self to make it a method instead of associated ufction
    // https://doc.rust-lang.org/reference/items/associated-items.html
    pub fn add(self: &Self, conn: &PgConnection) {
        use diesel::insert_into;
        use diesel::RunQueryDsl;

        insert_into(media_datas)
            .values(self)
            //.get_result(&*conn)
            .execute(&*conn)
            .expect("Insertion of new media error");
    }
}