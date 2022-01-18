use async_trait::async_trait;
use chrono::Utc;
use dataloader::non_cached::Loader;
use dataloader::BatchFn;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use std::collections::HashMap;
use std::sync::Arc;

use crate::data_sources::entities::{errors, user};
use crate::resolvers::objects::*;

pub type UserSaveResult = Result<Result<User, Vec<errors::ValidationError>>, DbErr>;
pub type UserDeleteResult = Result<Result<i32, Vec<errors::ValidationError>>, DbErr>;

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

    pub async fn create(&self, input: UserInput) -> UserSaveResult {
        let now = Utc::now().naive_utc();
        let build_res = user::ModelBuilder::new()
            .name(input.name)
            .updated_at(now)
            .created_at(now)
            .build();

        match build_res {
            Ok(build_res) => match build_res {
                Ok(model) => match model.insert(self.conn.as_ref()).await {
                    Ok(data) => Ok(Ok(User { data })),
                    Err(e) => Err(e),
                },
                Err(validation_err) => Ok(Err(validation_err)),
            },
            Err(e) => Err(e),
        }
    }

    pub async fn update(&self, id: i32, input: UserInput) -> UserSaveResult {
        let builder = user::ModelBuilder::from_exists_data(self.conn.as_ref(), id).await;
        match builder {
            Ok(builder) => match builder
                .name(input.name)
                // TODO: before_saveフックに実装
                .updated_at(Utc::now().naive_utc())
                .build()
            {
                Ok(build_res) => match build_res {
                    Ok(model) => match model.update(self.conn.as_ref()).await {
                        Ok(data) => Ok(Ok(User { data })),
                        Err(e) => Err(e),
                    },
                    Err(validation_err) => Ok(Err(validation_err)),
                },
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    pub async fn delete(&self, id: i32) -> UserDeleteResult {
        let builder = user::ModelBuilder::from_exists_data(self.conn.as_ref(), id).await;
        match builder {
            Ok(builder) => match builder.build() {
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
