use rocket::serde::{Deserialize, Serialize};
use surrealdb::Datetime;
use surrealdb::sql::{Bytes, Object, Thing};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Shared {
    pub description: Option<String>,
    pub expiration_date: Datetime,
    pub file_id: Option<Thing>,
    pub folder_id: Option<Thing>,
    pub id: Thing,
    pub link_token: Option<String>,
    pub permissions: u64,
    pub share_user_id: Option<Thing>,
    pub user_id: Thing,
}