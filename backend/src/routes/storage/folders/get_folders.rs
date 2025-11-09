use std::sync::Arc;
use rocket::{post, State};
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use surrealdb::engine::any::Any;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use crate::guards::auth_type::AuthToken;
use crate::types::storage::folder::{Folder, GetFolders};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PathStructure {
    id: Thing,
    name: String,
    next: Option<Box<PathStructure>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PathObjectResult {
    id: Thing,
    name: String,
}

#[post("/storage/get_folders", data = "<data>")]
pub async fn get_folders (
    data: Json<GetFolders>,
    db: &State<Arc<Surreal<Any>>>,
    auth_token: AuthToken,
) -> Result<(Status, Json<(Option<Folder>, Vec<Folder>, Vec<PathObjectResult>)>), (Status, &'static str)> {

    println!("{}", auth_token.0.sub);

    // Get folders

    let query = match data.parent_id {
        Some(_) => {
            "select * from folders where parent_id = $parent_id and user_id = $user_id"
        }
        None => {
            "select * from folders where
             parent_id.parent_id = none
             and parent_id != none
             and user_id = $user_id"
        }
    };

    let mut result_folders = db
        .query(query)
        .bind(("parent_id", data.parent_id.clone()))
        .bind(("user_id", Thing::from(("users", auth_token.0.sub.as_str()))))
        .await
        .map_err(|e| {
            println!("{}", e);
            (Status::InternalServerError, "could not retrieve folder")
        })?;

    let folders = match result_folders.take::<Vec<Folder>>(0) {
        Ok(folders) => folders,
        Err(e) => {
            eprintln!("{}", e);
            return Err((Status::InternalServerError, "Couldn't get folders"))
        }
    };

    // Get parent

    let query = match data.parent_id {
        Some(_) => {
            "select * from folders where id = $id and user_id = $user_id"
        }
        None => {
            "select * from folders where parent_id = none and user_id = $user_id"
        }
    };

    let mut result_parent_folder = db
        .query(query)
        .bind(("id", data.parent_id.clone()))
        .bind(("user_id", Thing::from(("users", auth_token.0.sub.as_str()))))
        .await
        .map_err(|e| {
            println!("{}", e);
            (Status::InternalServerError, "could not retrieve folder")
        })?;

    let parent_folder:Option<Folder> = match result_parent_folder.take::<Vec<Folder>>(0) {
        Ok(folders) => folders.first().cloned(),
        Err(_) => return Err((Status::InternalServerError, "Couldn't get folders"))
    };

    // Get path directory

    let query = match data.parent_id {
        Some(_) => {
            "select @.{1..256}.{id, name, next:parent_id.@} from folders where user_id = $user_id and id = $folder_id;"
        }
        None => {
            "select @.{1..256}.{id, name, next:parent_id.@} from folders where user_id = $user_id and parent_id = none;"
        }
    };

    let mut result_path_directory = db
        .query(query)
        .bind(("folder_id", data.parent_id.clone()))
        .bind(("user_id", Thing::from(("users", auth_token.0.sub.as_str()))))
        .await
        .map_err(|e| {
            println!("{}", e);
            (Status::InternalServerError, "Could not retrieve path directory")
        })?;

    let result: PathStructure = match result_path_directory.take::<Vec<PathStructure>>(0) {
        Ok(value) => {
            value.into_iter().nth(0).unwrap()
        },
        Err(err) => {
            println!("Failed to move file: {:?}", err);
            return Err((Status::InternalServerError, "Occurred an error while moving folder"));
        }
    };

    let mut object_list:Vec<PathObjectResult> = Vec::new();
    let result: PathStructure = result;
    let mut current: &PathStructure = &result;

    loop {
        object_list.push(PathObjectResult { id: current.id.clone(), name: current.name.clone()});
        if let Some(next) = current.next.as_deref() {
            current = next;
        } else {
            break;
        }
    }

    // Return values
    Ok((Status::Ok, Json((parent_folder, folders, object_list))))
}