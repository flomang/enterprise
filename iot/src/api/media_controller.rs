use actix_web::{get, post, HttpResponse, web};

#[post("/{device_id}")]
async fn upload(path: web::Path<u32>) -> HttpResponse {
    let device_id = path.into_inner();
    debug!("upload");
    HttpResponse::Ok().body(format!("upload {}", device_id))
}

#[get("/{id}")]
async fn download(path: web::Path<u32>) -> HttpResponse {
    let id = path.into_inner();
    debug!("download");
    HttpResponse::Ok().body(format!("download {}", id))
}
