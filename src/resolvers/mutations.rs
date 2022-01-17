use juniper::graphql_object;

use crate::context::Context;
use crate::resolvers::objects::*;

pub struct Mutation;
#[graphql_object(context = Context)]
impl Mutation {
    async fn add_user(context: &Context, input: UserInput) -> Option<User> {
        context.datasources.user.create(input).await
    }

    async fn update_user(context: &Context, id: i32, input: UserInput) -> User {
        context.datasources.user.update(id, input).await
    }

    async fn delete_user(context: &Context, id: i32) -> i32 {
        context.datasources.user.delete(id).await
    }

    async fn add_post(context: &Context, input: PostInput) -> Option<Post> {
        context.datasources.post.create(input).await
    }

    async fn update_post(context: &Context, id: i32, input: PostInput) -> Post {
        context.datasources.post.update(id, input).await
    }

    async fn delete_post(context: &Context, id: i32) -> i32 {
        context.datasources.post.delete(id).await
    }
}
