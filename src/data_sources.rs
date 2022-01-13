use async_trait::async_trait;
use dataloader::non_cached::Loader;
use dataloader::BatchFn;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

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

// TODO: RDBMSを使うようになったらonce_cellもアンインストール
static DB_USERS: Lazy<Mutex<DbUsers>> = Lazy::new(|| Mutex::new(HashMap::new()));
static DB_POSTS: Lazy<Mutex<DbPosts>> = Lazy::new(|| Mutex::new(HashMap::new()));
pub fn init_data() -> () {
    if DB_USERS.lock().unwrap().len() > 0 {
        return;
    }

    let items = ["Aron", "Bea", "carl", "Dora"];

    for (i, item) in items.iter().enumerate() {
        let id = i as i32 + 1;
        DB_USERS.lock().unwrap().insert(
            id,
            DbUser {
                id,
                name: item.to_string(),
            },
        );
    }

    let mut id = 0;
    let mut insert_post = |user_id: i32, vol: i32| -> () {
        id = id + 1;
        DB_POSTS.lock().unwrap().insert(
            id,
            DbPost {
                id,
                user_id,
                title: format!(
                    "{} vol:{}",
                    DB_USERS.lock().unwrap().get(&user_id).unwrap().name,
                    vol
                ),
            },
        );
    };

    insert_post(1, 1);
    insert_post(1, 2);
    insert_post(3, 1);
    insert_post(4, 1);
    insert_post(4, 2);
    insert_post(4, 3);
}

pub struct DataSources {
    user_loader: UserLoaderType,
    post_loader: PostLoaderType,
}
impl DataSources {
    pub fn new() -> DataSources {
        init_data();

        DataSources {
            user_loader: create_user_loader(),
            post_loader: create_post_loader(),
        }
    }

    pub async fn get_user(&self, id: i32) -> Option<User> {
        self.user_loader.load(id).await
    }

    pub fn get_users(&self) -> Option<Vec<User>> {
        let users = DB_USERS
            .lock()
            .unwrap()
            .values()
            .map(|u| User { data: u.clone() })
            .collect::<Vec<User>>();

        Some(users)
    }

    pub fn create_user(&self, input: UserInput) -> Option<User> {
        let next_id = DB_USERS.lock().unwrap().len() as i32 + 1;
        let data = DbUser {
            id: next_id,
            name: input.name,
        };

        let mut db = DB_USERS.lock().unwrap();
        db.insert(next_id, data.clone());

        return Some(User { data });
    }

    pub fn update_user(&self, id: i32, input: UserInput) -> User {
        // TODO: unwrapせずにResult型で返す
        let current_data = DB_USERS.lock().unwrap().get(&id).unwrap().clone();

        let new_data = DbUser {
            name: input.name,
            ..current_data
        };
        let mut db = DB_USERS.lock().unwrap();
        db.insert(id, new_data.clone());

        return User { data: new_data };
    }

    pub fn delete_user(&self, id: i32) -> i32 {
        // TODO: unwrapせずにResult型で返す
        let delete_data = DB_USERS.lock().unwrap().remove(&id).unwrap();
        let mut posts = DB_POSTS.lock().unwrap();
        posts.retain(|_, v| v.user_id != delete_data.id);

        return delete_data.id;
    }

    pub async fn get_posts_by_user_id(&self, id: i32) -> Vec<Post> {
        self.post_loader.load(id).await
    }

    pub fn get_posts(&self) -> Option<Vec<Post>> {
        let posts = DB_POSTS
            .lock()
            .unwrap()
            .values()
            .map(|p| Post { data: p.clone() })
            .collect::<Vec<Post>>();

        Some(posts)
    }

    pub fn create_post(&self, input: PostInput) -> Option<Post> {
        let next_id = DB_POSTS.lock().unwrap().len() as i32 + 1;
        let data = DbPost {
            id: next_id,
            user_id: input.user_id,
            title: input.title,
        };

        let mut db = DB_POSTS.lock().unwrap();
        db.insert(next_id, data.clone());

        return Some(Post { data });
    }

    pub fn update_post(&self, id: i32, input: PostInput) -> Post {
        // TODO: unwrapせずにResult型で返す
        let current_data = DB_POSTS.lock().unwrap().get(&id).unwrap().clone();

        let new_data = DbPost {
            title: input.title,
            ..current_data
        };
        let mut db = DB_POSTS.lock().unwrap();
        db.insert(id, new_data.clone());

        return Post { data: new_data };
    }

    pub fn delete_post(&self, id: i32) -> i32 {
        // TODO: unwrapせずにResult型で返す
        let delete_data = DB_POSTS.lock().unwrap().remove(&id).unwrap();

        return delete_data.id;
    }
}

pub struct UserLoader;
#[async_trait]
impl BatchFn<i32, Option<User>> for UserLoader {
    async fn load(&mut self, keys: &[i32]) -> HashMap<i32, Option<User>> {
        println!("fetch user_id = {:?}", keys);
        let mut hashmap: HashMap<i32, Option<User>> = HashMap::new();

        let db = DB_USERS.lock().unwrap();
        let fetch_data = db
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
pub fn create_user_loader() -> UserLoaderType {
    Loader::new(UserLoader).with_yield_count(100)
}

pub struct PostLoader;
#[async_trait]
impl BatchFn<i32, Vec<Post>> for PostLoader {
    async fn load(&mut self, keys: &[i32]) -> HashMap<i32, Vec<Post>> {
        println!("fetch post_id = {:?}", keys);
        let mut hashmap: HashMap<i32, Vec<Post>> = HashMap::new();

        let db = DB_POSTS.lock().unwrap();
        let fetch_data = db
            .values()
            .filter(|i| keys.contains(&i.user_id))
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
pub fn create_post_loader() -> PostLoaderType {
    Loader::new(PostLoader).with_yield_count(100)
}
