use juniper::{graphql_object, GraphQLInputObject};

use super::User;
use crate::context::Context;
use crate::data_sources::DbPost;

#[derive(Clone)]
pub struct Post {
    pub data: DbPost,
}

#[graphql_object(context = Context)]
impl Post {
    pub fn id(&self) -> &i32 {
        &self.data.id
    }

    pub fn title(&self) -> &String {
        &self.data.title
    }

    async fn user(&self, context: &Context) -> Option<User> {
        context.datasources.get_user(self.data.user_id).await
    }
}

#[derive(GraphQLInputObject)]
pub struct PostInput {
    pub user_id: i32,
    pub title: String,
}
