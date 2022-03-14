use actix_identity::Identity;
use actix_web::{delete, get, post, web, HttpResponse};
use chrono::prelude::Utc;
use diesel::prelude::*;
use diesel::PgConnection;
use serde::{Deserialize, Serialize};

use crate::errors::ServiceError;
use crate::models::{Pool, Ritual, SlimUser};

#[derive(Debug, Deserialize)]
pub struct RitualData {
    pub title: String,
    pub body: String,
}

#[post("")]
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

#[derive(Deserialize)]
pub struct PageInfo {
    page: i64,
    page_size: i64,
}

#[derive(Serialize)]
struct RitualPage {
    page: i64,
    page_size: i64,
    rituals: Vec<Ritual>,
    total: i64,
}

#[get("")]
pub async fn list_rituals(
    info: web::Query<PageInfo>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    use crate::pagination::*;

    if let Some(str) = id.identity() {
        use crate::schema::rituals::dsl::*;

        let user: SlimUser = serde_json::from_str(&str).unwrap();
        let params = info.into_inner();
        let mut conn = pool.get().unwrap();

        let (results, total_pages) = rituals
            .filter(user_id.eq(&user.id))
            .order_by(created_at)
            .paginate(params.page)
            .per_page(params.page_size)
            .load_and_count_pages::<Ritual>(&mut conn)
            .expect("query fav failed");

        let page = RitualPage {
            page: params.page,
            page_size: params.page_size,
            rituals: results,
            total: total_pages,
        };

        let json = serde_json::to_string(&page).unwrap();
        Ok(HttpResponse::Ok().json(json))
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

        let _user: SlimUser = serde_json::from_str(&str).unwrap();
        let rid = path.into_inner();
        let rid = uuid::Uuid::parse_str(&rid).unwrap();
        let conn = pool.get().unwrap();

        let results = diesel::delete(rituals.filter(id.eq(&rid)))
            .execute(&conn)
            .expect("Error deleting posts");

        Ok(HttpResponse::Ok().json(results))
    } else {
        Err(ServiceError::Unauthorized)
    }
}
