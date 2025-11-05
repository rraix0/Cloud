use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use aws_sdk_s3::{Client, primitives::ByteStream};
use image::ImageFormat;
use image::imageops::FilterType;
use rocket::{post, FromForm, State, form::Form};
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::serde::json::Json;
use surrealdb::{Surreal, engine::any::Any};
use surrealdb::sql::Bytes;
use surrealdb::sql::Thing;
use uuid::Uuid;
use webp::{Encoder, WebPMemory};
use crate::routes::storage::objects::functions::get_thumbnail::get_thumbnail;
use crate::types::auth_type::{AuthToken};

#[derive(FromForm, Debug)]
pub struct UploadObject<'a> {
    file: TempFile<'a>,
    title: String,
    folder: Json<Thing>,
}

#[post("/storage/upload_object", data = "<form>")]
pub async fn upload_object<'a>(
    db: &State<Arc<Surreal<Any>>>,
    client: &State<Arc<Client>>,
    form: Form<UploadObject<'a>>,
    auth_token: AuthToken,
) -> Result<(Status, Json<String>), (Status, &'static str)> {
    // Check storage capacity
    let mut actual_files_size_response = db
        .query("RETURN [math::sum((select size from files where $user_id).size), (select storage_capacity from $user_id).storage_capacity[0]];")
        .bind(("user_id", Thing::from(("users", auth_token.0.sub.as_str()))))
        .await
        .map_err(|err| {
            eprintln!("Occurred an error while trying to remove file after db error: {:?}", err);
            (Status::InternalServerError, "Couldn't check storage capacity!")
        })?;

    let actual_files_size: [u64; 2] = match actual_files_size_response.take::<Vec<u64>>(0) {
        Ok(size) => {
            size.try_into().unwrap()
        },
        Err(_) => {
            return Err((Status::InternalServerError, "Couldn't parse storage capacity!"));
        }
    };
    if actual_files_size[0] + form.file.len() > actual_files_size[1] {
        return Err((Status::BadRequest, "You don't have enough space!"));
    } // END

    println!("{:?}, {:?}", form.file.name(), form.file.content_type());
    println!("{:?}", form.file.path().clone());

        // THUMBNAIL

    //Format
    let img_bytes:Option<Bytes> =  match get_thumbnail(form.title.clone(), form.file.path()) {
        Ok(img_vec) => {
            println!("{:?}", img_vec);
            let img_bytes: Bytes = Bytes::from(img_vec);
            Some(img_bytes)
        }
        Err(_) => {
            None
        }
    };

    let path = form.file.path();

    let stream_bytes = ByteStream::from_path(path.unwrap()).await.unwrap();

    let object_id = Uuid::now_v7();
    let size = form.file.len();
    let key = format!("{}/{}", auth_token.0.sub, object_id);

    client.put_object()
        .bucket(env::var("AWS_BUCKET").unwrap())
        .key(key.clone())
        .body(stream_bytes)
        .send()
        .await
        .map_err(|_| (Status::BadRequest, "no upload object"))?;


    let resp = db
        .query("insert into files
                (id, user_id, object_id, name, size, created_at, parent_id, thumbnail)
                values ($id, $user_id, $object_id, $name, $size, time::now(), $parent_id, $thumbnail);")
        .bind(("id", object_id.to_string().clone()))
        .bind(("user_id", Thing::from(("users", auth_token.0.sub.as_str()))))
        .bind(("object_id", object_id.to_string().clone()))
        .bind(("name", form.title.clone()))
        .bind(("size", size.clone()))
        .bind(("parent_id", form.folder.clone().into_inner() ))
        .bind(("thumbnail", img_bytes))
        .await;


    if let Err(_) = resp {
        let _ = client.delete_object()
            .bucket(env::var("AWS_BUCKET").unwrap())
            .key(key)
            .send()
            .await
            .map_err(|err| {
                eprintln!("Occurred an error while trying to remove file after db error: {:?}", err);
            });
        return Err((Status::InternalServerError, "Occurred server error while trying to upload file"));
    }

    Ok((Status::Ok, Json(format!("{:#?}", resp))))
}