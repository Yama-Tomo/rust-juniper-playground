use juniper::{graphql_object, FieldResult, GraphQLInputObject, GraphQLObject, GraphQLUnion};

use super::User;
use crate::context::Context;
use crate::data_sources::models;
use crate::resolvers::errors::data_load_error;
use crate::resolvers::objects::ValidationErrors;

#[derive(Clone)]
pub struct Post {
    pub data: models::Post,
}

#[graphql_object(context = Context)]
impl Post {
    pub fn id(&self) -> &i32 {
        &self.data.id
    }

    pub fn title(&self) -> &String {
        &self.data.title
    }

    async fn user(&self, context: &Context) -> FieldResult<Option<User>> {
        let user = context.datasources.user.get_by_id(self.data.user_id).await;
        match user {
            Ok(user) => Ok(user),
            Err(e) => Err(data_load_error(e)),
        }
    }
}

#[derive(GraphQLObject)]
pub struct DeletedPost {
    pub id: i32,
}

#[derive(GraphQLInputObject)]
pub struct PostInput {
    pub user_id: i32,
    pub title: String,
}

#[derive(GraphQLUnion)]
#[graphql(Context = Context)]
pub enum PostSaveMutationResult {
    Ok(Post),
    Err(ValidationErrors),
}

#[derive(GraphQLUnion)]
#[graphql(Context = Context)]
pub enum PostDeleteMutationResult {
    Ok(DeletedPost),
    Err(ValidationErrors),
}
