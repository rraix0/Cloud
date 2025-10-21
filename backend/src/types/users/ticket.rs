use rocket::serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

#[derive(Deserialize, Serialize, Debug)]
pub struct Ticket {
    pub id: Thing,
    pub ticket: String,
    pub expire_at: Datetime,
    pub description: String,
    pub storage_capacity: i32,
}