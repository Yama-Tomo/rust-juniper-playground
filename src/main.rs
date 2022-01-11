use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};
use juniper::{graphql_object, EmptyMutation, EmptySubscription, GraphQLObject, RootNode};
use juniper_actix::{graphql_handler, playground_handler};
use std::collections::HashMap;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Clone, GraphQLObject)]
pub struct User {
    id: i32,
    name: String,
}

#[derive(Default, Clone)]
pub struct Database {
    users: HashMap<i32, User>,
}
impl Database {
    pub fn new() -> Database {
        let mut users = HashMap::new();
        users.insert(
            1,
            User {
                id: 1,
                name: "Aron".to_string(),
            },
        );
        users.insert(
            2,
            User {
                id: 2,
                name: "Bea".to_string(),
            },
        );
        users.insert(
            3,
            User {
                id: 3,
                name: "Carl".to_string(),
            },
        );
        users.insert(
            4,
            User {
                id: 4,
                name: "Dora".to_string(),
            },
        );
        Database { users }
    }
    pub fn get_user(&self, id: &i32) -> Option<&User> {
        self.users.get(id)
    }
}

struct Context {
    db: Database,
}

impl juniper::Context for Context {}

struct Query;
#[graphql_object(context = Context)]
impl Query {
    fn api_version() -> &'static str {
        "1.0"
    }

    fn user(
        context: &Context,
        #[graphql(description = "id of the user")] id: i32,
    ) -> Option<&User> {
        context.db.get_user(&id)
    }

    fn hello() -> &str {
        "hello world!"
    }
}

type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new(),
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
    let context = Context {
        db: Database::new(),
    };
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
