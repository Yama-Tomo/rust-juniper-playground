use actix_web::{
    web::{self, Data},
    App, Error, HttpResponse, HttpServer, Responder,
};
use dotenv::dotenv;
use juniper_actix::{graphql_handler, playground_handler};
use sea_orm::DatabaseConnection;

use crate::data_sources::create_db_connection;

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

pub async fn run() -> std::io::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let conn = create_db_connection().await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema::create()))
            .app_data(Data::new(conn.clone()))
            .route("/", web::get().to(index))
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql_route))
                    .route(web::get().to(playground_route)),
            )
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}
