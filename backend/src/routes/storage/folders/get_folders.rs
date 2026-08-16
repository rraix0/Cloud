use std::ops::Deref;
use std::sync::Arc;
use rocket::{post, State};
use rocket::http::Status;
use rocket::request::Outcome;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use surrealdb::engine::any::Any;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use crate::guards::auth_type::AuthToken;
use crate::routes::storage::share::functions::check_share::check_if_shared;
use crate::types::share::shared::CheckObjectShare;
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
    auth_token: Option<AuthToken>,
) -> Result<(Status, Json<(Option<Folder>, Vec<Folder>, Vec<PathObjectResult>)>), (Status, &'static str)> {

    // NORMAL

    let user_id: Option<Thing> = auth_token.map(|auth| Thing::from(("users", auth.0.sub.as_str())));
    let share_folder_id: Option<Thing> = match data.share_token {
        None => None,
        Some(_) => {
            let query = "select VALUE folder_id from shared where link_token = $link_token";
            let mut result_share_folder_id = db
                .query(query)
                .bind(("link_token", data.share_token.clone()))
                .await
                .map_err(|e| {
                    println!("{}", e);
                    (Status::InternalServerError, "could not retrieve folder")
                })?;
            let share_folder_id: Option<Thing> =result_share_folder_id.take::<Vec<Thing>>(0).ok().and_then(|folder| folder.first().cloned());
            if share_folder_id.is_none() {
                return Ok((Status::Ok, Json((None, Vec::new(), Vec::new()))));
            }
            share_folder_id
        }
    };


    // Get folders
    let query = match data.share_token.clone() {
        Some(share_token) => {
            // TODO check if even possible v



            match data.parent_id {
                Some(_) => {
                    let x = check_if_shared(
                        &db,
                        CheckObjectShare {
                            link_token: Some(share_token),
                            share_user_id: None,
                            share_id: None,
                            object_id: None,
                            folder_id: data.parent_id.clone()
                        }
                    ).await;
                    println!("SHARE CHECK: {:?}", x);
                    if let Err(err) = x {
                        return Err((Status::BadRequest, err));
                    }
                    "select * omit share_id from folders where parent_id = $parent_id"
                }
                None => {
                    "select * omit share_id  from folders where parent_id = (select VALUE folder_id.id from shared where link_token = $link_token).first();"
                }
            }
        }
        None => {
            if user_id.is_none() {
                return Err((Status::Unauthorized, "No jwt found or jwt incorrect"))
            }
            match data.parent_id {
                Some(_) => {
                    "select *, share_id.*.* as share from folders where parent_id = $parent_id and user_id = $user_id"
                }
                None => {
                    "select *, share_id.*.* as share from folders where
                     parent_id.parent_id = none
                     and parent_id != none
                     and user_id = $user_id"
                }
            }
        }
    };

/*
    let query = match data.parent_id {
        Some(_) => {
            "select *, share_id.*.* as share from folders where parent_id = $parent_id and user_id = $user_id"
        }
        None => {
            "select *, share_id.*.* as share from folders where
             parent_id.parent_id = none
             and parent_id != none
             and user_id = $user_id"
        }
    };


 */
    let mut result_folders = db
        .query(query)
        .bind(("parent_id", data.parent_id.clone()))
        .bind(("user_id", user_id.clone()))
        .bind(("link_token", data.share_token.clone()))
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
        .bind(("user_id", user_id.clone()))
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
    let query:Option<&str> = match data.share_token.clone() {
        Some(share_token) => {
            // TODO check if even possible v
            match data.parent_id {
                Some(_) => {
                    Some("select @.{1..256}.{id, name, next:parent_id.@} from folders where id = $folder_id;")
                }
                None => {
                    Some("select VALUE folder_id.{id, name} from shared where link_token = $link_token")
                    //None
                }
            }
        }
        None => {
            if user_id.is_none() {
                return Err((Status::Unauthorized, "No jwt found or jwt incorrect"))
            }
            match data.parent_id {
                Some(_) => {
                    Some("select @.{1..256}.{id, name, next:parent_id.@} from folders where user_id = $user_id and id = $folder_id;")
                }
                None => {
                    Some("select @.{1..256}.{id, name, next:parent_id.@} from folders where user_id = $user_id and parent_id = none;")
                }
            }
        }
    };
/*
    let query = match data.parent_id {
        Some(_) => {
            "select @.{1..256}.{id, name, next:parent_id.@} from folders where user_id = $user_id and id = $folder_id;"
        }
        None => {
            "select @.{1..256}.{id, name, next:parent_id.@} from folders where user_id = $user_id and parent_id = none;"
        }
    };

 */
    let mut object_list:Vec<PathObjectResult> = Vec::new();
    if let Some(query) = query {
        let mut result_path_directory = db
            .query(query)
            .bind(("folder_id", data.parent_id.clone()))
            .bind(("user_id", user_id.clone()))
            .bind(("link_token", data.share_token.clone()))
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
                println!("Failed to get path: {:?}", err);
                return Err((Status::InternalServerError, "Could not retrieve path directory"));
            }
        };


        let result: PathStructure = result;
        let mut current: &PathStructure = &result;

        println!("{:?}", share_folder_id);
        loop {
            object_list.push(PathObjectResult { id: current.id.clone(), name: current.name.clone()});
            if let Some(folder_id) = share_folder_id.clone() {
                if current.id == folder_id {
                    break
                }
            }

            println!("{:?}", object_list);
            if let Some(next) = current.next.as_deref() {
                current = next;
                /*
                if let Some(folder_id) = share_folder_id.clone() {
                    if next.id == folder_id {
                        break
                    }
                }

                 */
            } else {
                break;
            }
        }
    }

    // Return values
    Ok((Status::Ok, Json((parent_folder, folders, object_list))))
}