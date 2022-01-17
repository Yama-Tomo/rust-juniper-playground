mod entities;
mod post;
mod user;

use self::post::DB_POSTS;
use self::user::DB_USERS;
pub use self::entities::public::*;
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::sync::Arc;

fn init_data() -> () {
    if DB_USERS.lock().unwrap().len() > 0 {
        return;
    }

    let items = ["Aron", "Bea", "carl", "Dora"];

    for (i, item) in items.iter().enumerate() {
        let id = i as i32 + 1;
        DB_USERS.lock().unwrap().insert(
            id,
            User {
                id,
                name: item.to_string(),
            },
        );
    }

    let mut id = 0;
    let mut insert_post = |user_id: i32, vol: i32| -> () {
        id = id + 1;
        DB_POSTS.lock().unwrap().insert(
            id,
            Post {
                id,
                user_id,
                title: format!(
                    "{} vol:{}",
                    DB_USERS.lock().unwrap().get(&user_id).unwrap().name,
                    vol
                ),
            },
        );
    };

    insert_post(1, 1);
    insert_post(1, 2);
    insert_post(3, 1);
    insert_post(4, 1);
    insert_post(4, 2);
    insert_post(4, 3);
}

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
        init_data();

        DataSources {
            post: post::Datasource::new(conn),
            user: user::Datasource::new(conn),
        }
    }
}
