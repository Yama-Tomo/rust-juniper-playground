use juniper::{graphql_object, FieldResult};

use crate::context::Context;
use crate::resolvers::errors::*;
use crate::resolvers::objects::*;

pub struct Query;
#[graphql_object(context = Context)]
impl Query {
    fn api_version() -> &'static str {
        "1.0"
    }

    async fn user(
        context: &Context,
        #[graphql(description = "id of the user")] id: i32,
    ) -> FieldResult<Option<User>> {
        let user = context.datasources.user.get_by_id(id).await;
        match user {
            Ok(user) => Ok(user),
            Err(e) => Err(data_load_error(e)),
        }
    }

    async fn users(context: &Context) -> FieldResult<Vec<User>> {
        let users = context.datasources.user.get_all().await;
        match users {
            Ok(users) => Ok(users),
            Err(e) => Err(data_load_error(e)),
        }
    }

    async fn posts(context: &Context) -> FieldResult<Vec<Post>> {
        let posts = context.datasources.post.get_all().await;
        match posts {
            Ok(posts) => Ok(posts),
            Err(e) => Err(data_load_error(e)),
        }
    }

    fn hello() -> &'static str {
        "hello world!"
    }
}
