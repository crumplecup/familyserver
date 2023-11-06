use fake::Dummy;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub enum OneOrMore {
    One(String),
    More(Vec<String>),
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Dummy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct User {
    id: Uuid,
    username: String,
    password_hash: String,
}

impl User {
    pub fn new(username: &str, password: &str) -> Self {
        let id = uuid::Uuid::new_v4();
        User {
            id,
            username: username.to_owned(),
            password_hash: password.to_owned(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id.clone()
    }

    pub fn id_ref(&self) -> &Uuid {
        &self.id
    }

    pub fn set_id(&mut self, value: Uuid) {
        self.id = value;
    }

    pub fn username(&self) -> String {
        self.username.clone()
    }

    pub fn username_ref(&self) -> &String {
        &self.username
    }

    pub fn password_hash(&self) -> String {
        self.password_hash.clone()
    }

    pub fn password_hash_ref(&self) -> &String {
        &self.password_hash
    }
}

pub struct Family {
    family_id: i32,
    name: String,
}

pub struct Identity {
    user_id: i32,
    first_name: String,
    last_name: Option<String>,
    middle_names: Option<OneOrMore>,
    family_id: Option<i32>,
    birthday: Option<String>,
}

pub struct Health {
    user_id: i32,
    age: Option<i32>,
    height_m: Option<i32>,
    weight_kg: Option<i32>,
}
