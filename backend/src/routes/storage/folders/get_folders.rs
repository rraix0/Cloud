use std::sync::Arc;
use rocket::{post, State};
use rocket::http::Status;
use rocket::serde::json::Json;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use crate::surreal::authorization::{handle_authorization};
use crate::types::storage::folder::{Folder, GetFolders};

#[post("/storage/get_folders", data = "<data>")]
pub async fn get_folders (
    data: Json<GetFolders>,
    db: &State<Arc<Surreal<Any>>>
) -> Result<(Status, Json<Vec<Folder>>), (Status, &'static str)> {

  handle_authorization(db, data.authorization.clone()).await?;

    let mut result = db
        .query("select * from folders where parent_id = $parent_id")
        .bind(("parent_id", data.parent_id.clone()))
        .await
        .map_err(|e| {
            println!("{}", e);
            (Status::InternalServerError, "could not retrieve folder")
        })?;

    let folders = match result.take::<Vec<Folder>>(0) {
        Ok(folders) => folders,
        Err(e) => {
            eprintln!("{}", e);
            return Err((Status::InternalServerError, "Couldn't get folders"))
        }
    };
    println!("{:?}", folders.get(0));

    Ok((Status::Ok, Json(folders)))
}