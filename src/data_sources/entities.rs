#[derive(Clone)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
}

#[derive(Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
}
