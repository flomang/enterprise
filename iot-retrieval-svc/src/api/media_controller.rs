use actix_web::{get, post, web, HttpResponse};
use diesel::PgConnection;
use library::db::Pool;
use serde::Deserialize;
use uuid::Uuid;

use crate::actions::media_data::MediaDataAdd;
use crate::models::comment::Comment;

#[post("/add")]
async fn add_media(data: web::Json<MediaDataAdd>, pool: web::Data<Pool>) -> HttpResponse {
    debug!("add media data");
    //debug!(">>>> JSON ::: {:?}", data);

    web::block(move || {
        let conn: &PgConnection = &pool.get().unwrap();
        data.into_inner().save(conn);
    })
    .await;

    HttpResponse::Ok().body(format!("OK"))
}


#[derive(Deserialize)]
pub struct CommentRequest {
    pub comment: String,
}

#[post("/add/{media_item_id}")]
async fn add_comment(path: web::Path<String>, data: web::Json<CommentRequest>, pool: web::Data<Pool>) -> HttpResponse {
    debug!("add comment");
    let id = path.into_inner();
    let id = Uuid::parse_str(&id).unwrap();

    // TODO this will fail if the media item ID does not exist
    web::block(move || {
        let conn: &PgConnection = &pool.get().unwrap();
        let req =  data.into_inner();
        Comment::add(&conn, id, req.comment);
    });

    HttpResponse::Ok().body(format!("OK"))
}
