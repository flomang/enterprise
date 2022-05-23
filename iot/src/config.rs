use crate::api::*;
use actix_web::web;

//pub const KEY: [u8; 16] = *include_bytes!("./secret.key");
//pub const IGNORE_ROUTES: [&str; 3] = ["/health", "/upload", "/download"];

pub fn config_services(cfg: &mut web::ServiceConfig) {
    log::info!("Configuring routes...");
    cfg.service(
        web::scope("")
            .service(ping_controller::healthz)
            .service(web::scope("/upload").service(media_controller::upload))
            .service(web::scope("/download").service(media_controller::download)),
    );
}
