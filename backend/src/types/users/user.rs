use rocket::serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

#[derive(Deserialize, Serialize, Debug)]
#[derive(Clone)]
pub struct User {
    pub id: Thing,
    pub created_at: Datetime,
    pub name: String,
    pub email: String,
    pub password: String,
    pub storage_capacity: u64,
    pub jwt: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[derive(Clone)]
pub struct SafeUserData {
    pub id: Thing,
    pub created_at: Datetime,
    pub name: String,
    pub email: String,
    pub storage_capacity: u64,
}

impl User {
    pub fn get_id (& self) -> String {
        self.id.id.to_raw()
    }
}