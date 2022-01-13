use async_trait::async_trait;
use dataloader::non_cached::Loader;
use dataloader::BatchFn;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::data_sources::entities::User as UserEntity;
use crate::data_sources::post::DB_POSTS;
use crate::resolvers::objects::*;

type DbUsers = HashMap<i32, UserEntity>;

// TODO: RDBMSを使うようになったらonce_cellもアンインストール
pub static DB_USERS: Lazy<Mutex<DbUsers>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub struct Datasource {
    loader: BatchLoaderType,
}
impl Datasource {
    pub fn new() -> Datasource {
        Datasource {
            loader: create_loader(),
        }
    }

    pub async fn get_by_id(&self, id: i32) -> Option<User> {
        self.loader.load(id).await
    }

    pub fn get_all(&self) -> Option<Vec<User>> {
        let users = DB_USERS
            .lock()
            .unwrap()
            .values()
            .map(|u| User { data: u.clone() })
            .collect::<Vec<User>>();

        Some(users)
    }

    pub fn create(&self, input: UserInput) -> Option<User> {
        let next_id = DB_USERS.lock().unwrap().len() as i32 + 1;
        let data = UserEntity {
            id: next_id,
            name: input.name,
        };

        let mut db = DB_USERS.lock().unwrap();
        db.insert(next_id, data.clone());

        return Some(User { data });
    }

    pub fn update(&self, id: i32, input: UserInput) -> User {
        // TODO: unwrapせずにResult型で返す
        let current_data = DB_USERS.lock().unwrap().get(&id).unwrap().clone();

        let new_data = UserEntity {
            name: input.name,
            ..current_data
        };
        let mut db = DB_USERS.lock().unwrap();
        db.insert(id, new_data.clone());

        return User { data: new_data };
    }

    pub fn delete(&self, id: i32) -> i32 {
        // TODO: unwrapせずにResult型で返す
        let delete_data = DB_USERS.lock().unwrap().remove(&id).unwrap();
        let mut posts = DB_POSTS.lock().unwrap();
        posts.retain(|_, v| v.user_id != delete_data.id);

        return delete_data.id;
    }
}

struct UserLoader;
#[async_trait]
impl BatchFn<i32, Option<User>> for UserLoader {
    async fn load(&mut self, keys: &[i32]) -> HashMap<i32, Option<User>> {
        println!("fetch user_id = {:?}", keys);
        let mut hashmap: HashMap<i32, Option<User>> = HashMap::new();

        let db = DB_USERS.lock().unwrap();
        let fetch_data = db
            .values()
            .filter(|i| keys.contains(&i.id))
            .collect::<Vec<&UserEntity>>();

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

type BatchLoaderType = Loader<i32, Option<User>, UserLoader>;
fn create_loader() -> BatchLoaderType {
    Loader::new(UserLoader).with_yield_count(100)
}
