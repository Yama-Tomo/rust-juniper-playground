use async_trait::async_trait;
use chrono::Utc;
use dataloader::non_cached::Loader;
use dataloader::BatchFn;
use mockall_double::double;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use std::collections::HashMap;
use std::sync::Arc;

use crate::data_sources::entities::{errors, user};
use crate::resolvers::objects::UserInput;

type User = user::Model;
pub type UserSaveResult = Result<Result<User, Vec<errors::ValidationError>>, DbErr>;
pub type UserDeleteResult = Result<Result<i32, Vec<errors::ValidationError>>, DbErr>;

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
        pub fn new(conn: &Arc<DatabaseConnection>) -> Self {
            Self {
                conn: Arc::clone(conn),
                loader: create_loader(Arc::clone(conn)),
            }
        }

        pub async fn get_by_id(&self, id: i32) -> Result<Option<User>, String> {
            self.loader.load(id).await
        }

        pub async fn get_all(&self) -> Result<Vec<User>, DbErr> {
            let users = user::Entity::find().all(self.conn.as_ref()).await?;

            let mut results: Vec<User> = vec![];
            for item in users {
                results.push(item);
            }

            Ok(results)
        }

        pub async fn create(&self, input: UserInput) -> UserSaveResult {
            let now = Utc::now().naive_utc();
            let build_res = user::ModelBuilder::new()
                .name(input.name)
                .updated_at(now)
                .created_at(now)
                .build()?;

            match build_res {
                Ok(model) => Ok(Ok(model.insert(self.conn.as_ref()).await?)),
                Err(validation_err) => Ok(Err(validation_err)),
            }
        }

        pub async fn update(&self, id: i32, input: UserInput) -> UserSaveResult {
            let build_res = user::ModelBuilder::from_exists_data(self.conn.as_ref(), id)
                .await?
                .name(input.name)
                // TODO: before_saveフックに実装
                .updated_at(Utc::now().naive_utc())
                .build()?;

            match build_res {
                Ok(model) => Ok(Ok(model.update(self.conn.as_ref()).await?)),
                Err(validation_err) => Ok(Err(validation_err)),
            }
        }

        pub async fn delete(&self, id: i32) -> UserDeleteResult {
            let build_res = user::ModelBuilder::from_exists_data(self.conn.as_ref(), id)
                .await?
                .build()?;

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
                    .map(|user| (user.id, Ok(Some(user))))
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
