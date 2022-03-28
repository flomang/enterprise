use actix_identity::Identity;
use actix_web::{delete, get, patch, post, web, HttpResponse};
use chrono::prelude::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use super::PageInfo;

use crate::models::{Pool, Ritual, UpdateRitual, SlimUser};
use crate::utils::errors::ServiceError;

#[derive(Debug, Deserialize)]
pub struct RitualData {
    pub title: String,
    pub body: String,
}

#[post("")]
pub async fn create_ritual(
    data: web::Json<RitualData>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    use crate::schema::rituals;

    // access request identity
    if let Some(str) = id.identity() {
        let data = data.into_inner();
        let user: SlimUser = serde_json::from_str(&str).unwrap();
        let conn = pool.get().unwrap();
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

        let result = diesel::insert_into(rituals::table)
            .values(&new_ritual)
            .get_result::<Ritual>(&conn);

        match result {
            Ok(ritual) => Ok(HttpResponse::Ok().json(ritual)),
            Err(_) => Err(ServiceError::InternalServerError),
        }
    } else {
        Err(ServiceError::Unauthorized)
    }
}

#[derive(Serialize)]
struct RitualPage {
    page: i64,
    page_size: i64,
    rituals: Vec<Ritual>,
    total_pages: i64,
}

#[get("")]
pub async fn list_rituals(
    params: web::Query<PageInfo>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    use crate::utils::pagination::*;

    if let Some(str) = id.identity() {
        use crate::schema::rituals::dsl::*;

        let user: SlimUser = serde_json::from_str(&str).unwrap();
        let mut conn = pool.get().unwrap();

        let result = rituals
            .filter(user_id.eq(&user.id))
            .order_by(created_at)
            .paginate(params.page)
            .per_page(params.page_size)
            .load_and_count_pages::<Ritual>(&mut conn);

        match result {
            Ok((results, total_pages)) => {
                let page = RitualPage {
                    page: params.page,
                    page_size: params.page_size,
                    rituals: results,
                    total_pages: total_pages,
                };

                Ok(HttpResponse::Ok().json(page))
            }
            Err(error) => Err(ServiceError::BadRequest(error.to_string())),
        }
    } else {
        Err(ServiceError::Unauthorized)
    }
}

#[delete("/{id}")]
pub async fn delete_ritual(
    path: web::Path<String>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(str) = id.identity() {
        use crate::schema::rituals::dsl::*;

        let user: SlimUser = serde_json::from_str(&str).unwrap();
        let ritual_id = path.into_inner();
        let conn = pool.get().unwrap();

        if let Ok(rid) = uuid::Uuid::parse_str(&ritual_id) {
            let result = diesel::delete(
                rituals
                    .filter(user_id.eq(&user.id))
                    .filter(id.eq(&rid)),
            )
            .execute(&conn);

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

#[get("/{id}")]
pub async fn get_ritual(
    path: web::Path<String>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(str) = id.identity() {
        use crate::schema::rituals::dsl::*;

        let _user: SlimUser = serde_json::from_str(&str).unwrap();
        let ritual_id = path.into_inner();
        let conn = pool.get().unwrap();

        if let Ok(rid) = uuid::Uuid::parse_str(&ritual_id) {
            match rituals.find(rid).first::<Ritual>(&conn) {
                Ok(ritual) => Ok(HttpResponse::Ok().json(ritual)),
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
pub struct RitualEditData {
    pub title: Option<String>,
    pub body: Option<String>,
    pub published: Option<bool>,
}

#[patch("/{id}")]
pub async fn patch_ritual(
    path: web::Path<String>,
    data: web::Json<RitualEditData>,
    identity: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(str) = identity.identity() {
        let ritual_id = path.into_inner();

        if let Ok(rid) = uuid::Uuid::parse_str(&ritual_id) {
            use crate::schema::rituals;

            let _user: SlimUser = serde_json::from_str(&str).unwrap();
            let conn = pool.get().unwrap();
            let data = data.into_inner();
            let now = Utc::now().naive_utc();

            let update_ritual = UpdateRitual{
                id: rid,
                title: data.title,
                body: data.body,
                published: data.published,
                updated_at: now,
            };

            let result = diesel::update(rituals::table)
                .set(&update_ritual)
                .get_result::<Ritual>(&conn);

            match result {
                Ok(ritual) => Ok(HttpResponse::Ok().json(ritual)),
                Err(error) => Err(ServiceError::BadRequest(error.to_string())),
            }
        } else {
            Err(ServiceError::BadRequest("invalid ritual id".to_string()))
        }
    } else {
        Err(ServiceError::Unauthorized)
    }
}

// #[derive(Deserialize)]
// pub struct RitualTimestamp {
//     ritual_id: String,
//     created_at: String,
// }

// #[post("")]
// pub async fn create_ritual_time(
//     data: web::Json<RitualTimestamp>,
//     identity: Identity,
//     pool: web::Data<Pool>,
// ) -> Result<HttpResponse, ServiceError> {
//     if let Some(_json_str) = identity.identity() {
//         use crate::schema::ritual_times;

//         if let Ok(ritual_id) = uuid::Uuid::parse_str(&data.ritual_id) {
//             let time =
//                 chrono::NaiveDateTime::parse_from_str(&data.created_at, "%Y-%m-%dT%H:%M:%S%z");

//             if let Ok(created_at) = time {
//                 let conn = pool.get().unwrap();

//                 let new_time = RitualTime {
//                     id: uuid::Uuid::new_v4(),
//                     ritual_id,
//                     created_at,
//                 };

//                 let result = diesel::insert_into(ritual_times::table)
//                     .values(&new_time)
//                     .get_result::<RitualTime>(&conn);

//                 match result {
//                     Ok(t) => Ok(HttpResponse::Ok().json(t)),
//                     Err(e) => Err(ServiceError::BadRequest(e.to_string())),
//                 }
//             } else {
//                 Err(ServiceError::BadRequest(
//                     "invalid timestamp format for created_at".to_string(),
//                 ))
//             }
//         } else {
//             Err(ServiceError::BadRequest("invalid ritual id".to_string()))
//         }
//     } else {
//         Err(ServiceError::Unauthorized)
//     }
// }

// #[derive(Serialize)]
// struct RitualTimePage {
//     page: i64,
//     page_size: i64,
//     timestamps: Vec<RitualTime>,
//     total_pages: i64,
// }

// #[get("/{ritual_id}")]
// pub async fn list_ritual_times(
//     params: web::Query<PageInfo>,
//     path: web::Path<String>,
//     identity: Identity,
//     pool: web::Data<Pool>,
// ) -> Result<HttpResponse, ServiceError> {
//     use crate::utils::pagination::*;

//     if let Some(json_str) = identity.identity() {
//         let ritual_id = path.into_inner();

//         if let Ok(rid) = uuid::Uuid::parse_str(&ritual_id) {
//             use crate::schema::ritual_times::dsl::*;
//             let _user: SlimUser = serde_json::from_str(&json_str).unwrap();
//             let mut conn = pool.get().unwrap();

//             let result = ritual_times
//                 .filter(ritual_id.eq(&rid))
//                 .order_by(created_at)
//                 .paginate(params.page)
//                 .per_page(params.page_size)
//                 .load_and_count_pages::<RitualTime>(&mut conn);

//             match result {
//                 Ok((results, total_pages)) => {
//                     let page = RitualTimePage {
//                         page: params.page,
//                         page_size: params.page_size,
//                         timestamps: results,
//                         total_pages: total_pages,
//                     };

//                     Ok(HttpResponse::Ok().json(page))
//                 }
//                 Err(error) => Err(ServiceError::BadRequest(error.to_string())),
//             }
//         } else {
//             Err(ServiceError::BadRequest("invalid ritual id".to_string()))
//         }
//     } else {
//         Err(ServiceError::Unauthorized)
//     }
// }

// #[delete("/{id}")]
// pub async fn delete_ritual_time(
//     path: web::Path<String>,
//     identity: Identity,
//     pool: web::Data<Pool>,
// ) -> Result<HttpResponse, ServiceError> {
//     if let Some(str) = identity.identity() {
//         use crate::schema::ritual_times::dsl::*;

//         let time_id = path.into_inner();

//         if let Ok(tid) = uuid::Uuid::parse_str(&time_id) {
//             let conn = pool.get().unwrap();
//             let _user: SlimUser = serde_json::from_str(&str).unwrap();

//             let result = diesel::delete(ritual_times.filter(id.eq(&tid))).execute(&conn);
//             match result {
//                 Ok(size) => Ok(HttpResponse::Ok().json(size)),
//                 Err(error) => Err(ServiceError::BadRequest(error.to_string())),
//             }
//         } else {
//             Err(ServiceError::BadRequest("invalid ritual id".to_string()))
//         }
//     } else {
//         Err(ServiceError::Unauthorized)
//     }
// }
