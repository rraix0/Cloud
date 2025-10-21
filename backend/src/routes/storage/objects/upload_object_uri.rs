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
use crate::surreal::authorization::handle_authorization;
use crate::types::storage::object::UploadObject;

#[post("/storage/upload_objects_uri", data = "<data>")]
pub async fn upload_objects_uri(
    db: &State<Arc<Surreal<Any>>>,
    client: &State<Arc<Client>>,
    data: Json<UploadObject>
) -> Result<(Status, Json<String>), (Status, &'static str)> {

    handle_authorization(db, data.authorization.clone()).await?;


    let resp = client
        .put_object()
        .bucket(env::var("AWS_BUCKET").unwrap())
        .key(format!("{}/{}", data.authorization.user_id.clone(), data.name.clone()))
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

