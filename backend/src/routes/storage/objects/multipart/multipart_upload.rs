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
use crate::surreal::authorization::handle_authorization;
use crate::types::auth_type::Authorization;

#[derive(FromForm)]
pub struct UploadPart<'a> {
    file: TempFile<'a>,
    part_number: i32,
    upload_id: String,
    authorization: Authorization,
}

#[post("/storage/multipart_upload", data = "<form>")]
pub async fn multipart_upload<'a>(
    db: &State<Arc<Surreal<Any>>>,
    client: &State<Arc<Client>>,
    form: Form<UploadPart<'a>>,
) -> Result<(Status, Json<String>), (Status, &'static str)> {
    handle_authorization(db, form.authorization.clone()).await?;

    let byte_stream = ByteStream::from_path(form.file.path().unwrap()).await.map_err(|err| {
        println!("{:?}, {:?}", form.file.path(), err);
        (Status::BadRequest, "{\"error\": \"invalid file\"}")
    })?;

    let key:String = {
        let storage: MutexGuard<PartInfoHashMap> = STORAGE.lock().map_err(|_| (Status::InternalServerError, "Error while collecting file parts"))?;
        let value = match storage.get(&(form.authorization.user_id.clone(), form.upload_id.clone())) {
            Some(storage) => storage.data.key.clone(),
            None => {
                return Err((Status::InternalServerError, "Error while collecting file part"));
            },
        };
        value
    };

    let resp = client.upload_part()
        .bucket(env::var("AWS_BUCKET").unwrap())
        .key(key.clone())
        .body(byte_stream)
        .part_number(form.part_number.clone())
        .upload_id(form.upload_id.clone())
        .send().await.map_err(|err| {
        println!("{:?}, {:?}", form.file.path(), err);
        (Status::BadRequest, "{\"error\": \"invalid file\"}")
    })?;

    let part = CompletedPart::builder().e_tag(resp.e_tag.unwrap()).part_number(form.part_number.clone()).build();

    let mut storage: MutexGuard<PartInfoHashMap> = STORAGE.lock().map_err(|_| (Status::InternalServerError, "Error while collecting file parts"))?;

    let part_info_storage: PartInfoStorage = match storage.get(&(form.authorization.user_id.clone(), form.upload_id.clone())) {
        None => {
            return Err((Status::InternalServerError, "Couldn't find part parts"));
        }
        Some(item) => {
            let handle= &item.auto_destruction;
            handle.abort();
            let client_clone = Arc::clone(client.inner());
            let key_clone = key.clone();
            let upload_id_clone = form.upload_id.clone();
            let user_id_clone = form.authorization.user_id.clone();

            let infos: PartInfoStorage = match &item.data.completed_multipart_upload.parts {
                None => {
                    let upload_id_clone = form.upload_id.clone();
                    let user_id_clone = form.authorization.user_id.clone();
                    let x:PartInfoStorage = PartInfoStorage {
                        data: PartInfoStorageData {
                            completed_multipart_upload: CompletedMultipartUpload::builder().parts(part).build(),
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
                            storage.remove(&(user_id_clone.clone(), upload_id_clone.clone()));})};
                    x
                }
                Some(parts_info) => {
                    let mut parts_vec = parts_info.clone();
                    parts_vec.push(part);
                    let x:CompletedMultipartUpload = CompletedMultipartUpload::builder().set_parts(Some(parts_vec)).build();

                    PartInfoStorage {
                        data: PartInfoStorageData {
                            completed_multipart_upload: x,
                            ..item.data.clone()},
                        auto_destruction: tokio::spawn(async move {
                            sleep(tokio::time::Duration::from_secs(10)).await;
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
            storage.remove(&(form.authorization.user_id.clone(), form.upload_id.clone()));
            infos
        }
    };

    storage.insert((form.authorization.user_id.clone(), form.upload_id.clone()), part_info_storage);

    Ok((Status::Ok, Json(format!("Uploaded {}", key.to_string()))))
}