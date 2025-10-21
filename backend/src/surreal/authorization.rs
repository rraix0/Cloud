use std::sync::Arc;
use rocket::{State};
use rocket::http::Status;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use crate::types::auth_type::Authorization;
use crate::types::users::user::User;

pub async fn authorize(
    db: &State<Arc<Surreal<Any>>>,
    data: Authorization
) -> Result<bool, String> {
    let mut response = db
        .query(
            "SELECT * FROM users
         WHERE id = $user_id
         AND jwt = $jwt"
        )
        .bind(("user_id", data.get_thing().clone()))
        .bind(("jwt", data.jwt.clone()))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            format!("Database error: {}", e)
        })?;

    match response.take::<Option<User>>(0) {
        Ok(Some(_)) => Ok(true),
        Ok(None) => {
            Ok(false)
        }
        Err(e) => {
            eprintln!("{}", e);
            Err("Failed to retrieve user from the database.".to_string())
        }
    }
}


pub async fn handle_authorization(
    db: &State<Arc<Surreal<Any>>>,
    data: Authorization
) -> Result<(), (Status, &'static str)> {
    match authorize(db, data.clone()).await {
        Ok(value) => {
            if !value {
                return Err((Status::BadRequest, "Invalid authorization"))
            } else {
                Ok(())
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            return Err((Status::InternalServerError, "Internal server error"))
        }
    }
}
