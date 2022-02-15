use actix::*;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use std::time::Instant;

use crate::asteroid::server;
use crate::asteroid::chat::*;

#[get("/bangs")]
pub async fn bangs(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("bang list {}!", &name)
}

#[post("/moonbang")]
pub async fn moonbang(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "Main".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}