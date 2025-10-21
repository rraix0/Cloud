use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserLoginRequest {
    pub email: String,
    pub password: String,
}