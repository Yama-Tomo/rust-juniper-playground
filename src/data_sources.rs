use std::collections::HashMap;

use crate::resolvers::objects::*;

pub struct DbUser {
    pub id: i32,
    pub name: String,
}

pub struct DbPost {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
}

type DbUsers = HashMap<i32, DbUser>;
type DbPosts = HashMap<i32, DbPost>;
#[derive(Default)]
pub struct DataSources {
    users: DbUsers,
    posts: DbPosts,
}
impl DataSources {
    pub fn new() -> DataSources {
        let mut users: DbUsers = HashMap::new();
        let items = ["Aron", "Bea", "carl", "Dora"];

        for (i, item) in items.iter().enumerate() {
            let id = i as i32 + 1;
            users.insert(
                id,
                DbUser {
                    id,
                    name: item.to_string(),
                },
            );
        }

        let mut posts: DbPosts = HashMap::new();
        let mut id = 0;
        let mut insert_post = |user_id: i32, vol: i32| -> () {
            id = id + 1;
            let user_name = &users.get(&user_id).unwrap().name;
            posts.insert(
                id,
                DbPost {
                    id,
                    user_id,
                    title: format!("{} vol:{}", user_name, vol),
                },
            );
        };

        insert_post(1, 1);
        insert_post(1, 2);
        insert_post(3, 1);
        insert_post(4, 1);
        insert_post(4, 2);
        insert_post(4, 3);

        DataSources { users, posts }
    }

    pub fn get_user(&self, id: &i32) -> Option<User> {
        let user = self.users.get(id);
        match user {
            Some(u) => Some(User { data: u }),
            None => None,
        }
    }

    pub fn get_users(&self) -> Option<Vec<User>> {
        let users = self
            .users
            .values()
            .map(|u| User { data: u })
            .collect::<Vec<User>>();

        Some(users)
    }

    pub fn get_posts(&self) -> Option<Vec<Post>> {
        let posts = self
            .posts
            .values()
            .map(|p| Post { data: p })
            .collect::<Vec<Post>>();

        Some(posts)
    }
}
