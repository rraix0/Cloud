use std::sync::Arc;
use rocket::http::Status;
use rocket::{post, State};
use rocket::serde::json::Json;
use surrealdb::engine::any::Any;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use uuid::Uuid;
use crate::types::storage::folder::{CreateFolder, Folder};
use crate::types::auth_type::AuthToken;

#[post("/storage/create_folder", data = "<data>")]
pub async fn create_folder(
    data: Json<CreateFolder>,
    db: &State<Arc<Surreal<Any>>>,
    auth_token: AuthToken,
) -> Result<(Status, Json<Folder>), (Status, &'static str)> {
    if !(data.clone().name.trim().len() > 0) {
        return Err((Status::BadRequest, "No file name specified"))
    }
    let result = db
        .query("insert into folders (id, created_at, parent_id, name, user_id)
values ($id, time::now(), $parent_id, $name, $user_id);")
        .bind(("id", Uuid::now_v7().to_string()))
        .bind(("parent_id", data.parent_id.clone()))
        .bind(("name", data.name.clone()))
        .bind(("user_id", Thing::from(("users", auth_token.0.sub.as_str()))))
        .await
        .map_err(|err| {
            eprintln!("{}", err);
            (Status::InternalServerError, "Internal server error")
        });

    let folder: Folder = match result {
        Ok(mut result) => {

            let folders: Vec<Folder> = result.take(0).unwrap();
            let folder: Folder = folders.into_iter().next().unwrap();

            println!("{:#?}", folder);
            println!("Successfully created folder {:?}", result);
            folder
        },
        Err(err) => {
            println!("Failed to create folder: {:?}", err);
            return Err((Status::InternalServerError, "Problem creating folder"));
        }
    };

    Ok((Status::Ok , Json(folder)))
}