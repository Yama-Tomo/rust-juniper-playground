mod entities;
mod post;
mod user;

pub use self::post::DbPost;
use self::post::DB_POSTS;
pub use self::user::DbUser;
use self::user::DB_USERS;

fn init_data() -> () {
    if DB_USERS.lock().unwrap().len() > 0 {
        return;
    }

    let items = ["Aron", "Bea", "carl", "Dora"];

    for (i, item) in items.iter().enumerate() {
        let id = i as i32 + 1;
        DB_USERS.lock().unwrap().insert(
            id,
            DbUser {
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
            DbPost {
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

pub struct DataSources {
    pub post: post::Datasource,
    pub user: user::Datasource,
}
impl DataSources {
    pub fn new() -> DataSources {
        init_data();

        DataSources {
            post: post::Datasource::new(),
            user: user::Datasource::new(),
        }
    }
}
