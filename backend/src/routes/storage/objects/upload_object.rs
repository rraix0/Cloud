use std::env;
use std::sync::Arc;
use aws_sdk_s3::{Client, primitives::ByteStream};
use rocket::{post, FromForm, State, form::Form};
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::serde::json::Json;
use surrealdb::{Surreal, engine::any::Any};
use crate::surreal::authorization::handle_authorization;
use crate::types::auth_type::Authorization;

#[derive(FromForm)]
pub struct UploadObject<'a> {
    file: TempFile<'a>,
    title: String,
    authorization: Authorization,
}

#[post("/storage/upload_object", data = "<form>")]
pub async fn upload_object<'a>(
    db: &State<Arc<Surreal<Any>>>,
    client: &State<Arc<Client>>,
    form: Form<UploadObject<'a>>,
) -> Result<(Status, Json<String>), (Status, &'static str)> {
    handle_authorization(db, form.authorization.clone()).await?;

    println!("{:?}, {:?}", form.file.name(), form.file.content_type());
    println!("{:?}", form.file.path().clone());

    let path = form.file.path();

    let stream_bytes = ByteStream::from_path(path.unwrap()).await.unwrap();

    client.put_object()
        .bucket(env::var("AWS_BUCKET").unwrap())
        .key(format!("{}", form.title.clone()))
        .body(stream_bytes)
        .send()
        .await
        .map_err(|_| (Status::BadRequest, "no upload object"))?;

    Ok((Status::Ok, Json(format!("Uploaded {}", "sdfklj".to_string()))))
}