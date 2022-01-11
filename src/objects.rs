use juniper::GraphQLObject;

#[derive(Clone, GraphQLObject)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Clone, GraphQLObject)]
pub struct Post {
    pub id: i32,
    pub title: String,
}
