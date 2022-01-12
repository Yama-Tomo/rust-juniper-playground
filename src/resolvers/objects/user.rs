use juniper::graphql_object;

use crate::context::Context;
use crate::data_sources::DbUser;

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
}
