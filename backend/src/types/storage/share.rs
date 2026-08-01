use rocket::serde::{Deserialize, Serialize};
use surrealdb::Datetime;
use surrealdb::sql::Thing;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ShareType {
    pub id: Thing,
    pub description: Option<String>,
    pub expiration_date: Option<Datetime>,
    pub file_id: Option<Thing>,
    pub folder_id: Option<Thing>,
    pub link_token: Option<String>,
    pub permissions: Vec<String>,
    pub share_user_id: Option<Vec<Thing>>,
    pub user_id: Thing,
}