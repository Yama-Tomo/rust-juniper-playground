use actix_web::{
    web::{self, Data},
    App, Error, HttpResponse, HttpServer, Responder,
};
use dotenv::dotenv;
use juniper_actix::{graphql_handler, playground_handler};
use sea_orm::DatabaseConnection;
use std::env;

pub use crate::data_sources::create_db_connection;

mod context;
mod data_sources;
mod resolvers;
mod schema;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn playground_route() -> Result<HttpResponse, Error> {
    playground_handler("/graphql", None).await
}

async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<schema::Schema>,
    conn: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    let context = context::create(&conn);
    graphql_handler(&schema, &context, req, payload).await
}

// TODO: テスト時のDBをどうするかを決めたらOption外す
pub fn configure(cfg: &mut web::ServiceConfig, conn: Option<DatabaseConnection>) {
    if let Some(conn) = conn {
        cfg.app_data(Data::new(conn));
    }

    cfg.app_data(Data::new(schema::create()))
        .route("/", web::get().to(index))
        .service(
            web::resource("/graphql")
                .route(web::post().to(graphql_route))
                .route(web::get().to(playground_route)),
        );
}

pub async fn run() -> std::io::Result<()> {
    dotenv().ok();

    let trace_level = if cfg!(debug_assertions) {
        tracing::Level::DEBUG
    } else {
        tracing::Level::WARN
    };
    tracing_subscriber::fmt()
        .with_max_level(trace_level)
        .with_test_writer()
        .init();

    let conn = create_db_connection().await;
    let port = env::var("PORT").unwrap_or_else(|_| "8088".to_string());

    HttpServer::new(move || App::new().configure(|cfg| configure(cfg, Some(conn.clone()))))
        .bind(format!("0.0.0.0:{}", port))?
        .run()
        .await
}
