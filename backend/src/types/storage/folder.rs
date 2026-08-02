use rocket::serde::{Deserialize, Serialize};
use surrealdb::Datetime;
use surrealdb::sql::{Thing};
use crate::types::storage::share::ShareType;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Folder {
    pub created_at: Datetime,
    pub parent_id: Option<Thing>,
    pub id: Thing,
    pub name: String,
    pub share_id: Option<Vec<Thing>>,
    pub updated_at: Option<Datetime>,
    pub user_id: Thing,
    pub share: Option<Vec<ShareType>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateFolder {
    pub parent_id: Thing,
    pub name: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetFolders {
    pub parent_id: Option<Thing>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteFolders {
    pub folders_ids: Vec<Thing>,
}

#[derive(Deserialize, Serialize, Debug)]
#[derive(Clone)]
pub struct RenameFolder {
    pub folder_id: Thing,
    pub name: String,
}