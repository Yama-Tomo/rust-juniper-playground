use async_trait::async_trait;
use chrono::Utc;
use dataloader::non_cached::Loader;
use dataloader::BatchFn;
use once_cell::sync::Lazy;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::data_sources::entities::user;
use crate::resolvers::objects::*;

type DbUsers = HashMap<i32, UserEntity>;

// TODO: RDBMSを使うようになったらonce_cellもアンインストール
pub static DB_USERS: Lazy<Mutex<DbUsers>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub struct Datasource {
    conn: Arc<DatabaseConnection>,
    loader: BatchLoaderType,
}
impl Datasource {
    pub fn new(conn: &Arc<DatabaseConnection>) -> Datasource {
        Datasource {
            conn: conn.clone(),
            loader: create_loader(conn.clone()),
        }
    }

    pub async fn get_by_id(&self, id: i32) -> Option<User> {
        self.loader.load(id).await
    }

    pub async fn get_all(&self) -> Option<Vec<User>> {
        let users = user::Entity::find().all(self.conn.as_ref()).await.unwrap();

        let mut results: Vec<User> = vec![];
        for item in users {
            results.push(User { data: item });
        }

        Some(results)
    }

    pub async fn create(&self, input: UserInput) -> Option<User> {
        let now = Utc::now().naive_utc();
        let data = user::ActiveModel {
            name: Set(input.name),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        // TODO: unwrapせずにResult型で返す
        let res = data.insert(self.conn.as_ref()).await.unwrap();

        Some(User { data: res })
    }

    pub async fn update(&self, id: i32, input: UserInput) -> User {
        // TODO: unwrapせずにResult型で返す
        let mut current_data: user::ActiveModel = user::Entity::find_by_id(id)
            .one(self.conn.as_ref())
            .await
            .expect("fetch current user data")
            .unwrap()
            .into();

        current_data.name = Set(input.name);
        // TODO: before_saveフックに実装
        current_data.updated_at = Set(Utc::now().naive_utc());
        let updated = current_data.update(self.conn.as_ref()).await;

        User {
            data: updated.unwrap(),
        }
    }

    pub async fn delete(&self, id: i32) -> i32 {
        // TODO: unwrapせずにResult型で返す
        let current_data: user::ActiveModel = user::Entity::find_by_id(id)
            .one(self.conn.as_ref())
            .await
            .expect("fetch current user data")
            .unwrap()
            .into();

        current_data
            .delete(self.conn.as_ref())
            .await
            .expect("remove user data");

        id
    }
}

struct UserLoader {
    conn: Arc<DatabaseConnection>,
}
#[async_trait]
impl BatchFn<i32, Option<User>> for UserLoader {
    async fn load(&mut self, keys: &[i32]) -> HashMap<i32, Option<User>> {
        println!("fetch user_id = {:?}", keys);
        let mut hashmap: HashMap<i32, Option<User>> = HashMap::new();

        let fetch_data = user::Entity::find()
            .filter(user::Column::Id.is_in(keys.to_vec()))
            .all(self.conn.as_ref())
            .await
            .unwrap();

        for key in keys {
            let data = match fetch_data.iter().find(|&i| &i.id == key) {
                Some(u) => Some(User { data: u.clone() }),
                None => None,
            };

            hashmap.insert(*key, data);
        }

        hashmap
    }
}

type BatchLoaderType = Loader<i32, Option<User>, UserLoader>;
fn create_loader(conn: Arc<DatabaseConnection>) -> BatchLoaderType {
    Loader::new(UserLoader { conn }).with_yield_count(100)
}
