use async_trait::async_trait;
use chrono::Utc;
use dataloader::non_cached::Loader;
use dataloader::BatchFn;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use std::collections::HashMap;
use std::sync::Arc;

use crate::data_sources::entities::{errors, post};
use crate::resolvers::objects::*;

pub type PostSaveResult = Result<Result<Post, Vec<errors::ValidationError>>, DbErr>;
pub type PostDeleteResult = Result<Result<i32, Vec<errors::ValidationError>>, DbErr>;

pub struct Datasource {
    conn: Arc<DatabaseConnection>,
    loader: BatchLoaderType,
}
impl Datasource {
    pub fn new(conn: &Arc<DatabaseConnection>) -> Datasource {
        Datasource {
            conn: Arc::clone(conn),
            loader: create_loader(Arc::clone(conn)),
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

    pub async fn create(&self, input: PostInput) -> PostSaveResult {
        let now = Utc::now().naive_utc();
        let build_res = post::ModelBuilder::new()
            .title(input.title)
            .user_id(input.user_id)
            .updated_at(now)
            .created_at(now)
            .build(self.conn.as_ref())
            .await;

        match build_res {
            Ok(build_res) => match build_res {
                Ok(model) => match model.insert(self.conn.as_ref()).await {
                    Ok(data) => Ok(Ok(Post { data })),
                    Err(e) => Err(e),
                },
                Err(validation_err) => Ok(Err(validation_err)),
            },
            Err(e) => Err(e),
        }
    }

    pub async fn update(&self, id: i32, input: PostInput) -> PostSaveResult {
        let builder = post::ModelBuilder::from_exists_data(self.conn.as_ref(), id).await;
        match builder {
            Ok(builder) => match builder
                .title(input.title)
                .user_id(input.user_id)
                // TODO: before_saveフックに実装
                .updated_at(Utc::now().naive_utc())
                .build(self.conn.as_ref())
                .await
            {
                Ok(build_res) => match build_res {
                    Ok(model) => match model.update(self.conn.as_ref()).await {
                        Ok(data) => Ok(Ok(Post { data })),
                        Err(e) => Err(e),
                    },
                    Err(validation_err) => Ok(Err(validation_err)),
                },
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    pub async fn delete(&self, id: i32) -> PostDeleteResult {
        let builder = post::ModelBuilder::from_exists_data(self.conn.as_ref(), id).await;
        match builder {
            Ok(builder) => match builder.build(self.conn.as_ref()).await {
                Ok(build_res) => match build_res {
                    Ok(model) => match model.delete(self.conn.as_ref()).await {
                        Ok(_) => Ok(Ok(id)),
                        Err(e) => Err(e),
                    },
                    Err(validation_err) => Ok(Err(validation_err)),
                },
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
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
