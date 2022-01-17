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

use crate::data_sources::entities::post;
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

    pub async fn get_by_user_id(&self, id: i32) -> Result<Vec<Post>, String> {
        self.loader.load(id).await
    }

    pub async fn get_all(&self) -> Result<Vec<Post>, DbErr> {
        let posts = post::Entity::find().all(self.conn.as_ref()).await;

        match posts {
            Ok(posts) => {
                let mut results: Vec<Post> = vec![];
                for item in posts {
                    results.push(Post { data: item });
                }

                Ok(results)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn create(&self, input: PostInput) -> Option<Post> {
        let now = Utc::now().naive_utc();
        let data = post::ActiveModel {
            title: Set(input.title),
            user_id: Set(input.user_id),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        // TODO: unwrapせずにResult型で返す
        let res = data.insert(self.conn.as_ref()).await.unwrap();

        Some(Post { data: res })
    }

    pub async fn update(&self, id: i32, input: PostInput) -> Post {
        // TODO: unwrapせずにResult型で返す
        let mut current_data: post::ActiveModel = post::Entity::find_by_id(id)
            .one(self.conn.as_ref())
            .await
            .expect("fetch current post data")
            .unwrap()
            .into();

        current_data.title = Set(input.title);
        // TODO: before_saveフックに実装
        current_data.updated_at = Set(Utc::now().naive_utc());
        let updated = current_data.update(self.conn.as_ref()).await;

        Post {
            data: updated.unwrap(),
        }
    }

    pub async fn delete(&self, id: i32) -> i32 {
        // TODO: unwrapせずにResult型で返す
        let current_data: post::ActiveModel = post::Entity::find_by_id(id)
            .one(self.conn.as_ref())
            .await
            .expect("fetch current post data")
            .unwrap()
            .into();

        current_data
            .delete(self.conn.as_ref())
            .await
            .expect("remove post data");

        id
    }
}

struct PostLoader {
    conn: Arc<DatabaseConnection>,
}
#[async_trait]
impl BatchFn<i32, Result<Vec<Post>, String>> for PostLoader {
    async fn load(&mut self, keys: &[i32]) -> HashMap<i32, Result<Vec<Post>, String>> {
        let fetch_data = post::Entity::find()
            .filter(post::Column::UserId.is_in(keys.to_vec()))
            .all(self.conn.as_ref())
            .await;

        match fetch_data {
            Ok(fetch_data) => {
                let mut hashmap: HashMap<i32, Result<Vec<Post>, String>> = HashMap::new();
                for key in keys {
                    let posts = fetch_data
                        .iter()
                        .filter(|&i| &i.user_id == key)
                        .map(|i| Ok(Post { data: i.clone() }))
                        .collect::<Result<Vec<Post>, String>>();

                    hashmap.insert(*key, posts);
                }

                hashmap
            }
            Err(db_err) => keys.iter().map(|k| (*k, Err(db_err.to_string()))).collect(),
        }
    }
}

type BatchLoaderType = Loader<i32, Result<Vec<Post>, String>, PostLoader>;
fn create_loader(conn: Arc<DatabaseConnection>) -> BatchLoaderType {
    Loader::new(PostLoader { conn }).with_yield_count(100)
}
