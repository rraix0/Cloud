use std::env;
use std::sync::Arc;
use std::time::Duration;
use aws_sdk_s3::Client;
use aws_sdk_s3::presigning::PresigningConfig;
use rocket::{post, State};
use rocket::http::Status;
use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use surrealdb::engine::any::Any;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use crate::types::auth_type::{AuthToken};
use crate::types::storage::object::ObjectType;

#[derive(Deserialize, Debug)]
pub struct GetDownloadUri {
    object_id: Thing,
}


#[post("/storage/get_download_uri", data = "<data>")]
pub async fn get_download_uri (
    db: &State<Arc<Surreal<Any>>>,
    client: &State<Arc<Client>>,
    data: Json<GetDownloadUri>,
    auth_token: AuthToken,
) -> Result<(Status, String), (Status, &'static str)> {

    let mut resp = db
        .query("SELECT * FROM files where user_id = $user_id AND id = $id")
        .bind(("user_id", Thing::from(("users", auth_token.0.sub.clone().as_str()))))
        .bind(("id", data.object_id.clone()))
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

    let file_name = objects[0].name.clone();

    let resp = client
        .get_object()
        .bucket(env::var("AWS_BUCKET").unwrap())
        .key(format!("{}/{}", auth_token.0.sub.clone(), data.object_id.id.to_raw().clone()))
        .response_content_disposition(&format!("attachment; filename={:#?}", file_name))
        .presigned(
            PresigningConfig::builder()
                .expires_in(Duration::from_secs(5 * 60))
                .build().unwrap()
        )
        .await.unwrap();

    println!("{:?}", resp);

    Ok((Status::Ok, format!("{:?}", resp.uri())))
}