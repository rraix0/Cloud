use std::sync::Arc;
use rocket::http::Status;
use rocket::{post, State};
use rocket::serde::json::Json;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use uuid::Uuid;
use crate::types::storage::folder::{CreateFolder, Folder};
use crate::surreal::authorization::{handle_authorization};

#[post("/storage/create_folder", data = "<data>")]
pub async fn create_folder(
    data: Json<CreateFolder>,
    db: &State<Arc<Surreal<Any>>>
) -> Result<(Status, Json<Folder>), (Status, &'static str)> {
    handle_authorization(db, data.authorization.clone()).await?;

    println!("{}     {}", data.authorization.jwt.clone(), data.authorization.user_id.clone());

    let result = db
        .query("insert into folders (id, created_at, parent_id, name, user_id)
values ($id, time::now(), $parent_id, $name, $user_id);")
        .bind(("id", Uuid::now_v7().to_string()))
        .bind(("parent_id", data.parent_id.clone()))
        .bind(("name", data.name.clone()))
        .bind(("user_id", data.authorization.get_thing()))
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