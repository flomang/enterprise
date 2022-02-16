use actix::*;
use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use futures::StreamExt;
use std::time::Instant;


use crate::asteroid::server;
use crate::asteroid::chat::*;
use crate::asteroid::MoonBang;

const MAX_SIZE: usize = 262_144; // max payload size is 256k

#[get("/bangs")]
pub async fn bangs(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("bang list {}!", &name)
}

#[post("/moonbang")]
pub async fn moonbang(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<MoonBang>(&body)?;
    Ok(HttpResponse::Ok().json(obj)) // <- send response
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