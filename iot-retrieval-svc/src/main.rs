#[macro_use]
extern crate diesel;
#[macro_use] 
extern crate log;
extern crate pretty_env_logger;

use std::{io, sync::Arc};
use actix_cors::Cors;
use actix_web::{http, get, route, web::{self, Data}, App, HttpServer, HttpResponse, Responder};
use clap::{AppSettings, ArgMatches, SubCommand, Parser};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv;
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};


mod api;
mod database;
mod errors;
mod config;
mod middleware;
mod models;
mod graphql;


/// GraphiQL playground UI
#[get("/playground")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graph(st: web::Data<graphql::schema::Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let user = data.execute(&st, &()).await;
    HttpResponse::Ok().json(user)
}


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(env, short, long, default_value_t = 3010)]
    port: i32,
    #[clap(env, short, long)]
    database_url: String,
    #[clap(env, short, long)]
    allowed_origin: String,
}

// Tokio-based single-threaded async runtime for the Actix ecosystem.
// To achieve similar performance to multi-threaded, work-stealing runtimes, applications using actix-rt will create multiple, mostly disconnected, single-threaded runtimes.
// This approach has good performance characteristics for workloads where the majority of tasks have similar runtime expense.
// https://docs.rs/actix-rt/latest/actix_rt/index.html
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let cli = Cli::parse();
    let port = cli.port;

    log::info!("starting HTTP server on port {}", port);
    log::info!("GraphiQL playground: http://localhost:{}/graphiql", port);

    // Start http server
    HttpServer::new(move || {
        let database_url = cli.database_url.clone();
        let allowed_origin = cli.allowed_origin.clone();

        let schema = Arc::new(graphql::schema::create_schema());

        // create db connection pool
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: library::db::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        //let cors = Cors::default()
        //    .allowed_origin_fn(move |origin, _req_head| {
        //        origin.as_bytes().ends_with(allowed_origin.as_bytes())
        //    })
        //    .allowed_methods(vec!["GET", "POST"])
        //    .allowed_headers(vec![http::header::ACCEPT, http::header::CONTENT_TYPE])
        //    .max_age(3600);
        let cors = Cors::permissive();

        App::new()
            .app_data(web::Data::new(pool))
            .app_data(Data::from(schema))
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            .wrap(middleware::timer::ResponseTime)
            .service(graph)
            .service(graphql_playground)
            //.configure(config::config_services)
            .app_data(web::JsonConfig::default().limit(4096))
    })
    .workers(1)
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
