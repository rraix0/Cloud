use std::sync::Arc;
use rocket::http::Status;
use rocket::{post, State};
use rocket::form::validate::Contains;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use surrealdb::engine::any::Any;
use surrealdb::sql::Thing;
use surrealdb::{Surreal};
use crate::guards::auth_type::AuthToken;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ReturnStructure {
  id: Thing,
  next: Option<Box<ReturnStructure>>,
}
#[derive(Deserialize, Serialize, Debug)]
#[derive(Clone)]
pub struct MoveFolders {
  pub folder_ids: Vec<Thing>,
  pub parent_id: Thing,
}
#[post("/storage/move_folders", data = "<data>")]
pub async fn move_folders(
  data: Json<MoveFolders>,
  db: &State<Arc<Surreal<Any>>>,
  auth_token: AuthToken,
) -> Result<(Status, Json<String>), (Status, &'static str)> {
  let mut result = db
    .query("select @.{1..256}.{id, next: parent_id.@} from $parent_id where user_id = $user_id;")
    .bind(("parent_id", data.parent_id.clone()))
    .bind(("user_id", Thing::from(("users", auth_token.0.sub.as_str()))))
    .await
    .map_err(|err| {
      eprintln!("{}", err);
      (Status::InternalServerError, "Internal server error")
    })?;

  let result: ReturnStructure = match result.take::<Vec<ReturnStructure>>(0) {
    Ok(value) => {
      value.into_iter().nth(0).unwrap()
    },
    Err(err) => {
      println!("Failed to move file: {:?}", err);
      return Err((Status::InternalServerError, "Occurred an error while moving folder"));
    }
  };

  let mut object_list:Vec<Thing> = Vec::new();
  let result: ReturnStructure = result;
  let mut current: &ReturnStructure = &result;

  loop {
    //println!("{:#?}", current);
    object_list.push(current.id.clone());
    if let Some(next) = current.next.as_deref() {
      current = next;
    } else {
      break;
    }
  }

  println!("{:#?}", object_list);

  if object_list.iter().any(|x| data.folder_ids.contains(x)) {
    println!("An idiot has been prevented from orphaning files. Moron's id btw: {}", auth_token.0.sub);
    return Err((Status::BadRequest, "You can't move folder to it self."));
  }



  let result = db
    .query("update folders set parent_id = $parent_id where id in $folder_ids and user_id = $user_id;")
    .bind(("parent_id", data.parent_id.clone()))
    .bind(("folder_ids", data.folder_ids.clone()))
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