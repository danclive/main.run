use mon::oid::ObjectId;

use chrono::Utc;

use super::User;
use super::Role;

impl User {
    pub fn new_empty() -> User {
        User {
            id: ObjectId::new().unwrap(),
            username: "".to_owned(),
            avatar: "".to_owned(),
            role: Role::Admin,
            password: vec![1, 2, 3, 4],
            create_at: Utc::now().into(),
            update_at: Utc::now().into()
        }
    }
}
