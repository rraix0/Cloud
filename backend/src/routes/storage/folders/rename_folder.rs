use std::env;
use std::sync::Arc;
use rocket::http::Status;
use rocket::{post, State};
use rocket::serde::json::Json;
use surrealdb::engine::any::Any;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use uuid::Uuid;
use crate::types::storage::folder::{CreateFolder, Folder, RenameFolder};
use crate::types::auth_type::AuthToken;
use crate::types::storage::object::ObjectType;

#[post("/storage/rename_folder", data = "<data>")]
pub async fn rename_folder(
  data: Json<RenameFolder>,
  db: &State<Arc<Surreal<Any>>>,
  auth_token: AuthToken,
) -> Result<(Status, Json<Folder>), (Status, &'static str)> {
  if !(data.clone().name.trim().len() > 0) {
    return Err((Status::BadRequest, "No file name specified"))
  }
  let resp = db
    .query("update folders set name = $name where id = $folder_id and user_id = $user_id;")
    .bind(("name", data.name.clone()))
    .bind(("folder_id", data.folder_id.clone()))
    .bind(("user_id", Thing::from(("users", auth_token.0.sub.as_str()))))
    .await
    .map_err(|err| {
      eprintln!("{}", err);
      (Status::InternalServerError, "Internal server error")
    });

  let response_object:Folder = match resp {
    Ok(mut resp) => {
      match resp.take::<Vec<Folder>>(0){
        Ok(object) => {
          if let Some(object) = object.get(0) {
            object.clone()
          } else {
            return Err((Status::InternalServerError, "Occurred an error while renaming folder"));
          }
        }
        Err(_) => {
          return Err((Status::InternalServerError, "Occurred an error while renaming folder"));
        }
      }
    },
    Err(err) => {
      return Err((Status::InternalServerError, "Occurred an error while renaming folder"));
    }
  };
  return Ok((Status::Ok, Json(response_object)));
}