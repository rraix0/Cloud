use std::env;
use std::sync::Arc;
use std::time::Duration;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::Client;
use rocket::{post, State};
use rocket::http::Status;
use rocket::serde::json::Json;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use crate::guards::auth_type::AuthToken;
use crate::types::storage::object::UploadObject;

#[post("/storage/upload_objects_uri", data = "<data>")]
pub async fn upload_objects_uri(
    _db: &State<Arc<Surreal<Any>>>,
    client: &State<Arc<Client>>,
    data: Json<UploadObject>,
    auth_token: AuthToken,
) -> Result<(Status, Json<String>), (Status, &'static str)> {


    let resp = client
        .put_object()
        .bucket(env::var("AWS_BUCKET").unwrap())
        .key(format!("{}/{}", auth_token.0.sub.clone(), data.name.clone()))
        .content_length(data.length.clone()) // set size fx. 50833
        .presigned(
            PresigningConfig::builder()
            .expires_in(Duration::from_secs(5 * 60))
            .build().unwrap()
        )
        .await.unwrap();
    println!("{:?}", resp);

    Ok((Status::Ok, Json(format!("{:?}", resp.uri()))))
}

