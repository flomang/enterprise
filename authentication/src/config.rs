use crate::api::*;
use actix_web::web;

pub const IGNORE_ROUTES: [&str; 3] = ["/api/ping", "/api/login", "/api/register"];

pub fn config_services(cfg: &mut web::ServiceConfig) {
    log::info!("Configuring routes...");
    cfg.service(
        web::scope("/api")
            .service(ping_controller::ping)
            .service(web::scope("/invitations").service(invitation_controller::create_invitation))
            .service(web::scope("/register").service(account_controller::register_user))
            .service(web::scope("/login").service(account_controller::login))
            .service(web::scope("/logout").service(account_controller::logout)),
    );
}
