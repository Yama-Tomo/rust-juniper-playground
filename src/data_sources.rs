use std::collections::HashMap;

use crate::objects::User;

struct DbUser {
    pub id: i32,
    pub name: String,
}

type DbUsers = HashMap<i32, DbUser>;
#[derive(Default)]
pub struct Database {
    users: DbUsers,
}
impl Database {
    pub fn new() -> Database {
        let mut users: DbUsers = HashMap::new();
        let items = ["Aron", "Bea", "carl", "Dora"];

        for (i, item) in items.iter().enumerate() {
            let id = i as i32 + 1;
            users.insert(
                id,
                DbUser {
                    id,
                    name: item.to_string(),
                },
            );
        }

        Database { users }
    }

    pub fn get_user(&self, id: &i32) -> Option<User> {
        let user = self.users.get(id);
        match user {
            Some(u) => Some(User {
                id: u.id,
                name: u.name.to_string(),
            }),
            None => None,
        }
    }

    pub fn get_users(&self) -> Option<Vec<User>> {
        let users = self
            .users
            .values()
            .map(|u| User {
                id: u.id,
                name: u.name.to_string(),
            })
            .collect::<Vec<User>>();

        Some(users)
    }
}
