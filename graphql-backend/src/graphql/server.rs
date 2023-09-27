use crate::{
    db::{new_pool, DbExecutor},
    utils::auth::Token,
};
use actix::prelude::{Addr, SyncArbiter};
use actix_cors::Cors;
use actix_http::header::HeaderMap;
use actix_web::{
    http::header::{AUTHORIZATION, CONTENT_TYPE},
    middleware::Logger,
    web,
    web::Data,
    App, HttpRequest, HttpResponse, HttpServer,
};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use super::mutation::MutationRoot;
use super::query::QueryRoot;
use std::env;

pub type GraphqlSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

fn get_token_from_headers(headers: &HeaderMap) -> Option<Token> {
    headers
        .get("Token")
        .and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok())
}

async fn index(
    schema: web::Data<GraphqlSchema>,
    req: HttpRequest,
    gql_request: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = gql_request.into_inner();
    if let Some(token) = get_token_from_headers(req.headers()) {
        request = request.data(token);
    }
    schema.execute(request).await.into()
}

async fn index_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

fn routes(app: &mut web::ServiceConfig) {
    app.service(
        web::resource("/")
            .route(web::post().to(index))
            .route(web::get().to(index_playground)),
    );
}

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    // set vars from .env file
    let frontend_origin = env::var("FRONTEND_ORIGIN").ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");

    let database_pool = new_pool(database_url).expect("Failed to create pool.");
    let database_address =
        SyncArbiter::start(num_cpus::get(), move || DbExecutor(database_pool.clone()));

    log::info!("GraphiQL IDE: {}", bind_address);
    HttpServer::new(move || {
        let state = AppState {
            db: database_address.clone(),
        };

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

        let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
            .data(state)
            .finish();

        App::new()
            .app_data(Data::new(schema.clone()))
            .wrap(Logger::default())
            .wrap(cors)
            .configure(routes)
    })
    .bind(&bind_address)?
    .run()
    .await
}
