use std::sync::Arc;
use rocket::{post, State};
use rocket::http::Status;
use rocket::serde::json::Json;
use surrealdb::engine::any::Any;
use surrealdb::sql::{Thing};
use surrealdb::Surreal;
use uuid::Uuid;
use crate::guards::auth_type::AuthToken;

#[post("/users/reset_token")]
pub async fn reset_token (
    db: &State<Arc<Surreal<Any>>>,
    auth_token: AuthToken,
) -> Result<(Status, Json<String>), (Status, &'static str)> {
    let _ = db
        .query("update users set jwt = $new_jwt where id = $user_id")
        .bind(("new_jwt", Uuid::now_v7()))
        .bind(("user_id", Thing::from(("users", auth_token.0.sub.as_str()))))
        .await
        .map_err(|e| {
            println!("{}", e);
            (Status::InternalServerError, "Could not edit user data")
        })?;

    Ok((Status::Ok, Json(String::from("User token has successfully reset."))))
}