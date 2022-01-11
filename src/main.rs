use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};
use juniper::{EmptyMutation, EmptySubscription, RootNode};
use juniper_actix::{graphql_handler, playground_handler};

mod context;
mod data_sources;
mod objects;
mod queries;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

type Schema = RootNode<
    'static,
    queries::Query,
    EmptyMutation<context::Context>,
    EmptySubscription<context::Context>,
>;

fn schema() -> Schema {
    Schema::new(
        queries::Query,
        EmptyMutation::<context::Context>::new(),
        EmptySubscription::<context::Context>::new(),
    )
}

async fn playground_route() -> Result<HttpResponse, Error> {
    playground_handler("/graphql", None).await
}

async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
    let context = context::create();
    graphql_handler(&schema, &context, req, payload).await
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(schema())
            .route("/", web::get().to(index))
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql_route))
                    .route(web::get().to(playground_route)),
            )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
