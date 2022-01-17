use async_trait::async_trait;
use chrono::Utc;
use dataloader::non_cached::Loader;
use dataloader::BatchFn;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter,
};
use std::collections::HashMap;
use std::sync::Arc;

use crate::data_sources::entities::user;
use crate::resolvers::objects::*;

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

    pub async fn get_by_id(&self, id: i32) -> Result<Option<User>, String> {
        self.loader.load(id).await
    }

    pub async fn get_all(&self) -> Result<Vec<User>, DbErr> {
        let users = user::Entity::find().all(self.conn.as_ref()).await;
        match users {
            Ok(users) => {
                let mut results: Vec<User> = vec![];
                for item in users {
                    results.push(User { data: item });
                }

                Ok(results)
            }
            Err(e) => Err(e),
        }
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
impl BatchFn<i32, Result<Option<User>, String>> for UserLoader {
    async fn load(&mut self, keys: &[i32]) -> HashMap<i32, Result<Option<User>, String>> {
        let fetch_data = user::Entity::find()
            .filter(user::Column::Id.is_in(keys.to_vec()))
            .all(self.conn.as_ref())
            .await;

        match fetch_data {
            Ok(fetch_data) => {
                let hashmap = fetch_data
                    .into_iter()
                    .map(|user| (user.id, Ok(Some(User { data: user }))))
                    .collect::<HashMap<i32, Result<Option<User>, String>>>();

                keys.iter().fold(hashmap, |mut map, key| {
                    map.entry(*key).or_insert(Ok(None));
                    map
                })
            }
            Err(db_err) => keys.iter().map(|k| (*k, Err(db_err.to_string()))).collect(),
        }
    }
}

type BatchLoaderType = Loader<i32, Result<Option<User>, String>, UserLoader>;
fn create_loader(conn: Arc<DatabaseConnection>) -> BatchLoaderType {
    Loader::new(UserLoader { conn }).with_yield_count(100)
}
