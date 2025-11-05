use rocket::serde::{Deserialize, Serialize};
use surrealdb::Datetime;
use surrealdb::sql::{Bytes, Object, Thing};

#[derive(Deserialize, Serialize, Debug)]
pub struct ObjectType {
    pub created_at: Datetime,
    pub id: Thing,
    pub size: u64,
    pub metadata: Option<Object>,
    pub name: String,
    pub parent_id: Thing,
    pub share_id: Option<Vec<Thing>>,
    pub tags: Option<Vec<String>>,
    pub updated_at: Option<Datetime>,
    pub user_id: Thing,
    pub thumbnail: Option<Bytes>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UploadObject {
    pub name: String,
    pub length: i64,
}