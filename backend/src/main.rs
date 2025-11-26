use std::sync::Arc;
use aws_sdk_s3::Client;
use rocket::{launch, routes};
use dotenvy::dotenv;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
mod routes;
mod types;
mod surreal;
mod minio;
mod guards;

use crate::{
    minio::connection_minio::get_connection_minio
    ,
    routes::{
        storage::{
            folders::{
                create_folder::create_folder,
                delete_folders::delete_folders,
                get_folders::get_folders,
            },
            objects::{
                get_objects::get_objects,
                multipart::{
                    complete_multipart_upload::complete_multipart_upload,
                    create_multipart_upload::create_multipart_upload,
                    multipart_upload::multipart_upload,
                    part_info_storage::STORAGE,
                },
                upload_object::upload_object,
            }
        },
        users::{
            login::login,
            register::register,
        }
    },
    surreal::connection::get_connection
};
use crate::routes::storage::folders::move_folders::move_folders;
use crate::routes::storage::folders::rename_folder::rename_folder;
use crate::routes::storage::objects::delete_objects::delete_objects;
use crate::routes::storage::objects::get_download_uri::get_download_uri;
use crate::routes::storage::objects::move_objects::move_objects;
use crate::routes::storage::objects::multipart::abort_multipart_upload::abort_multipart_upload;
use crate::routes::storage::objects::rename_object::rename_object;
use crate::routes::storage::share::create_share::create_share;
use crate::routes::users::change_password::change_password;
use crate::routes::users::edit_user::edit_user;
use crate::routes::users::get_user_info::get_user_info;
use crate::routes::users::reset_token::reset_token;

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
            edit_user,
            register,
            get_user_info,
            create_folder,
            get_folders,
            delete_folders,
            get_objects,
            upload_object,
            create_multipart_upload,
            multipart_upload,
            complete_multipart_upload,
            abort_multipart_upload,
            get_download_uri,
            delete_objects,
            rename_folder,
            rename_object,
            move_objects,
            move_folders,
            reset_token,
            change_password,
            create_share,
        ])
}