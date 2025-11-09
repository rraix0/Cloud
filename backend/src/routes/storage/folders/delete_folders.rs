use std::sync::Arc;
use rocket::{post, State};
use rocket::http::Status;
use rocket::serde::json::Json;
use surrealdb::engine::any::Any;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use crate::guards::auth_type::AuthToken;
use crate::types::storage::folder::{DeleteFolders, Folder};

#[post("/storage/delete_folders", data = "<data>")]
pub async fn delete_folders (
    data: Json<DeleteFolders>,
    db: &State<Arc<Surreal<Any>>>,
    auth_token: AuthToken,
) -> Result<(Status, Json<Vec<Thing>>), (Status, &'static str)> {

    let resp = db
        .query("{LET $TO_DELETE = select VALUE id FROM folders
        WHERE
        id in $folders_ids
        and parent_id != none
        and user_id = $user_id
        and id not IN array::distinct((SELECT parent_id FROM folders WHERE parent_id IN $folders_ids).map(|$x|  $x.parent_id))
        and id not IN array::distinct((SELECT parent_id FROM files WHERE parent_id IN $folders_ids).map(|$x|  $x.parent_id));

        DELETE FROM folders where id in $TO_DELETE;
        return $TO_DELETE;}
        ")
        .bind(("folders_ids", data.folders_ids.clone()))
      .bind(("user_id", Thing::from(("users", auth_token.0.sub.as_str()))))
        .await
        .map_err(|e| {
            eprintln!("Failed to delete folder: {}", e);
        });

    let response_object:Vec<Thing> = match resp {
        Ok(mut resp) => {
            match resp.take::<Vec<Thing>>(0){
                Ok(object) => {
                    println!("{:#?}", object);
                    object
                }
                Err(err) => {
                    println!("{:#?}", err);
                    return Err((Status::InternalServerError, "Occurred an error while renaming folder"));
                }
            }
        },
        Err(err) => {
            println!("{:#?}", err);
            return Err((Status::InternalServerError, "Occurred an error while renaming folder"));
        }
    };
    return Ok((Status::Ok, Json(response_object)));

}