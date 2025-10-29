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
pub struct MoveObject {
  pub object_ids: Vec<Thing>,
  pub parent_id: Thing,
}
#[post("/storage/move_objects", data = "<data>")]
pub async fn move_objects(
  data: Json<MoveObject>,
  db: &State<Arc<Surreal<Any>>>,
  auth_token: AuthToken,
) -> Result<(Status, Json<String>), (Status, &'static str)> {
  let result = db
    .query("update files set parent_id = $parent_id where id in $object_ids and user_id = $user_id;")
    .bind(("parent_id", data.parent_id.clone()))
    .bind(("object_ids", data.object_ids.clone()))
    .bind(("user_id", Thing::from(("users", auth_token.0.sub.as_str()))))
    .await
    .map_err(|err| {
      eprintln!("{}", err);
      (Status::InternalServerError, "Internal server error")
    });

  match result {
    Ok(_) => {
      return Ok((Status::Ok, Json("File has been moved.".to_string())));

    },
    Err(err) => {
      println!("Failed to move file: {:?}", err);
      return Err((Status::InternalServerError, "Occurred an error while moving folder"));
    }
  };
}