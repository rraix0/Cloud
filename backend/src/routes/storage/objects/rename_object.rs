use std::sync::Arc;
use rocket::http::Status;
use rocket::{post, State};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use surrealdb::engine::any::Any;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use crate::types::auth_type::AuthToken;

#[derive(Deserialize, Serialize, Debug)]
#[derive(Clone)]
pub struct RenameObject {
    pub object_id: Thing,
    pub name: String,
}
#[post("/storage/rename_object", data = "<data>")]
pub async fn rename_object(
    data: Json<RenameObject>,
    db: &State<Arc<Surreal<Any>>>,
    auth_token: AuthToken,
) -> Result<(Status, Json<String>), (Status, &'static str)> {
    if !(data.clone().name.trim().len() > 0) {
        return Err((Status::BadRequest, "No file name specified"))
    }
    let result = db
        .query("update files set name = $name where id = $object_id and user_id = $user_id;")
        .bind(("name", data.name.clone()))
        .bind(("object_id", data.object_id.clone()))
        .bind(("user_id", Thing::from(("users", auth_token.0.sub.as_str()))))
        .await
        .map_err(|err| {
            eprintln!("{}", err);
            (Status::InternalServerError, "Internal server error")
        });

    match result {
        Ok(_) => {
            return Ok((Status::Ok, Json("Folder has been renamed.".to_string())));

        },
        Err(err) => {
            println!("Failed to create folder: {:?}", err);
            return Err((Status::InternalServerError, "Problem creating folder"));
        }
    };
}