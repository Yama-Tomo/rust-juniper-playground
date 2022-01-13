use async_trait::async_trait;
use dataloader::non_cached::Loader;
use dataloader::BatchFn;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::resolvers::objects::*;

#[derive(Clone)]
pub struct DbPost {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
}

type DbPosts = HashMap<i32, DbPost>;

// TODO: RDBMSを使うようになったらonce_cellもアンインストール
pub static DB_POSTS: Lazy<Mutex<DbPosts>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub struct Datasource {
    loader: BatchLoaderType,
}
impl Datasource {
    pub fn new() -> Datasource {
        Datasource {
            loader: create_loader(),
        }
    }

    pub async fn get_posts_by_user_id(&self, id: i32) -> Vec<Post> {
        self.loader.load(id).await
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

struct PostLoader;
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

type BatchLoaderType = Loader<i32, Vec<Post>, PostLoader>;
fn create_loader() -> BatchLoaderType {
    Loader::new(PostLoader).with_yield_count(100)
}
