use rocket::serde::{Deserialize, Serialize};
use surrealdb::Datetime;
use surrealdb::sql::{Bytes, Object, Thing};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Shared {
    pub expiration_date: Datetime,
    pub file_id: Option<Thing>,
    pub folder_id: Option<Thing>,
    pub id: Thing,
    pub link_token: Option<String>,
    pub permission: u64,
    pub share_eser_id: Option<Thing>,
    pub user_id: Thing,
}