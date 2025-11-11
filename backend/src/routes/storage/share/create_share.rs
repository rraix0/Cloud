use std::sync::Arc;
use rocket::http::Status;
use rocket::{post, State};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use surrealdb::engine::any::Any;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use uuid::Uuid;
use crate::guards::auth_type::AuthToken;
use crate::types::storage::folder::Folder;
use crate::types::storage::object::ObjectType;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateShare {
    pub file_id: Option<Thing>,
    pub folder_id: Option<Thing>,
    pub permissions: u64,
    pub share_user_id: Option<Thing>,
    pub user_id: Thing
}

pub enum ShareReturn {
    ObjectType(ObjectType),
    Folder(Folder),
}
#[post("/storage/create_share", data = "<data>")]
pub async fn create_share(
    data: Json<CreateShare>,
    db: &State<Arc<Surreal<Any>>>,
    auth_token: AuthToken,
) -> Result<(Status, Json<String>), (Status, &'static str)> {
    /*
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




     */
    Ok((Status::Ok , Json("UGH".to_string())))
}