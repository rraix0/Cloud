use std::env;
use std::sync::Arc;
use aws_sdk_s3::Client;
use aws_sdk_s3::types::{Delete, ObjectIdentifier};
use rocket::{post, State};
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use surrealdb::engine::any::Any;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use crate::types::auth_type::{AuthToken};
use crate::types::storage::object::ObjectType;

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteObjects {
    pub ids: Vec<Thing>,
}

#[post("/storage/delete_objects", data = "<data>")]
pub async fn delete_objects (
    db: &State<Arc<Surreal<Any>>>,
    client: &State<Arc<Client>>,
    data: Json<DeleteObjects>,
    auth_token: AuthToken,
) -> Result<(Status, ()), (Status, &'static str)> {

    let objects = data.ids.iter()
        .map(|x|  {
            ObjectIdentifier::builder()
                .key(x.id.to_raw())
                .build()
                .map_err(|err| {
                    println!("{:#?}", err);
                    (Status::InternalServerError, "Couldn't parse objects.")
                })
        })
        .collect::<Result<Vec<ObjectIdentifier>, _>>()?;



    /*
    let resp = client
        .delete_objects()
        .bucket(env::var("AWS_BUCKET").unwrap().as_ref())
        .delete(Delete {
            objects: vec![],
            quiet: None
        })



    let resp = db
        .query("delete from files where id in [$file_ids] and user_id = $user_id")
        .bind(("file_ids",  data.ids.clone()))
        .bind(("user_id", Thing::from(("users", auth_token.0.sub.as_str()))))
        .await
        .map_err(|e| {
            eprintln!("Failed to delete folder: {}", e);
            (Status::NotFound, "Object not found")
        })?;



    println!("{:#?}", resp);
     */

    Ok((Status::Ok, ()))
}