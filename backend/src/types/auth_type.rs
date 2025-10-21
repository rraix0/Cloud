use rocket::FromForm;
use rocket::serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing};

#[derive(Deserialize, Serialize, Debug, Clone, FromForm)]
pub struct Authorization {
    pub user_id: String,
    pub jwt: String,
}

impl Authorization {
    pub fn get_thing(& self) -> Thing {
        Thing::from(("users", self.user_id.as_str()))
    }
}