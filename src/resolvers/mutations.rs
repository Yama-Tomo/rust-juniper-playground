use juniper::graphql_object;

use crate::context::Context;
use crate::resolvers::objects::*;

pub struct Mutation;
#[graphql_object(context = Context)]
impl Mutation {
    fn add_user(context: &Context, input: UserInput) -> Option<User> {
        context.datasources.create_user(input)
    }

    fn update_user(context: &Context, id: i32, input: UserInput) -> User {
        context.datasources.update_user(id, input)
    }

    fn add_post(context: &Context, input: PostInput) -> Option<Post> {
        context.datasources.create_post(input)
    }

    fn update_post(context: &Context, id: i32, input: PostInput) -> Post {
        context.datasources.update_post(id, input)
    }
}
