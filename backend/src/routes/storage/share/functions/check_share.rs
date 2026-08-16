use std::sync::Arc;
use rocket::futures::StreamExt;
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use surrealdb::engine::any::Any;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use crate::types::share::shared::{CheckObjectShare, Shared};

pub async fn check_if_shared(
    db: &State<Arc<Surreal<Any>>>,
    data: CheckObjectShare
) -> Result<(), &'static str> {
    println!("{:?}", data);
    if !(data.link_token.is_some() ^ (data.share_user_id.is_some() && data.share_id.is_some())) {
        return Err("Token or shared user id is required.");
    }

    if data.object_id.is_none() && data.folder_id.is_none() {
        return Ok(());
    }



    // only token works rn
    if data.link_token.is_none() {
        return Err("Feature not finished.");
    }

    //get share
    let query = "select *, file_id.parent_id as parent_id from shared where link_token = $link_token";

    let mut resp = db
        .query(query)
        .bind(("link_token", data.link_token.clone()))
        .await
        .map_err(|e| {
            eprintln!("Failed to get share: {}", e);
            "Database error while fetching share"
        })?;

    let share:Vec<Shared> = match resp.take::<Vec<Shared>>(0) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to get share: {}", e);
            return Err("Database error while fetching share");
        }
    };

    let share = {
      if share.len() == 1 {
          share.first().cloned().unwrap()
      } else {
          return Err("Token is incorrect.")
      }
    };

    println!("{:?}", share);

    if share.folder_id.is_some() {
        if data.folder_id.is_some() {
            if share.folder_id == data.folder_id {
                return Ok(())
            }

            let result = check_parents(&db, data.folder_id.unwrap(), share.folder_id.unwrap()).await;
            match result {
                Ok(_) => {
                    return Ok(())
                }
                Err(error_message) => {
                    return Err(error_message);
                }
            }
        } else if data.object_id.is_some() {
            let query = "select VALUE parent_id from files where id = $object_id;";

            let mut resp = db
            .query(query)
                .bind(("object_id", data.object_id.clone()))
                .await
                .map_err(|e| {
                    eprintln!("Failed to get share: {}", e);
                    "Database error while fetching share"
                })?;
            let object_parent_id_vec:Vec<Thing> = match resp.take::<Vec<Thing>>(0) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Failed to get file: {}", e);
                    return Err("Database error while fetching object id");
                }
            };
            let object_parent_id = {
                if object_parent_id_vec.len() == 1 {
                    object_parent_id_vec.first().cloned().unwrap()
                } else {
                    return Err("No file has found.")
                }
            };

            let result = check_parents(&db, object_parent_id, share.folder_id.unwrap()).await;
            match result {
                Ok(_) => {
                    return Ok(())
                }
                Err(error_message) => {
                    return Err(error_message);
                }
            }
        }


    }

    if share.file_id.is_some() {
        if share.file_id == data.object_id {
            return Ok(())
        } else {
            return Err("Incorrect token.")
        }
    }
    Err("No matching results :(")
}

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
async fn check_parents (db: &Arc<Surreal<Any>>, folder_id: Thing, looking_for: Thing) -> Result<(), &'static str> {
    let query = "select @.{1..256}.{id, name, next:parent_id.@} from folders where id = $folder_id";

    let mut resp = db
        .query(query)
        .bind(("folder_id", folder_id))
        .await
        .map_err(|e| {
            eprintln!("Failed to get share: {}", e);
            "Database error while fetching share"
        })?;

    let path_structure:Vec<PathStructure> = match resp.take::<Vec<PathStructure>>(0) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to get share: {}", e);
            return Err("Database error while fetching share");
        }
    };

    let mut path_structure = {
        if path_structure.len() == 1 {
            path_structure.first().cloned().unwrap()
        } else {
             return Err("Could not resolve path")
        }
    };

    loop {
        if path_structure.id == looking_for {
            return Ok(())
        } else if let Some(parent) = path_structure.next.as_deref() {
            path_structure = parent.clone();
        } else {
            return Err("Token doesn't give you permission for that.");
        }
    }
}
