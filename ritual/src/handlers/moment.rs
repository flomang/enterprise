use actix_identity::Identity;
use actix_web::{delete, get, patch, post, web, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{Pool, RitualMoment, SlimUser, UpdateRitualMoment};
use kitchen::utils::errors::ServiceError;
use kitchen::utils::pagination::PageInfo;

#[derive(Deserialize)]
pub struct RitualTimestamp {
    ritual_id: String,
    notes: Option<String>,
    created_at: String,
}

#[post("")]
pub async fn create_ritual_moment(
    data: web::Json<RitualTimestamp>,
    identity: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(_json_str) = identity.identity() {
        use crate::schema::ritual_moments;

        if let Ok(ritual_id) = uuid::Uuid::parse_str(&data.ritual_id) {
            let data = data.into_inner();
            let time =
                chrono::NaiveDateTime::parse_from_str(&data.created_at, "%Y-%m-%dT%H:%M:%S%z");

            if let Ok(created_at) = time {
                let conn = pool.get().unwrap();

                let new_moment = RitualMoment {
                    id: uuid::Uuid::new_v4(),
                    ritual_id,
                    notes: data.notes,
                    created_at,
                };

                let result = diesel::insert_into(ritual_moments::table)
                    .values(&new_moment)
                    .get_result::<RitualMoment>(&conn);

                match result {
                    Ok(t) => Ok(HttpResponse::Ok().json(t)),
                    Err(e) => Err(ServiceError::BadRequest(e.to_string())),
                }
            } else {
                Err(ServiceError::BadRequest(
                    "invalid timestamp format for created_at".to_string(),
                ))
            }
        } else {
            Err(ServiceError::BadRequest("invalid ritual id".to_string()))
        }
    } else {
        Err(ServiceError::Unauthorized)
    }
}

#[derive(Serialize)]
struct RitualMomentPage {
    page: i64,
    page_size: i64,
    total_pages: i64,
    moments: Vec<RitualMoment>,
}

#[get("/{ritual_id}")]
pub async fn list_ritual_moments(
    params: web::Query<PageInfo>,
    path: web::Path<String>,
    identity: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    use kitchen::utils::pagination::*;

    if let Some(json_str) = identity.identity() {
        let ritual_id = path.into_inner();

        if let Ok(rid) = uuid::Uuid::parse_str(&ritual_id) {
            use crate::schema::ritual_moments::dsl::*;
            let _user: SlimUser = serde_json::from_str(&json_str).unwrap();
            let mut conn = pool.get().unwrap();

            let result = ritual_moments
                .filter(ritual_id.eq(&rid))
                .order_by(created_at)
                .paginate(params.page)
                .per_page(params.page_size)
                .load_and_count_pages::<RitualMoment>(&mut conn);

            match result {
                Ok((results, total_pages)) => {
                    let page = RitualMomentPage {
                        page: params.page,
                        page_size: params.page_size,
                        moments: results,
                        total_pages: total_pages,
                    };

                    Ok(HttpResponse::Ok().json(page))
                }
                Err(error) => Err(ServiceError::BadRequest(error.to_string())),
            }
        } else {
            Err(ServiceError::BadRequest("invalid ritual id".to_string()))
        }
    } else {
        Err(ServiceError::Unauthorized)
    }
}

#[delete("/{moment_id}")]
pub async fn delete_ritual_moment(
    path: web::Path<String>,
    identity: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(str) = identity.identity() {
        use crate::schema::ritual_moments::dsl::*;

        let moment_id = path.into_inner();

        if let Ok(mid) = uuid::Uuid::parse_str(&moment_id) {
            let conn = pool.get().unwrap();
            let _user: SlimUser = serde_json::from_str(&str).unwrap();

            let result = diesel::delete(ritual_moments.filter(id.eq(&mid))).execute(&conn);
            match result {
                Ok(size) => Ok(HttpResponse::Ok().json(size)),
                Err(error) => Err(ServiceError::BadRequest(error.to_string())),
            }
        } else {
            Err(ServiceError::BadRequest("invalid ritual id".to_string()))
        }
    } else {
        Err(ServiceError::Unauthorized)
    }
}

#[derive(Debug, Deserialize)]
pub struct MomentEditData {
    pub notes: Option<String>,
    pub created_at: Option<String>,
}

#[patch("/{moment_id}")]
pub async fn patch_ritual_moment(
    path: web::Path<String>,
    data: web::Json<MomentEditData>,
    identity: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(str) = identity.identity() {
        let moment_id = path.into_inner();

        if let Ok(id) = uuid::Uuid::parse_str(&moment_id) {
            use crate::schema::ritual_moments;

            let _user: SlimUser = serde_json::from_str(&str).unwrap();
            let data = data.into_inner();
            let notes = data.notes;
            let created_at: Option<chrono::NaiveDateTime> = match data.created_at {
                Some(date_str) => {
                    if let Ok(time) =
                        chrono::NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%z")
                    {
                        Some(time)
                    } else {
                        return  Err(ServiceError::BadRequest(
                            "invalid timestamp format for created_at".to_string(),
                        ));
                    }
                }
                _ => None,
            };

            let update_ritual = UpdateRitualMoment {
                id,
                notes,
                created_at,
            };

            let conn = pool.get().unwrap();
            let result = diesel::update(ritual_moments::table)
                .set(&update_ritual)
                .get_result::<RitualMoment>(&conn);

            match result {
                Ok(moment) => Ok(HttpResponse::Ok().json(moment)),
                Err(error) => Err(ServiceError::BadRequest(error.to_string())),
            }
        } else {
            Err(ServiceError::BadRequest("invalid ritual id".to_string()))
        }
    } else {
        Err(ServiceError::Unauthorized)
    }
}
