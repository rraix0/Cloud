use std::sync::Arc;
use rocket::{post, State};
use rocket::form::validate::Len;
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use surrealdb::engine::any::Any;
use surrealdb::sql::{Datetime, Thing};
use surrealdb::{Response, Surreal};
use crate::guards::auth_type::AuthToken;
use crate::types::users::user::{SafeUserData, User};

#[derive(Deserialize, Serialize, Debug)]
pub struct ChangePassword {
    pub old_pass: String,
    pub new_pass: String,
}

#[post("/users/change_password", data = "<data>")]
pub async fn change_password (
    data: Json<ChangePassword>,
    db: &State<Arc<Surreal<Any>>>,
    auth_token: AuthToken,
) -> Result<(Status, Json<String>), (Status, &'static str)> {
    if data.new_pass.trim().len() < 8{
        return Err((Status::BadRequest, "Password must be at least 8 characters long"));
    }

    // Edit user

    let mut resp = db
        .query("update users set password = $new_pass where id = $user_id and password = $old_pass")
        .bind(("new_pass", data.new_pass.clone()))
        .bind(("old_pass", data.old_pass.clone()))
        .bind(("user_id", Thing::from(("users", auth_token.0.sub.as_str()))))
        .await
        .map_err(|e| {
            println!("{}", e);
            (Status::InternalServerError, "Could not edit user data")
        })?;

    match resp.take::<Vec<User>>(0) {
        Ok(user) => {
            match user.get(0) {
                None => {
                    Err((Status::BadRequest, "Old pass does not match."))
                }
                Some(x) => {
                    println!("Successfully updated user: {:?}", x);
                    Ok((Status::Ok, Json(String::from("Password has been successfully changed."))))

                }
            }
        }
        Err(err) => {
            eprintln!("{:?}", err);
            Err((Status::InternalServerError, "Occurred an error while trying to change your password."))
        }
    }

}