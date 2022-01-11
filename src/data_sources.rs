use std::collections::HashMap;

use crate::objects::User;

#[derive(Default, Clone)]
pub struct Database {
    users: HashMap<i32, User>,
}
impl Database {
    pub fn new() -> Database {
        let mut users = HashMap::new();
        users.insert(
            1,
            User {
                id: 1,
                name: "Aron".to_string(),
            },
        );
        users.insert(
            2,
            User {
                id: 2,
                name: "Bea".to_string(),
            },
        );
        users.insert(
            3,
            User {
                id: 3,
                name: "Carl".to_string(),
            },
        );
        users.insert(
            4,
            User {
                id: 4,
                name: "Dora".to_string(),
            },
        );
        Database { users }
    }

    pub fn get_user(&self, id: &i32) -> Option<&User> {
        self.users.get(id)
    }

    pub fn get_users(&self) -> Option<Vec<&User>> {
        Some(self.users.values().collect::<Vec<&User>>())
    }
}
