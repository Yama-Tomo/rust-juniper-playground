use juniper::graphql_object;

use crate::context::Context;
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
    ) -> Option<User> {
        context.datasources.get_user(id).await
    }

    fn users(context: &Context) -> Option<Vec<User>> {
        context.datasources.get_users()
    }

    fn posts(context: &Context) -> Option<Vec<Post>> {
        context.datasources.get_posts()
    }

    fn hello() -> &'static str {
        "hello world!"
    }
}
