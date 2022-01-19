use juniper::{graphql_object, FieldResult, GraphQLInputObject, GraphQLObject, GraphQLUnion};

use crate::context::Context;
use crate::data_sources::models;
use crate::resolvers::errors::data_load_error;
use crate::resolvers::objects::{to_graphql_posts, Post, ValidationErrors};

#[derive(Clone)]
pub struct User {
    pub data: models::User,
}

#[graphql_object(context = Context)]
impl User {
    pub fn id(&self) -> &i32 {
        &self.data.id
    }

    pub fn name(&self) -> &String {
        &self.data.name
    }

    pub async fn posts(&self, context: &Context) -> FieldResult<Vec<Post>> {
        let posts = context.datasources.post.get_by_user_id(self.data.id).await;
        match posts {
            Ok(posts) => Ok(to_graphql_posts(posts)),
            Err(e) => Err(data_load_error(e)),
        }
    }
}

#[derive(GraphQLObject)]
pub struct DeletedUser {
    pub id: i32,
}

#[derive(GraphQLInputObject)]
pub struct UserInput {
    #[graphql(description = "name of the user")]
    pub name: String,
}

// TODO: genericsを使ってボイラープレートを減らしたい
#[derive(GraphQLUnion)]
#[graphql(Context = Context)]
pub enum UserSaveMutationResult {
    Ok(User),
    Err(ValidationErrors),
}

// TODO: genericsを使ってボイラープレートを減らしたい
#[derive(GraphQLUnion)]
#[graphql(Context = Context)]
pub enum UserDeleteMutationResult {
    Ok(DeletedUser),
    Err(ValidationErrors),
}

pub fn to_graphql_users(users: Vec<models::User>) -> Vec<User> {
    users
        .into_iter()
        .map(|i| User { data: i })
        .collect::<Vec<User>>()
}

pub fn to_optional_graphql_user(user: Option<models::User>) -> Option<User> {
    user.map(|data| User { data })
}
