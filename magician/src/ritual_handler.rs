use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use chrono::prelude::Utc;
use diesel::prelude::*;
use diesel::PgConnection;
use serde::Deserialize;

use crate::errors::ServiceError;
use crate::models::{Pool, Ritual, SlimUser};

#[derive(Debug, Deserialize)]
pub struct RitualData {
    pub title: String,
    pub body: String,
}

pub async fn create_ritual(
    ritual_data: web::Json<RitualData>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    use crate::schema::rituals;

    // access request identity
    if let Some(str) = id.identity() {
        let data = ritual_data.into_inner();
        let user: SlimUser = serde_json::from_str(&str).unwrap();
        let conn: &PgConnection = &pool.get().unwrap();
        let now = Utc::now().naive_utc();
        let new_ritual = Ritual {
            id: uuid::Uuid::new_v4(),
            user_id: user.id,
            title: data.title,
            body: data.body,
            published: false,
            created_at: now,
            updated_at: now,
        };
        let ritual: Ritual = diesel::insert_into(rituals::table)
            .values(&new_ritual)
            .get_result(conn)
            .expect("Error saving new post");
        let json = serde_json::to_string(&ritual).unwrap();

        Ok(HttpResponse::Ok().json(json))
    } else {
        Err(ServiceError::Unauthorized)
    }
}
