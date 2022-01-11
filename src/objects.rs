use juniper::{graphql_object, GraphQLObject};

#[derive(Clone, GraphQLObject)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Clone)]
pub struct Post {
    pub id: i32,
    pub title: String,
}

#[graphql_object]
impl Post {
    pub fn id(&self) -> &i32 {
        &self.id
    }

    pub fn title(&self) -> &String {
        &self.title
    }
}
