use juniper::graphql_object;

use crate::context::Context;
use crate::objects::User;

pub struct Query;
#[graphql_object(context = Context)]
impl Query {
    fn api_version() -> &'static str {
        "1.0"
    }

    fn user(context: &Context, #[graphql(description = "id of the user")] id: i32) -> Option<User> {
        context.db.get_user(&id)
    }

    fn users(context: &Context) -> Option<Vec<User>> {
        context.db.get_users()
    }

    fn hello() -> &str {
        "hello world!"
    }
}
