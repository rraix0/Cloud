use std::sync::{Arc, RwLock};
use once_cell::sync::Lazy;
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
    pub permissions: Vec<String>,
    pub share_user_id: Option<Thing>,
    pub user_id: Thing,
}

pub static PERMISSIONS: Lazy<RwLock<Arc<Vec<String>>>> = Lazy::new(|| {
    RwLock::new(Arc::new(vec![
        "read".to_string(),
        "write".to_string(),
        "delete".to_string(),
        "recursive".to_string(),
    ]))
});