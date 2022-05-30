use actix_web::{get, post, web, HttpResponse};
use diesel::PgConnection;
use library::db::Pool;

use crate::actions::media_data::MediaDataAdd;
//use crate::database::PgPooled;

#[post("/add")]
async fn add_media(data: web::Json<MediaDataAdd>, pool: web::Data<Pool>) -> HttpResponse {
    debug!("-- add media data --");
    debug!(">>>> JSON ::: {:?}", data);

    web::block(move || {
        let conn: &PgConnection = &pool.get().unwrap();
        data.into_inner().save(conn);
    })
    .await;

    HttpResponse::Ok().body(format!("OK"))
}

#[get("/{id}")]
async fn download(path: web::Path<u32>) -> HttpResponse {
    let id = path.into_inner();
    debug!("download");
    HttpResponse::Ok().body(format!("download {}", id))
}
