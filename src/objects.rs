use juniper::{graphql_object, GraphQLObject};

use crate::context::Context;

#[derive(Clone, GraphQLObject)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Clone)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub user_id: i32,
}

#[graphql_object(context = Context)]
impl Post {
    pub fn id(&self) -> &i32 {
        &self.id
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    fn user(&self, context: &Context) -> Option<User> {
        context.db.get_user(&self.user_id)
    }
}
