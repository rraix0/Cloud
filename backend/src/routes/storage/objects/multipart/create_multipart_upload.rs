use std::env;
use std::sync::{Arc, MutexGuard};
use std::time::SystemTime;
use aws_sdk_s3::{Client};
use aws_sdk_s3::primitives::DateTime;
use aws_sdk_s3::types::CompletedMultipartUpload;
use rocket::{post, FromForm, State, form::Form};
use rocket::http::Status;
use rocket::serde::json::Json;
use surrealdb::{Surreal, engine::any::Any};
use rocket::time::Duration;
use surrealdb::sql::Thing;
use tokio::time::sleep;
use uuid::Uuid;
use crate::routes::storage::objects::multipart::part_info_storage::{PartInfoHashMap, PartInfoStorage, PartInfoStorageData};
use crate::STORAGE;
use crate::surreal::authorization::handle_authorization;
use crate::types::auth_type::Authorization;

#[derive(FromForm)]
pub struct CreateMultipartUpload {
    title: String,
    authorization: Authorization,
    folder: Json<Thing>,
    size: u64,
}

#[post("/storage/create_multipart_upload", data = "<form>")]
pub async fn create_multipart_upload(
    db: &State<Arc<Surreal<Any>>>,
    client: &State<Arc<Client>>,
    form: Form<CreateMultipartUpload>,
) -> Result<(Status, Json<String>), (Status, &'static str)> {

    handle_authorization(db, form.authorization.clone()).await?;

    let file_id = Uuid::now_v7();
    let key = format!("{}/{}", form.authorization.user_id , file_id);

    let resp = client.create_multipart_upload()
        .bucket(env::var("AWS_BUCKET").unwrap())
        .key(key.clone())
        .expires(DateTime::from(SystemTime::now() + Duration::new(300, 0)))
        .send()
        .await.map_err(|err| {println!("{:#?}", err); (Status::BadRequest, "Failed to create upload object")})?;

    println!("STARTED UPLOAD: {:?}", resp);

    let upload_id_clone = resp.clone().upload_id.unwrap().to_string();

    let mut storage: MutexGuard<PartInfoHashMap> = STORAGE.lock().map_err(|_| (Status::InternalServerError, "Error while collecting file parts"))?;
    storage.remove(&(form.authorization.user_id.clone(), resp.clone().upload_id.unwrap().to_string()));
    storage.insert((form.authorization.user_id.clone(), resp.clone().upload_id.unwrap().to_string()),
       PartInfoStorage {
           data: PartInfoStorageData {
               completed_multipart_upload: CompletedMultipartUpload::builder().build(),
               object_id: file_id.to_string(),
               key: key,
               title: form.title.clone(),
               folder: form.folder.clone().into_inner(),
               size: form.size,
           },
           auto_destruction: tokio::spawn(async move {
               sleep(tokio::time::Duration::from_secs(300)).await;
               println!("USUWANIE A");
               let mut storage: MutexGuard<PartInfoHashMap> = STORAGE.lock().unwrap();
               storage.remove(&(form.authorization.user_id.clone(), upload_id_clone.clone()));
           }),
       });


    Ok((Status::Ok, Json(format!("{}", resp.upload_id().unwrap().to_string()))))
}