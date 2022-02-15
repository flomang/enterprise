use actix_web::{get, post, HttpRequest, HttpResponse, Responder};

#[get("/bangs")]
pub async fn bangs(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("bang list {}!", &name)
}

#[post("/moonbang")]
pub async fn moonbang(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}