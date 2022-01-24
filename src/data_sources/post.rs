use async_trait::async_trait;
use chrono::Utc;
use dataloader::non_cached::Loader;
use dataloader::BatchFn;
use mockall_double::double;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use std::collections::HashMap;
use std::sync::Arc;

use crate::data_sources::entities::{errors, post};
use crate::resolvers::objects::PostInput;

type Post = post::Model;
pub type PostSaveResult = Result<Result<Post, Vec<errors::ValidationError>>, DbErr>;
pub type PostDeleteResult = Result<Result<i32, Vec<errors::ValidationError>>, DbErr>;

mod mockable {
    use super::*;
    #[cfg(test)]
    use mockall::automock;

    pub struct Datasource {
        conn: Arc<DatabaseConnection>,
        loader: BatchLoaderType,
    }

    #[cfg_attr(test, automock)]
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
            let posts = post::Entity::find().all(self.conn.as_ref()).await?;

            let mut results: Vec<Post> = vec![];
            for item in posts {
                results.push(item);
            }

            Ok(results)
        }

        pub async fn create(&self, input: PostInput) -> PostSaveResult {
            let now = Utc::now().naive_utc();
            let build_res = post::ModelBuilder::new()
                .title(input.title)
                .user_id(input.user_id)
                .updated_at(now)
                .created_at(now)
                .build(self.conn.as_ref())
                .await?;

            match build_res {
                Ok(model) => Ok(Ok(model.insert(self.conn.as_ref()).await?)),
                Err(validation_err) => Ok(Err(validation_err)),
            }
        }

        pub async fn update(&self, id: i32, input: PostInput) -> PostSaveResult {
            let build_res = post::ModelBuilder::from_exists_data(self.conn.as_ref(), id)
                .await?
                .title(input.title)
                .user_id(input.user_id)
                // TODO: before_saveフックに実装
                .updated_at(Utc::now().naive_utc())
                .build(self.conn.as_ref())
                .await?;

            match build_res {
                Ok(model) => Ok(Ok(model.update(self.conn.as_ref()).await?)),
                Err(validation_err) => Ok(Err(validation_err)),
            }
        }

        pub async fn delete(&self, id: i32) -> PostDeleteResult {
            let build_res = post::ModelBuilder::from_exists_data(self.conn.as_ref(), id)
                .await?
                .build(self.conn.as_ref())
                .await?;

            match build_res {
                Ok(model) => {
                    model.delete(self.conn.as_ref()).await?;
                    Ok(Ok(id))
                }
                Err(validation_err) => Ok(Err(validation_err)),
            }
        }
    }
}

#[double]
pub use mockable::Datasource;

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
                        .map(|i| Ok(i.clone()))
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
