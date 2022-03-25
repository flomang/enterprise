use actix_identity::Identity;
use actix_web::{delete, get, post, web, HttpResponse};
use chrono::prelude::Utc;
use diesel::prelude::*;
use diesel::PgConnection;
use serde::{Deserialize, Serialize};

use crate::models::{Pool, Ritual, RitualTime, SlimUser};
use crate::utils::errors::ServiceError;

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
    total_pages: i64,
}

#[get("")]
pub async fn list_rituals(
    info: web::Query<PageInfo>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    use crate::utils::pagination::*;

    if let Some(str) = id.identity() {
        use crate::schema::rituals::dsl::*;

        let user: SlimUser = serde_json::from_str(&str).unwrap();
        let params = info.into_inner();
        let mut conn = pool.get().unwrap();

        //let (results, total_pages) = rituals
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
        let rid = path.into_inner();
        let conn = pool.get().unwrap();

        if let Ok(ritual_id) = uuid::Uuid::parse_str(&rid) {
            match diesel::delete(rituals.filter(user_id.eq(&user.id)).filter(id.eq(&ritual_id))).execute(&conn) {
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
        let rid = path.into_inner();
        let rid = uuid::Uuid::parse_str(&rid).unwrap();
        let conn = pool.get().unwrap();

        let results = rituals
            .filter(id.eq(&rid))
            .load::<Ritual>(&conn)
            .expect("could not find ritual by id");

        if results.len() > 0 {
            let json = serde_json::to_string(&results[0]).unwrap();
            Ok(HttpResponse::Ok().json(json))
        } else {
            Ok(HttpResponse::NotFound().json(""))
        }
    } else {
        Err(ServiceError::Unauthorized)
    }
}

#[derive(Deserialize)]
pub struct RitualTimestamp {
    ritual_id: String,
    created_at: String,
}

#[post("")]
pub async fn create_ritual_time(
    json: web::Json<RitualTimestamp>,
    identity: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(json_str) = identity.identity() {
        use crate::schema::ritual_times;

        let data = json.into_inner();
        let time = chrono::NaiveDateTime::parse_from_str(&data.created_at, "%Y-%m-%dT%H:%M:%S%z");
        let rid = uuid::Uuid::parse_str(&data.ritual_id);

        match (time, rid) {
            (Ok(created_at), Ok(ritual_id)) => {
                let conn = pool.get().unwrap();

                let new_time = RitualTime {
                    id: uuid::Uuid::new_v4(),
                    ritual_id,
                    created_at,
                };

                let result = diesel::insert_into(ritual_times::table)
                    .values(&new_time)
                    .get_result::<RitualTime>(&conn);

                match result {
                    Ok(t) => Ok(HttpResponse::Ok().json(t)),
                    Err(e) => Err(ServiceError::BadRequest(e.to_string())),
                }
            }
            (Err(_), _) => Err(ServiceError::BadRequest(
                "invalid created_at format".to_string(),
            )),
            (_, Err(_)) => Err(ServiceError::BadRequest(
                "invalid ritual_id format expected uuid".to_string(),
            )),
        }
    } else {
        Err(ServiceError::Unauthorized)
    }
}

#[derive(Serialize)]
struct RitualTimePage {
    page: i64,
    page_size: i64,
    timestamps: Vec<RitualTime>,
    total_pages: i64,
}

#[get("/{ritual_id}")]
pub async fn list_ritual_times(
    query_params: web::Query<PageInfo>,
    ritual_id: web::Path<String>,
    identity: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    use crate::utils::pagination::*;

    let ritual_id = ritual_id.into_inner();
    let ritual_id = uuid::Uuid::parse_str(&ritual_id).unwrap();

    if let Some(json_str) = identity.identity() {
        use crate::schema::ritual_times::dsl::*;

        let user: SlimUser = serde_json::from_str(&json_str).unwrap();
        let params = query_params.into_inner();
        let mut conn = pool.get().unwrap();

        let (results, total_pages) = ritual_times
            .filter(ritual_id.eq(&ritual_id))
            .order_by(created_at)
            .paginate(params.page)
            .per_page(params.page_size)
            .load_and_count_pages::<RitualTime>(&mut conn)
            .expect("query fav failed");

        let page = RitualTimePage {
            page: params.page,
            page_size: params.page_size,
            timestamps: results,
            total_pages: total_pages,
        };

        Ok(HttpResponse::Ok().json(page))
    } else {
        Err(ServiceError::Unauthorized)
    }
}

#[delete("/{id}")]
pub async fn delete_ritual_time(
    path: web::Path<String>,
    identity: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(str) = identity.identity() {
        use crate::schema::ritual_times::dsl::*;

        let _user: SlimUser = serde_json::from_str(&str).unwrap();
        let time_id = path.into_inner();
        let rid = uuid::Uuid::parse_str(&time_id).unwrap();
        let conn = pool.get().unwrap();

        let results = diesel::delete(ritual_times.filter(id.eq(&rid)))
            .execute(&conn)
            .expect("Error deleting posts");

        Ok(HttpResponse::Ok().json(results))
    } else {
        Err(ServiceError::Unauthorized)
    }
}

pub fn publish_ritual(conn: &PgConnection, id: uuid::Uuid) -> Ritual {
    use crate::schema::rituals::dsl::{published, rituals};

    diesel::update(rituals.find(id))
        .set(published.eq(true))
        .get_result::<Ritual>(conn)
        .expect(&format!("Unable to find ritual {}", id))
}

#[cfg(test)]
mod tests {
    use crate::establish_connection;

    #[test]
    fn test_ritual() {
        let mut conn = establish_connection();
        //create_ritual(&conn, "Quit Weed1", "one");
        //create_ritual(&conn, "Quit Weed2", "two");
        //create_ritual(&conn, "Quit Weed3", "three");
        //create_ritual(&conn, "Quit Weed4", "four");
        //create_ritual(&conn, "Quit Weed5", "five");
        //create_ritual(&conn, "Quit Weed6", "six");
        // let page_size = 3;
        // let (results, total_pages) = list_rituals(&mut conn, 2, page_size);
        // println!(
        //     "Displaying {} of total: {}",
        //     results.len(),
        //     (total_pages * page_size)
        // );
        // for ritual in results {
        //     println!("title: {}", ritual.title);
        //     println!("body: {}", ritual.body);
        //     println!("----------\n");
        // }

        // let mut weed = Ritual::new(String::from("Smoking Weed"));

        // weed.times = vec![
        //     Utc.ymd(2022, 3, 2).and_hms(12, 30, 11),
        //     Utc.ymd(2022, 3, 2).and_hms(1, 30, 10),
        //     Utc.ymd(2022, 3, 2).and_hms(17, 40, 10),
        // ];

        // weed.times.push( Utc.ymd(2022, 3, 2).and_hms(17, 40, 10));

        //assert_eq!(5, weed.times.len());
        assert_eq!(true, true);
    }
}
