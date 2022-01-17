mod entities;
mod post;
mod user;

pub use self::entities::public::*;
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::sync::Arc;

pub async fn create_db_connection() -> DatabaseConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Database::connect(database_url)
        .await
        .expect("create database connection")
}

pub struct DataSources {
    pub post: post::Datasource,
    pub user: user::Datasource,
}
impl DataSources {
    pub fn new(conn: &Arc<DatabaseConnection>) -> DataSources {
        DataSources {
            post: post::Datasource::new(conn),
            user: user::Datasource::new(conn),
        }
    }
}
