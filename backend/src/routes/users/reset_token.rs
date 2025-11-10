use std::sync::Arc;
use rocket::{post, State};
use rocket::http::Status;
use rocket::serde::json::Json;
use surrealdb::engine::any::Any;
use surrealdb::sql::{Thing};
use surrealdb::Surreal;
use uuid::Uuid;
use crate::guards::auth_type::AuthToken;
use crate::types::users::user::User;

#[post("/users/reset_token")]
pub async fn reset_token (
    db: &State<Arc<Surreal<Any>>>,
    auth_token: AuthToken,
) -> Result<(Status, Json<String>), (Status, &'static str)> {
    let mut resp = db
        .query("update users set jwt = $new_jwt where id = $user_id")
        .bind(("new_jwt", Uuid::new_v4().to_string()))
        .bind(("user_id", Thing::from(("users", auth_token.0.sub.as_str()))))
        .await
        .map_err(|e| {
            println!("{}", e);
            (Status::InternalServerError, "Could not reset token")
        })?;

    match resp.take::<Vec<User>>(0) {
        Ok(user) => {
            match user.get(0) {
                Some(_) => {
                    return Ok((Status::Ok, Json(String::from("User token has successfully reset."))))
                }
                None => {
                    return Err((Status::InternalServerError, "Could not reset token"));
                }
            }
        }
        Err(err) => {
            println!("Occurred an error while trying to reset user's token: {}", err);
            return Err((Status::InternalServerError, "Could not reset token"));
        }
    }
}