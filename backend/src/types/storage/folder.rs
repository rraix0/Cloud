use crate::types::auth_type::Authorization;
use rocket::serde::{Deserialize, Serialize};
use surrealdb::Datetime;
use surrealdb::sql::{Thing};

#[derive(Deserialize, Serialize, Debug)]
pub struct Folder {
    created_at: Datetime,
    parent_id: Option<Thing>,
    id: Thing,
    name: String,
    share_id: Option<Vec<Thing>>,
    updated_at: Option<Datetime>,
    user_id: Thing,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateFolder {
    pub authorization: Authorization,
    pub parent_id: Thing,
    pub name: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetFolders {
    pub authorization: Authorization,
    pub parent_id: Option<Thing>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteFolders {
    pub authorization: Authorization,
    pub folders_ids: Vec<Thing>,
}