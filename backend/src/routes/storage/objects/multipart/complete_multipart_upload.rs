use std::env;
use std::sync::{Arc, MutexGuard};
use aws_sdk_s3::{Client};
use rocket::{post, FromForm, State, form::Form};
use rocket::http::Status;
use rocket::serde::json::Json;
use surrealdb::{Surreal, engine::any::Any};
use surrealdb::sql::Thing;
use crate::routes::storage::objects::multipart::part_info_storage::{PartInfoHashMap, PartInfoStorageData};
use crate::STORAGE;
use crate::types::auth_type::{AuthToken};

#[derive(FromForm)]
pub struct CompleteUpload {
    upload_id: String,
}

#[post("/storage/complete_multipart_upload", data = "<form>")]
pub async fn complete_multipart_upload(
    db: &State<Arc<Surreal<Any>>>,
    client: &State<Arc<Client>>,
    form: Form<CompleteUpload>,
    auth_token: AuthToken,
) -> Result<(Status, Json<String>), (Status, &'static str)> {

    let part: PartInfoStorageData = {
        let storage: MutexGuard<PartInfoHashMap> = STORAGE.lock().map_err(|_| (Status::InternalServerError, "Error while collecting file parts"))?;
        let x = match storage.get(&(auth_token.0.sub.clone(), form.upload_id.clone())) {
            Some(part) => {
                part.data.clone()
            }
            None => {
                return Err((Status::InternalServerError, "Missing part"));
            }
        };
        x.clone()
    };

    {
        let mut storage: MutexGuard<PartInfoHashMap> = STORAGE.lock().map_err(|_| (Status::InternalServerError, "Error while collecting file parts"))?;
        storage.remove(&(auth_token.0.sub.clone(), form.upload_id.clone()));
    }

    let sum_parts_size = part.parts_size.iter().map(|(_, v)| *v as u64).sum::<u64>();
    println!("{}, {},  {:?}", part.size, sum_parts_size, part.parts_size);
    if part.size != sum_parts_size {
        let _ = client.abort_multipart_upload()
            .bucket(env::var("AWS_BUCKET").unwrap())
            .key(part.key.clone())
            .upload_id(form.upload_id.clone())
            .send()
            .await
            .map_err(|err| {
                eprintln!("Error aborting multipart upload: {:?}", err);
            });
        return Err((Status::BadRequest, "Parts size mismatch"));
    }

    let _resp = client.complete_multipart_upload()
        .bucket(env::var("AWS_BUCKET").unwrap())
        .key(part.key.clone())
        .upload_id(form.upload_id.clone())
        .multipart_upload(part.completed_multipart_upload.clone())
        .send()
        .await.map_err(|err| {println!("{:#?}", err); (Status::BadRequest, "Failed to create upload object")})?;


    let resp = db
        .query("insert into files
            (id, user_id, object_id, name, size, created_at, parent_id)
            values ($id, $user_id, $object_id, $name, $size, time::now(), $parent_id);")
        .bind(("id", part.object_id.clone()))
        .bind(("user_id", Thing::from(("users", auth_token.0.sub.as_str()))))
        .bind(("object_id", part.object_id.clone()))
        .bind(("name", part.title.clone()))
        .bind(("size", part.size.clone()))
        .bind(("parent_id", part.folder.clone()))
        .await;

    if let Err(_) = resp {
        let _ = client.delete_object()
            .bucket(env::var("AWS_BUCKET").unwrap())
            .key(part.key)
            .send()
            .await
            .map_err(|err| {
                eprintln!("Occurred an error while trying to remove file after db error: {:?}", err);
            });
        return Err((Status::InternalServerError, "Occurred server error while trying to upload file"));
    }

    return Ok((Status::Ok, Json(format!("{:#?}", resp))));
}