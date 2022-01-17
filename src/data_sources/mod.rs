mod entities;
mod post;
mod user;

pub use self::entities::public::*;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use std::sync::Arc;

pub async fn create_db_connection() -> DatabaseConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(10).sqlx_logging(false);

    Database::connect(opt)
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
