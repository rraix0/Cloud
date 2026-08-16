use std::sync::Arc;
use aws_sdk_s3::Client;
use rocket::{post, State};
use rocket::http::Status;
use rocket::request::Outcome;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use surrealdb::engine::any::Any;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use crate::guards::auth_type::{AuthToken};
use crate::routes::storage::share::functions::check_share::check_if_shared;
use crate::types::share::shared::CheckObjectShare;
use crate::types::storage::object::ObjectType;

#[derive(Deserialize, Serialize, Debug)]
pub struct GetObjects {
    pub parent_id: Option<Thing>,
    pub share_token: Option<String>,
}

#[post("/storage/get_objects", data = "<data>")]
pub async fn get_objects (
    db: &State<Arc<Surreal<Any>>>,
    _client: &State<Arc<Client>>,
    data: Json<GetObjects>,
    auth_token: Option<AuthToken>,
) -> Result<(Status, Json<Vec<ObjectType>>), (Status, &'static str)> {
    /*
let x = check_if_shared(db, CheckObjectShare {
    link_token: data.share_token.clone(),
    share_user_id: None,
    share_id: None,
    object_id: None,
    folder_id: data.parent_id.clone(),
}).await;

    println!("{:?}", x);

     */


    let query = match data.share_token.clone() {
        Some(share_token) => {
            match data.parent_id {
                Some(_) => {
                    let x = check_if_shared(
                        &db,
                        CheckObjectShare {
                            link_token: Some(share_token),
                            share_user_id: None,
                            share_id: None,
                            object_id: None,
                            folder_id: data.parent_id.clone()
                        }
                    ).await;
                    println!("SHARE CHECK: {:?}", x);
                    if let Err(err) = x {
                        return Err((Status::BadRequest, err));
                    }
                    "SELECT * omit share_id FROM files where parent_id = $parent_id;"
                }
                None => {
                    "{
                        LET $file_id = ((select VALUE file_id from shared where link_token = $share_token).first());

                        if $file_id != NONE {
                            LET $one_file = select VALUE file_id.* OMIT share_id from shared where link_token = $share_token;
                            return $one_file;
                        } else {
                            LET $folder_id = (select VALUE folder_id from shared where link_token = $share_token).first();
                            LET $files = select * omit share_id from files where parent_id = $folder_id;
                            return $files
                        };

                    }"
                }
            }
        }
        None => {
            if auth_token.is_none() {
                return Err((Status::Unauthorized, "Token Not Found"));
            }
            match data.parent_id {
                Some(_) => {
                    "SELECT *, share_id.*.* as share FROM files where user_id = $user_id AND parent_id = $parent_id;"
                }
                None => {
                    "SELECT *, share_id.*.* as share FROM files where user_id = $user_id AND parent_id.parent_id = none;"
                }
            }
        }
    };

    let user_id: Option<Thing> = match auth_token {
        Some(token) => {
            Some(Thing::from(("users", token.0.sub.as_str())))
        }
        None => {
            None
        }
    };
    
    let mut resp = db
    .query(query)
    .bind(("user_id", user_id))
    .bind(("parent_id", data.parent_id.clone()))
    .bind(("share_token", data.share_token.clone()))
    .await
    .map_err(|e| {
        eprintln!("Failed to delete folder: {}", e);
        (Status::NotFound, "Object not found")
    })?;

    let objects:Vec<ObjectType> = match resp.take::<Vec<ObjectType>>(0) {
        Ok(data) => {
            data
        }
        Err(e) => {
            eprintln!("Failed to parse user: {}", e);
            return Err((Status::InternalServerError, "Database error while fetching user"));
        }
    };
    Ok((Status::Ok, Json(objects)))

}