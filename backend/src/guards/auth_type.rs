use std::env;
use std::sync::Arc;
use jsonwebtoken::{DecodingKey, Validation};
use rocket::serde::{Deserialize, Serialize};
use rocket::http::Status;
use rocket::request::{Request, FromRequest, Outcome};
use surrealdb::engine::any::Any;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use crate::types::users::user::User;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub exp: usize,
    pub sub: String,
    pub sub_jwt: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthToken(pub Claims);
#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthToken {
    type Error = String;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let db = req.rocket().state::<Arc<Surreal<Any>>>();
        let db = match db {
            Some(db) => db,
            None => return Outcome::Error((Status::InternalServerError, ("Internal server error".to_string()))),
        };


        match req.headers().get_one("Authorization") {
            Some(token) => {
                let decoted_token = jsonwebtoken::decode::<Claims>(token, &DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes()), &Validation::default());
                match decoted_token {
                    Ok(token) => {
                        let mut response = db
                          .query("SELECT * FROM users WHERE id = $id and jwt = $jwt;")
                          .bind(("id", Thing::from(("users", token.claims.sub.as_str()))))
                          .bind(("jwt", token.claims.sub_jwt.clone()))
                        .await
                          .map_err(|e| -> Outcome<Status, String> {
                              eprintln!("Failed to query database: {}", e);
                              Outcome::Error((Status::InternalServerError, ("Internal server error".to_string())))
                          }).unwrap();

                        match response.take::<Option<User>>(0) {
                            Ok(Some(_)) => {  },
                            Ok(None) => {
                                return Outcome::Error((Status::Unauthorized, ("Wrong token".to_string())));
                            }
                            Err(err) => {
                                eprintln!("Failed to query database: {}", err);
                                return Outcome::Error((Status::InternalServerError, ("Internal server error".to_string())));
                            }
                        };

                        Outcome::Success(AuthToken(token.claims))

                    }
                    Err(err) => {
                        Outcome::Error((Status::Unauthorized, err.to_string()))
                    }
                }
            },
            None => Outcome::Error((Status::Unauthorized, "Token Not Found".to_string())),
        }
    }
}