use std::sync::Arc;
use rocket::{post, State};
use rocket::http::Status;
use rocket::serde::json::Json;
use surrealdb::engine::any::Any;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use crate::types::auth_type::AuthToken;
use crate::types::storage::folder::{Folder, GetFolders};

#[post("/storage/get_folders", data = "<data>")]
pub async fn get_folders (
    data: Json<GetFolders>,
    db: &State<Arc<Surreal<Any>>>,
    auth_token: AuthToken,
) -> Result<(Status, Json<Vec<Folder>>), (Status, &'static str)> {

    println!("{}", auth_token.0.sub);

    let mut result = db
        .query("select * from folders where parent_id = $parent_id and user_id = $user_id")
        .bind(("parent_id", data.parent_id.clone()))
        .bind(("user_id", Thing::from(("users", auth_token.0.sub.as_str()))))
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