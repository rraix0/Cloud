use std::sync::Arc;
use rocket::{post, State};
use rocket::http::Status;
use rocket::serde::json::Json;
use surrealdb::engine::any::Any;
use surrealdb::sql::{Thing};
use surrealdb::Surreal;
use crate::guards::auth_type::AuthToken;
use crate::types::users::user::SafeUserData;

#[post("/users/get_user_info")]
pub async fn get_user_info (
    db: &State<Arc<Surreal<Any>>>,
    auth_token: AuthToken,
) -> Result<(Status, Json<SafeUserData>), (Status, &'static str)> {

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