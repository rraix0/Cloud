use std::sync::Arc;
use aws_sdk_s3::Client;
use bytes::BufMut;
use rocket::{routes, launch};
use dotenvy::dotenv;
use rocket::data::ToByteUnit;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
use surrealdb::engine::any::Any;
use surrealdb::Surreal;

mod routes;
mod types;
mod surreal;
mod minio;

use crate::{
    routes::{
        users::{
            login::login,
            register::register,
        },
        storage::{
            folders::{
                get_folders::get_folders,
                create_folder::create_folder,
                delete_folders::delete_folders,
            },
            objects::{
                multipart::{
                    create_multipart_upload::create_multipart_upload,
                    multipart_upload::multipart_upload,
                    complete_multipart_upload::complete_multipart_upload,
                    part_info_storage::STORAGE,
                },
                get_objects::get_objects,
                upload_object_uri::upload_objects_uri,
                upload_object::upload_object
            }
        }
    },
    types::{
        users::{

        },
        storage::{

        },
        auth_type
    },
    surreal::{
        connection::get_connection,
    },
    minio::{
        connection_minio::get_connection_minio
    }
};
use crate::routes::storage::objects::get_download_uri::get_download_uri;
use crate::routes::storage::objects::multipart::abort_multipart_upload::abort_multipart_upload;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let db: Surreal<Any> = get_connection().await;
    let db = Arc::new(db);

    let client: Client = get_connection_minio().await;
    let client = Arc::new(client);

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    let cors = cors.to_cors().expect("Failed to create CORS fairing");

    rocket::build()
        .attach(cors)
        .manage(db)
        .manage(client)
        .mount("/api", routes![
            login,
            register,
            create_folder,
            get_folders,
            delete_folders,
            get_objects,
            upload_objects_uri,
            upload_object, //just leave it alone pls
            create_multipart_upload,
            multipart_upload,
            complete_multipart_upload,
            abort_multipart_upload,
            get_download_uri,
        ])
}