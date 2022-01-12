use async_trait::async_trait;
use dataloader::non_cached::Loader;
use dataloader::BatchFn;
use std::collections::HashMap;

use crate::resolvers::objects::*;

#[derive(Clone)]
pub struct DbUser {
    pub id: i32,
    pub name: String,
}

#[derive(Clone)]
pub struct DbPost {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
}

type DbUsers = HashMap<i32, DbUser>;
type DbPosts = HashMap<i32, DbPost>;

pub struct DataSources {
    users: DbUsers,
    user_loader: UserLoaderType,
    posts: DbPosts,
    post_loader: PostLoaderType,
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

        DataSources {
            users: users.clone(),
            posts: posts.clone(),
            user_loader: create_user_loader(users),
            post_loader: create_post_loader(posts),
        }
    }

    pub async fn get_user(&self, id: i32) -> Option<User> {
        self.user_loader.load(id).await
    }

    pub fn get_users(&self) -> Option<Vec<User>> {
        let users = self
            .users
            .values()
            .map(|u| User { data: u.clone() })
            .collect::<Vec<User>>();

        Some(users)
    }

    pub async fn get_post_by_user_id(&self, id: i32) -> Vec<Post> {
        self.post_loader.load(id).await
    }

    pub fn get_posts(&self) -> Option<Vec<Post>> {
        let posts = self
            .posts
            .values()
            .map(|p| Post { data: p.clone() })
            .collect::<Vec<Post>>();

        Some(posts)
    }
}

pub struct UserLoader {
    users: DbUsers,
}
#[async_trait]
impl BatchFn<i32, Option<User>> for UserLoader {
    async fn load(&mut self, keys: &[i32]) -> HashMap<i32, Option<User>> {
        println!("fetch user_id = {:?}", keys);
        let mut hashmap: HashMap<i32, Option<User>> = HashMap::new();

        let fetch_data = self
            .users
            .values()
            .filter(|i| keys.contains(&i.id))
            .collect::<Vec<&DbUser>>();

        for key in keys {
            let data = match fetch_data.iter().find(|i| &i.id == key) {
                Some(&u) => Some(User { data: u.clone() }),
                None => None,
            };

            hashmap.insert(*key, data);
        }

        hashmap
    }
}

pub type UserLoaderType = Loader<i32, Option<User>, UserLoader>;
pub fn create_user_loader(users: DbUsers) -> UserLoaderType {
    Loader::new(UserLoader { users }).with_yield_count(100)
}

pub struct PostLoader {
    posts: DbPosts,
}
#[async_trait]
impl BatchFn<i32, Vec<Post>> for PostLoader {
    async fn load(&mut self, keys: &[i32]) -> HashMap<i32, Vec<Post>> {
        println!("fetch post_id = {:?}", keys);
        let mut hashmap: HashMap<i32, Vec<Post>> = HashMap::new();

        let fetch_data = self
            .posts
            .values()
            .filter(|i| keys.contains(&i.id))
            .collect::<Vec<&DbPost>>();

        for key in keys {
            let posts = fetch_data
                .iter()
                .filter(|i| &i.user_id == key)
                .map(|&i| Post { data: i.clone() })
                .collect::<Vec<Post>>();

            hashmap.insert(*key, posts);
        }

        hashmap
    }
}

pub type PostLoaderType = Loader<i32, Vec<Post>, PostLoader>;
pub fn create_post_loader(posts: DbPosts) -> PostLoaderType {
    Loader::new(PostLoader { posts }).with_yield_count(100)
}
