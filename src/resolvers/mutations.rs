use juniper::graphql_object;

use crate::context::Context;
use crate::resolvers::objects::*;

pub struct Mutation;
#[graphql_object(context = Context)]
impl Mutation {
    fn add_user(context: &Context, input: UserInput) -> Option<User> {
        context.datasources.user.create_user(input)
    }

    fn update_user(context: &Context, id: i32, input: UserInput) -> User {
        context.datasources.user.update_user(id, input)
    }

    fn delete_user(context: &Context, id: i32) -> i32 {
        context.datasources.user.delete_user(id)
    }

    fn add_post(context: &Context, input: PostInput) -> Option<Post> {
        context.datasources.post.create_post(input)
    }

    fn update_post(context: &Context, id: i32, input: PostInput) -> Post {
        context.datasources.post.update_post(id, input)
    }

    fn delete_post(context: &Context, id: i32) -> i32 {
        context.datasources.post.delete_post(id)
    }
}
