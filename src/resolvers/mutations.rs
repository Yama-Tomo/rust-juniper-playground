use juniper::graphql_object;

use crate::context::Context;
use crate::resolvers::objects::*;

pub struct Mutation;
#[graphql_object(context = Context)]
impl Mutation {
    fn add_user(context: &Context, input: UserInput) -> Option<User> {
        context.datasources.create_user(input)
    }

    fn add_post(context: &Context, input: PostInput) -> Option<Post> {
        context.datasources.create_post(input)
    }
}
