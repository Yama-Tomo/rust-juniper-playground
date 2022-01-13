use juniper::{graphql_object, GraphQLInputObject};

use crate::context::Context;
use crate::data_sources::DbUser;
use crate::resolvers::objects::Post;

#[derive(Clone)]
pub struct User {
    pub data: DbUser,
}

#[graphql_object(context = Context)]
impl User {
    pub fn id(&self) -> &i32 {
        &self.data.id
    }

    pub fn name(&self) -> &String {
        &self.data.name
    }

    pub async fn posts(&self, context: &Context) -> Vec<Post> {
        context.datasources.get_posts_by_user_id(self.data.id).await
    }
}

#[derive(GraphQLInputObject)]
pub struct UserInput {
    #[graphql(description = "name of the user")]
    pub name: String,
}
