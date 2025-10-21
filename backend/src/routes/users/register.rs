use std::sync::Arc;
use rocket::http::Status;
use rocket::{post, State};
use rocket::serde::json::Json;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use uuid::{Uuid};
use crate::types::auth_type::Authorization;
use crate::types::users::{
    register_type::UserRegisterRequest,
    ticket::Ticket,
    user::User,
};

#[post("/users/register", data = "<account>")]
pub async fn register(
    account: Json<UserRegisterRequest>,
    db: &State<Arc<Surreal<Any>>>
) -> Result<(Status, Json<Authorization>), (Status, &'static str)> {


    let mut ticket_response = db
        .query("SELECT * FROM tickets WHERE ticket = $ticket LIMIT 1")
        .bind(("ticket", account.ticket.clone()))
        .await
        .map_err(|e| {
            eprintln!("Failed to query database: {}", e);
            (Status::InternalServerError, "Failed to query database")
        })?;

    let ticket: Ticket = match ticket_response.take::<Option<Ticket>>(0) {
        Ok(Some(ticket)) => ticket,
        Ok(None) => {
            return Err((Status::NotFound, "Ticket not found"));
        }
        Err(e) => {
            eprintln!("Failed to fetch ticket: {}", e);
            return Err((Status::InternalServerError, "Database error while fetching ticket"));
        }
    };

    let mut email_response = db
        .query("SELECT * FROM users WHERE email = $email LIMIT 1")
        .bind(("email", account.email.clone()))
        .await
        .map_err(|e| {
            eprintln!("Failed to query database: {}", e);
            (Status::InternalServerError, "Failed to query database")
        })?;

    println!("{:?}", email_response);
    match email_response.take::<Option<User>>(0) {
        Ok(Some(_)) => {
            return Err((Status::Conflict, "Email already in use"));
        }
        Ok(None) => {} // No user, you can create one
        Err(e) => {
            eprintln!("Failed to fetch user: {}", e);
            return Err((Status::InternalServerError, "Database error while finding user"));
        }
    };

    let create_account = db
        .query("insert into users (id, created_at, email, name, password, storage_capacity, jwt) values ($id, time::now(), $email, $name, $password, $capacity, $jwt)")
        .bind(("id", Uuid::now_v7().to_string()))
        .bind(("email", account.email.clone()))
        .bind(("name", account.name.clone()))
        .bind(("password", account.password.clone()))
        .bind(("capacity", ticket.storage_capacity.clone()))
        .bind(("jwt", Uuid::now_v7().to_string()))
        .await
        .map_err(|e| {
            eprintln!("Failed to query database: {}", e);
        });
    let mut created_user: Option<User> = match create_account {
        Ok(mut result) => {
            println!("Created new user {:?}", result);

            match db
                .query(format!("delete from tickets where id = {};", ticket.id.clone()))
                .await {
                Ok(i) => {
                    println!("Succesfully deleted ticket {:?}", i);
                },
                Err(e) => {
                    eprintln!("Failed to delete tickets: {}", e);
                }
            }
            result.take::<Option<User>>(0).unwrap()
        }
        Err(e) => {
            eprintln!("Failed to create an account: {:?}", e);
            return Err((Status::InternalServerError, "Failed to create an account"));
        }
    };

    let _ = db
    .query("insert into folders (id, created_at, parent_id, name, user_id, share_id) values ($id, time::now(), NONE, 'root', $user_id, []);")
    .bind(("id", Uuid::now_v7().to_string()))
    .bind(("user_id", created_user.clone().unwrap().id))
    .await
    .map_err(|e| {
        eprintln!("Failed to query database: {}", e);
    });


    match created_user.take() {
        Some(result) => {
            Ok((Status::Ok, Json(Authorization {
                user_id: result.get_id(),
                jwt: result.jwt.clone(),
            })))
        }
        None => {
            Err((Status::InternalServerError, "Couldn't parse user"))
        }
    }

}