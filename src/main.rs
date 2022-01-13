use actix_web::{
    web::{self, Data},
    App, Error, HttpResponse, HttpServer, Responder,
};
use juniper_actix::{graphql_handler, playground_handler};

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
) -> Result<HttpResponse, Error> {
    let context = context::create();
    graphql_handler(&schema, &context, req, payload).await
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(schema::create()))
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
