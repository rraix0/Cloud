use std::env;
use std::sync::Arc;
use aws_sdk_s3::Client;
use aws_sdk_s3::operation::delete_objects::DeleteObjectsOutput;
use aws_sdk_s3::types::{Delete, ObjectIdentifier};
use rocket::{post, State};
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use surrealdb::engine::any::Any;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use crate::guards::auth_type::{AuthToken};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeleteObjects {
    pub object_ids: Vec<Thing>,
}

#[post("/storage/delete_objects", data = "<data>")]
pub async fn delete_objects (
    db: &State<Arc<Surreal<Any>>>,
    client: &State<Arc<Client>>,
    data: Json<DeleteObjects>,
    auth_token: AuthToken,
) -> Result<(Status, ()), (Status, &'static str)> {

    let file_ids:Vec<ObjectIdentifier> = data.clone().object_ids.iter()
    .map(|id|
    ObjectIdentifier::builder()
        .key(format!("{}/{}", auth_token.0.sub, id.id.to_raw()))
        .build())
        .collect::<Result<Vec<ObjectIdentifier>, _>>()
        .map_err(|e| {
        println!("{}", e);
        (Status::InternalServerError, "could not retrieve folder")
    })?;

    let _resp:DeleteObjectsOutput = client
        .delete_objects()
        .bucket(env::var("AWS_BUCKET").unwrap())
        .delete( Delete::builder()
            .set_objects(Some(file_ids))
            .build()
            .map_err(|e| {
                println!("{}", e);
                (Status::InternalServerError, "could not retrieve folder")
            })?
        )
        .send()
        .await
        .map_err(|e| {
            println!("{}", e);
            (Status::InternalServerError, "could not retrieve folder")
        })?;

    let _resp = db
        .query("delete from files where id in $file_ids and user_id = $user_id")
        .bind(("file_ids",  data.object_ids.clone()))
        .bind(("user_id", Thing::from(("users", auth_token.0.sub.as_str()))))
        .await
        .map_err(|e| {
            eprintln!("Failed to delete folder: {}", e);
            (Status::NotFound, "Object not found")
        })?;

    println!("{:?}", data.object_ids.clone());

    return Ok((Status::Ok, ()))
}