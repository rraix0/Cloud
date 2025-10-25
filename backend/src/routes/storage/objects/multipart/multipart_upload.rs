use std::env;
use std::sync::{Arc, MutexGuard};
use aws_sdk_s3::{Client, primitives::ByteStream};
use aws_sdk_s3::types::{CompletedMultipartUpload, CompletedPart};
use rocket::{post, FromForm, State, form::Form};
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::serde::json::Json;
use surrealdb::{Surreal, engine::any::Any};
use tokio::time::sleep;
use crate::routes::storage::objects::multipart::part_info_storage::{PartInfoHashMap, PartInfoStorage, PartInfoStorageData};
use crate::STORAGE;
use crate::types::auth_type::{AuthToken};

#[derive(FromForm)]
pub struct UploadPart<'a> {
    file: TempFile<'a>,
    part_number: i32,
    upload_id: String,
    size: i64,
}

#[post("/storage/multipart_upload", data = "<form>")]
pub async fn multipart_upload<'a>(
    _db: &State<Arc<Surreal<Any>>>,
    client: &State<Arc<Client>>,
    form: Form<UploadPart<'a>>,
    auth_token: AuthToken,
) -> Result<(Status, Json<String>), (Status, &'static str)> {
    // File byte stream to upload
    let byte_stream = ByteStream::from_path(form.file.path().unwrap()).await.map_err(|err| {
        println!("{:?}, {:?}", form.file.path(), err);
        (Status::BadRequest, "{\"error\": \"invalid file\"}")
    })?;
    //Get key to upload from storage
    let storage_data = {
        let storage: MutexGuard<PartInfoHashMap> = STORAGE.lock().map_err(|_| (Status::InternalServerError, "Error while collecting file parts"))?;
        let value = match storage.get(&(auth_token.0.sub.clone(), form.upload_id.clone())) {
            Some(storage) => storage.data.clone(),
            None => {
                return Err((Status::InternalServerError, "Error while collecting file part"));
            },
        };
        value
    };

    // Check if the parts size is not greater than size of whole file
    let sum_parts_size = storage_data.parts_size.iter().map(|(_, v)| *v as u64).sum::<u64>();
    if storage_data.size < sum_parts_size {
        let _ = client.abort_multipart_upload()
            .bucket(env::var("AWS_BUCKET").unwrap())
            .key(storage_data.key.clone())
            .upload_id(form.upload_id.clone())
            .send()
            .await
            .map_err(|err| {
                eprintln!("Error aborting multipart upload: {:?}", err);
            });

        let mut storage: MutexGuard<PartInfoHashMap> = STORAGE.lock().map_err(|_| (Status::InternalServerError, "Error while collecting file parts"))?;
        storage.remove(&(auth_token.0.sub.clone(), form.upload_id.clone()));
        return Err((Status::BadRequest, "Parts size is greater than declared"));
    }


    // Upload file
    let resp = client.upload_part()
        .bucket(env::var("AWS_BUCKET").unwrap())
        .key(storage_data.key.clone())
        .body(byte_stream)
        .content_length(form.size)
        .part_number(form.part_number.clone())
        .upload_id(form.upload_id.clone())
        .send().await.map_err(|err| {
        println!("{:?}, {:?}", form.file.path(), err);
        (Status::BadRequest, "{\"error\": \"invalid file\"}")
    })?;
    // Create actual part info
    let part = CompletedPart::builder().e_tag(resp.e_tag.unwrap()).part_number(form.part_number.clone()).build();

    let mut storage: MutexGuard<PartInfoHashMap> = STORAGE.lock().map_err(|_| (Status::InternalServerError, "Error while collecting file parts"))?;
    // Check parts size | Create PartInfoStorage with new data & recreate auto destruction
    let part_info_storage: PartInfoStorage = match storage.get(&(auth_token.0.sub.clone(), form.upload_id.clone())) {
        None => {
            return Err((Status::InternalServerError, "Couldn't find part parts"));
        }
        Some(item) => {
            let handle= &item.auto_destruction;
            handle.abort();
            let client_clone = Arc::clone(client.inner());
            let key_clone = storage_data.key.clone();
            let upload_id_clone = form.upload_id.clone();
            let user_id_clone = auth_token.0.sub.clone();

            let mut parts_size = item.data.parts_size.clone();
            parts_size.insert(form.part_number.clone(), form.size.clone());

            // Recreate PartInfoStorage with data ab actual updating part
            let infos: PartInfoStorage = match &item.data.completed_multipart_upload.parts {
                None => {
                    let upload_id_clone = form.upload_id.clone();
                    let user_id_clone = auth_token.0.sub.clone();
                    let x:PartInfoStorage = PartInfoStorage {
                        data: PartInfoStorageData {
                            completed_multipart_upload: CompletedMultipartUpload::builder().parts(part).build(),
                            parts_size: parts_size,
                            ..item.data.clone()},
                        auto_destruction: tokio::spawn(async move {
                            sleep(tokio::time::Duration::from_secs(300)).await;
                            println!("USUWANIE B");

                            let _ = client_clone.abort_multipart_upload()
                                .bucket(env::var("AWS_BUCKET").unwrap())
                                .key(key_clone.clone())
                                .upload_id(upload_id_clone.clone())
                                .send()
                                .await
                                .map_err(|err| {
                                    eprintln!("Error aborting multipart upload: {:?}", err);
                                });

                            let mut storage: MutexGuard<PartInfoHashMap> = STORAGE.lock().unwrap();
                            storage.remove(&(user_id_clone.clone(), upload_id_clone.clone()));
                        })};
                    x
                }
                Some(parts_info) => {
                    let mut parts_vec = parts_info.clone();
                    parts_vec.push(part);
                    let x:CompletedMultipartUpload = CompletedMultipartUpload::builder().set_parts(Some(parts_vec)).build();



                    PartInfoStorage {
                        data: PartInfoStorageData {
                            completed_multipart_upload: x,
                            parts_size: parts_size,
                            ..item.data.clone()},
                        auto_destruction: tokio::spawn(async move {
                            sleep(tokio::time::Duration::from_secs(300)).await;
                            println!("USUWANIE B");

                            let _ = client_clone.abort_multipart_upload()
                                .bucket(env::var("AWS_BUCKET").unwrap())
                                .key(key_clone.clone())
                                .upload_id(upload_id_clone.clone())
                                .send()
                                .await
                                .map_err(|err| {
                                    eprintln!("Error aborting multipart upload: {:?}", err);
                                });

                            let mut storage: MutexGuard<PartInfoHashMap> = STORAGE.lock().unwrap();
                            storage.remove(&(user_id_clone.clone(), upload_id_clone.clone()));
                        })
                    }
                },
            };
            storage.remove(&(auth_token.0.sub.clone(), form.upload_id.clone()));
            infos
        }
    };

    storage.insert((auth_token.0.sub.clone(), form.upload_id.clone()), part_info_storage);

    Ok((Status::Ok, Json(format!("Uploaded {}", storage_data.key.to_string()))))
}