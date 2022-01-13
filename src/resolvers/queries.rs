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
        context.datasources.user.get_by_id(id).await
    }

    fn users(context: &Context) -> Option<Vec<User>> {
        context.datasources.user.get_all()
    }

    fn posts(context: &Context) -> Option<Vec<Post>> {
        context.datasources.post.get_all()
    }

    fn hello() -> &'static str {
        "hello world!"
    }
}
