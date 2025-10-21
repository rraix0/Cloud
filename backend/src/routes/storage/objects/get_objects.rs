use std::sync::Arc;
use aws_sdk_s3::Client;
use rocket::{post, State};
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use surrealdb::engine::any::Any;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use crate::surreal::authorization::handle_authorization;
use crate::types::auth_type::Authorization;
use crate::types::storage::object::ObjectType;

#[derive(Deserialize, Serialize, Debug)]
pub struct GetObjects {
    pub authorization: Authorization,
    pub parent_id: Thing,
}

#[post("/storage/get_objects", data = "<data>")]
pub async fn get_objects (
    db: &State<Arc<Surreal<Any>>>,
    _client: &State<Arc<Client>>,
    data: Json<GetObjects>
) -> Result<(Status, Json<Vec<ObjectType>>), (Status, &'static str)> {

    handle_authorization(db, data.authorization.clone()).await?;

    let mut resp = db
    .query("SELECT * FROM files where user_id = $user_id AND parent_id = $parent_id")
    .bind(("user_id", data.authorization.get_thing().clone()))
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