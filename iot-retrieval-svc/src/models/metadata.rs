
use chrono::NaiveDateTime;
use serde::Deserialize;
use uuid::Uuid;
use diesel::{insert_into, RunQueryDsl, PgConnection};
use diesel_geography::types::GeogPoint;

//use crate::database::schema::media_datas::dsl::*;
//use crate::database::schema::image_metadatas::dsl::*;
//use crate::database::PgPooled;
use crate::database::schema::image_metadatas;
use crate::database::schema::video_metadatas;
use crate::errors::Success;
use crate::models::media_data::NewMediaData;

use bigdecimal::BigDecimal;

//use chrono::{Utc, DateTime as DT};
//#[derive(Debug, Deserialize, Clone)]
#[derive(Insertable, Associations, Debug, Deserialize, Clone)]
#[belongs_to(NewMediaData, foreign_key="media_item_id")]
#[table_name="image_metadatas"]
pub struct Image {                     
//    pub id: i32,
    exif_version: Option<BigDecimal>,
    x_pixel_dimension: Option<i32>,
    y_pixel_dimension: Option<i32>,
    x_resolution: Option<i32>,
    y_resolution: Option<i32>,
    // uses RFC3339 out of the box
    //https://serde.rs/custom-date-format.html
    //#[serde(with = "my_date_format")]
    date_of_image: Option<NaiveDateTime>,
    flash: Option<bool>,
    make: Option<String>,
    model: Option<String>,
    exposure_time: Option<String>,
    f_number: Option<String>,
    aperture_value: Option<BigDecimal>,
    location: Option<GeogPoint>,     
    altitude: Option<BigDecimal>,
    speed: Option<BigDecimal>,
    media_item_id: Uuid
}

// #[derive(Debug, Deserialize, Clone)]

//#[derive(Insertable, Queryable, Debug, Deserialize, Clone)]
#[derive(Insertable, Associations, Debug, Deserialize, Clone)]
#[belongs_to(NewMediaData, foreign_key="media_item_id")]
#[table_name="video_metadatas"]
pub struct Video {                 
    video_duration: Option<BigDecimal>,
    video_width: Option<BigDecimal>,
    video_height: Option<BigDecimal>,
    video_codec: Option<String>,
    audio_track_id: Option<BigDecimal>,
    audio_codec: Option<String>,
    media_item_id: Uuid
}

impl Image {
    pub fn save(self: Self, conn: &PgConnection) -> Success {
        use crate::database::schema::image_metadatas::dsl::*;

        insert_into(image_metadatas)
            .values(self)
            //.get_result(&*conn)
            .execute(&*conn)
            .expect("Insertion of new image error");
        Ok(())
    }
}

impl Video {
    pub fn save(self: Self, conn: &PgConnection) {
        use crate::database::schema::video_metadatas::dsl::*;

        insert_into(video_metadatas)
            .values(self)
            //.get_result(&*conn)
            .execute(&*conn)
            .expect("Insertion of new image error");
    }
}