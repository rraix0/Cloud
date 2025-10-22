use std::env;
use std::sync::Arc;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use rocket::http::Status;
use rocket::{post, State};
use rocket::serde::json::Json;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use crate::types::users::user::User;
use crate::types::users::login_type::{
    UserLoginRequest,
};

use crate::types::auth_type::Claims;

#[post("/users/login", data = "<account>")]
pub async fn login(
    account: Json<UserLoginRequest>,
    db: &State<Arc<Surreal<Any>>>
) -> Result<(Status, Json<String>), (Status, &'static str)> {
    
    let mut response = db
        .query("SELECT * FROM users WHERE email = $email AND password = $password LIMIT 1")
        .bind(("email", account.email.clone()))
        .bind(("password", account.password.clone()))
        .await
        .map_err(|e| {
            eprintln!("Failed to query database: {}", e);
            (Status::InternalServerError, "Failed to query database")
        })?;

    let user: User = match response.take::<Option<User>>(0) {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err((Status::Unauthorized, "Invalid email or password"));
        }
        Err(e) => {
            eprintln!("Failed to parse user: {}", e);
            return Err((Status::InternalServerError, "Database error while fetching user"));
        }
    };

    let claims: Claims = Claims{
        exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
        sub: user.id.id.to_raw()
    };

    let token = encode(&Header::new(Algorithm::default()), &claims, &EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes()));
    match token {
        Ok(token) => {
            Ok((Status::Ok, Json(token)))
        }
        Err(_) => {
            Err((Status::InternalServerError, "Server error"))
        }
    }
}
