use juniper::graphql_object;

use crate::context::Context;
use crate::data_sources::DbUser;

pub struct User<'a> {
    pub data: &'a DbUser,
}

#[graphql_object(context = Context)]
impl<'a> User<'a> {
    pub fn id(&self) -> &i32 {
        &self.data.id
    }

    pub fn name(&self) -> &String {
        &self.data.name
    }
}
