use juniper::GraphQLObject;

#[derive(Clone, GraphQLObject)]
pub struct User {
    pub id: i32,
    pub name: String,
}
