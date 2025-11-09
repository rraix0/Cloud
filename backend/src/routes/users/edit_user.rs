use std::sync::Arc;
use rocket::{post, State};
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use surrealdb::engine::any::Any;
use surrealdb::sql::{Thing};
use surrealdb::Surreal;
use crate::guards::auth_type::AuthToken;
use crate::types::users::user::SafeUserData;

#[derive(Deserialize, Serialize, Debug)]
pub struct EditUser {
    pub name: String,
}

#[post("/users/edit_user", data = "<data>")]
pub async fn edit_user (
    data: Json<EditUser>,
    db: &State<Arc<Surreal<Any>>>,
    auth_token: AuthToken,
) -> Result<(Status, Json<SafeUserData>), (Status, &'static str)> {
    if data.name.trim().len() == 0{
        return Err((Status::BadRequest, "Name cannot be empty"));
    }

    // Edit user

    let _ = db
        .query("update users set name = $new_name where id = $user_id")
        .bind(("new_name", data.name.clone()))
        .bind(("user_id", Thing::from(("users", auth_token.0.sub.as_str()))))
        .await
        .map_err(|e| {
            println!("{}", e);
            (Status::InternalServerError, "Could not edit user data")
        })?;

    // Get user info

    let mut result_folders = db
        .query("select created_at, email, id, name, storage_capacity from users")
        .bind(("user_id", Thing::from(("users", auth_token.0.sub.as_str()))))
        .await
        .map_err(|e| {
            println!("{}", e);
            (Status::InternalServerError, "Could not retrieve user info")
        })?;

    let user: SafeUserData = match result_folders.take::<Vec<SafeUserData>>(0) {
        Ok(folders) => folders.get(0).unwrap().clone(),
        Err(e) => {
            eprintln!("{}", e);
            return Err((Status::InternalServerError, "Couldn't retrieve user info"));
        }
    };

    // Return values
    Ok((Status::Ok, Json(user)))
}