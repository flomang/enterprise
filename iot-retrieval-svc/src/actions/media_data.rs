use serde::Deserialize;
use uuid::Uuid;
use diesel::PgConnection;

use crate::database::{MediaEnum, LocationEnum};
use crate::models::metadata::{Image,Video};
use crate::models::media_data::{MediaData, NewMediaData};

#[derive(Deserialize, Debug, Clone)]
pub struct MediaDataAdd {
    pub id: Uuid,
    pub name: String,
    pub media_type: MediaEnum,
    pub location: String,
    pub location_type: LocationEnum,
    pub size: i32,
    pub device_id: Uuid,
    pub image_data: Option<Image>, 
    pub video_data: Option<Video>  
}

impl MediaDataAdd {
    // Cant do a reference for &self because it will move out of borrowed context error
    pub fn save(self: Self, pool: &PgConnection) {
        // save the image / video or save the data
        let media_data = NewMediaData {
            id: self.id,
            name: self.name,
            note: None,
            location: self.location,
            size: self.size,
            device_id: self.device_id,
            media_type: self.media_type,
            location_type: self.location_type
        };
        media_data.add(pool);
        // now save either the video or image
        self.image_data.map(| image | image.save(&pool));
        self.video_data.map(| video | video.save(&pool));
    }
}
