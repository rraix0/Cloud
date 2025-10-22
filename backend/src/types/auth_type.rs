use std::env;
use jsonwebtoken::{DecodingKey, Validation};
use rocket::serde::{Deserialize, Serialize};
use rocket::http::Status;
use rocket::request::{Request, FromRequest, Outcome};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub exp: usize,
    pub sub: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthToken(pub Claims);
#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthToken {
    type Error = String;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("Authorization") {
            Some(token) => {
                let jwt = jsonwebtoken::decode::<Claims>(token, &DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes()), &Validation::default());
                match jwt {
                    Ok(claims) => Outcome::Success(AuthToken(claims.claims)),
                    Err(err) => Outcome::Error((Status::Unauthorized, (err.to_string()))),
                }
            },
            None => Outcome::Error((Status::Unauthorized, ("Token Not Found".to_string()))),
        }
    }
}