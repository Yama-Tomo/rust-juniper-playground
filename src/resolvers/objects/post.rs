use juniper::graphql_object;

use super::User;
use crate::context::Context;
use crate::data_sources::DbPost;

pub struct Post<'a> {
    pub data: &'a DbPost,
}

#[graphql_object(context = Context)]
impl<'a> Post<'a> {
    pub fn id(&self) -> &i32 {
        &self.data.id
    }

    pub fn title(&self) -> &String {
        &self.data.title
    }

    fn user(&self, context: &Context) -> Option<User> {
        context.db.get_user(&self.data.user_id)
    }
}
