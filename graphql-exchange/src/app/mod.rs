pub mod profiles;
pub mod users;
pub mod orders;

mod mutation;
mod query;

use crate::{
    db::{new_pool, DbExecutor},
};
use actix::prelude::{Addr, SyncArbiter};
use actix_cors::Cors;
use actix_http::header::{HeaderMap, ORIGIN};
use actix_web::{
    guard,
    http::header::{AUTHORIZATION, CONTENT_TYPE},
    middleware::Logger,
    web,
    web::Data,
    App, HttpRequest, HttpResponse, HttpServer, Result,
};
use std::env;
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use query::QueryRoot;
use mutation::MutationRoot;

pub struct Token(pub String);

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

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish()))
}

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    let frontend_origin = env::var("FRONTEND_ORIGIN").ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_pool = new_pool(database_url).expect("Failed to create pool.");
    let database_address =
        SyncArbiter::start(num_cpus::get(), move || DbExecutor(database_pool.clone()));

    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");
    log::info!("GraphiQL IDE: {}", bind_address);

    HttpServer::new(move || {
        let state = AppState {
            db: database_address.clone(),
            //order_book: Mutex::new(order_book),
        };

        let _cors = match frontend_origin {
            Some(ref origin) if origin != "*" => Cors::default()
                .allowed_origin(origin)
                .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
                .max_age(3600),
            _ => Cors::default()
                .send_wildcard()
                .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE, ORIGIN])
                .max_age(3600),
        };

        let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
            .data(state)
            .finish();

        App::new()
            .app_data(Data::new(schema.clone()))
            .wrap(Logger::default())
            //.wrap(cors)
            .configure(routes)
    })
    .bind(&bind_address)?
    .run()
    .await
}

fn routes(app: &mut web::ServiceConfig)  {
    app.service(web::resource("/").guard(guard::Post()).to(index))
        .service(web::resource("/").guard(guard::Get()).to(index_graphiql));
}
