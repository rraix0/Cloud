use std::env;
use std::sync::{Arc, MutexGuard};
use aws_sdk_s3::{Client};
use rocket::{post, FromForm, State, form::Form};
use rocket::http::Status;
use rocket::serde::json::Json;
use surrealdb::{Surreal, engine::any::Any};
use crate::routes::storage::objects::multipart::part_info_storage::{PartInfoHashMap, STORAGE};
use crate::guards::auth_type::{AuthToken};

#[derive(FromForm)]
pub struct AbortUpload {
    upload_id: String,
}

#[post("/storage/abort_multipart_upload", data = "<form>")]
pub async fn abort_multipart_upload(
    _db: &State<Arc<Surreal<Any>>>,
    client: &State<Arc<Client>>,
    form: Form<AbortUpload>,
    auth_token: AuthToken,
) -> Result<(Status, Json<String>), (Status, &'static str)> {

    let key:String = {
        let mut storage: MutexGuard<PartInfoHashMap> = STORAGE.lock().map_err(|_| (Status::InternalServerError, "Error while collecting file parts"))?;
        let value = match storage.get(&(auth_token.0.sub.clone(), form.upload_id.clone())) {
            Some(storage) => storage.data.key.clone(),
            None => {
                return Err((Status::InternalServerError, "Error while collecting file part"));
            },
        };
        storage.remove(&(auth_token.0.sub.clone(), form.upload_id.clone()));
        value
    };

    let resp = client.abort_multipart_upload()
        .bucket(env::var("AWS_BUCKET").unwrap())
        .key(key)
        .upload_id(form.upload_id.clone())
        .send()
        .await.map_err(|err| {println!("{:#?}", err); (Status::BadRequest, "Failed to create upload object")})?;

    println!("COMPLETE UPLOAD: {:?}", resp);


    Ok((Status::Ok, Json(format!("Uploaded {}", "sdfklj".to_string()))))
}