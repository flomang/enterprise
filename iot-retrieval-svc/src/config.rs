use crate::api::*;
use actix_web::web;

//pub const KEY: [u8; 16] = *include_bytes!("./secret.key");
//pub const IGNORE_ROUTES: [&str; 3] = ["/health", "/upload", "/download"];

pub fn config_services(cfg: &mut web::ServiceConfig) {
    log::info!("Configuring routes...");
    cfg.service(
        web::scope("/api")
            .service(ping_controller::healthz)
            .service(web::scope("/media").service(media_controller::add_media))
            .service(web::scope("/comment").service(media_controller::add_comment)),
    );
}
