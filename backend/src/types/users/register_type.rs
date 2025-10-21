use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserRegisterRequest {
    pub ticket: String,
    pub name: String,
    pub email: String,
    pub password: String,
}