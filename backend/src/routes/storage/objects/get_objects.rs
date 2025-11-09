use std::sync::Arc;
use aws_sdk_s3::Client;
use rocket::{post, State};
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use surrealdb::engine::any::Any;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use crate::guards::auth_type::{AuthToken};
use crate::types::storage::object::ObjectType;

#[derive(Deserialize, Serialize, Debug)]
pub struct GetObjects {
    pub parent_id: Option<Thing>,
}

#[post("/storage/get_objects", data = "<data>")]
pub async fn get_objects (
    db: &State<Arc<Surreal<Any>>>,
    _client: &State<Arc<Client>>,
    data: Json<GetObjects>,
    auth_token: AuthToken,
) -> Result<(Status, Json<Vec<ObjectType>>), (Status, &'static str)> {

    let query = match data.parent_id {
        Some(_) => {
            "SELECT * FROM files where user_id = $user_id AND parent_id = $parent_id;"
        }
        None => {
            "SELECT * FROM files where user_id = $user_id AND parent_id.parent_id = none;"
        }
    };
    
    let mut resp = db
    .query(query)
    .bind(("user_id", Thing::from(("users", auth_token.0.sub.as_str()))))
    .bind(("parent_id", data.parent_id.clone()))
    .await
    .map_err(|e| {
        eprintln!("Failed to delete folder: {}", e);
        (Status::NotFound, "Object not found")
    })?;

    let objects:Vec<ObjectType> = match resp.take::<Vec<ObjectType>>(0) {
        Ok(data) => {
            data
        }
        Err(e) => {
            eprintln!("Failed to parse user: {}", e);
            return Err((Status::InternalServerError, "Database error while fetching user"));
        }
    };
    Ok((Status::Ok, Json(objects)))
}