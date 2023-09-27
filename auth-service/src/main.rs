extern crate auth_service;

use std::env;

use actix::SyncArbiter;
use actix_cors::Cors;
use actix_web::{
    http::header::{AUTHORIZATION, CONTENT_TYPE},
    web, App, HttpServer,
};
use auth_service::{
    configure_service, create_schema_with_context, db::DbExecutor, run_migrations, AppState,
};
use common_utils::create_connection_pool;
use dotenv::dotenv;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = create_connection_pool();
    run_migrations(&mut pool.get().expect("Can't get DB connection"));

    let database_address = SyncArbiter::start(num_cpus::get(), move || DbExecutor(pool.clone()));
    let frontend_origin: Option<String> = env::var("FRONTEND_ORIGIN").ok();
    let server_port = env::var("SERVER_PORT").expect("Can't get server port");

    HttpServer::new(move || {
        let state = AppState {
            db: database_address.clone(),
        };

        let schema = web::Data::new(create_schema_with_context(state));

        // allow wildcard for development purposes
        let cors = match frontend_origin {
            // TODO production should not be allowed to send wildcard
            Some(ref origin) if origin != "*" => Cors::default()
                .allowed_origin(origin)
                .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
                .max_age(3600),
            _ => Cors::default()
                .send_wildcard()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600),
        };

        App::new()
            .wrap(cors)
            .configure(configure_service)
            .app_data(schema.clone())
    })
    .bind(format!("0.0.0.0:{}", server_port))?
    .run()
    .await
}
