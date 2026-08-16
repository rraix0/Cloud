use std::env;
use std::sync::Arc;
use std::time::Duration;
use aws_config::meta::region::RegionProviderChain;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::Client;
use aws_sdk_s3::presigning::PresigningConfig;
use rocket::{post, State};
use rocket::http::Status;
use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use surrealdb::engine::any::Any;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use crate::guards::auth_type::{AuthToken};
use crate::routes::storage::share::functions::check_share::check_if_shared;
use crate::types::share::shared::CheckObjectShare;
use crate::types::storage::object::ObjectType;

#[derive(Deserialize, Debug)]
pub struct GetDownloadUri {
    object_id: Thing,
    share_token: Option<String>,
}


#[post("/storage/get_download_uri", data = "<data>")]
pub async fn get_download_uri (
    db: &State<Arc<Surreal<Any>>>,
    _client: &State<Arc<Client>>,
    data: Json<GetDownloadUri>,
    auth_token: Option<AuthToken>,
) -> Result<(Status, String), (Status, &'static str)> {
        let user_id: Thing = {
        if let Some(auth_token) = auth_token {
            Thing::from(("users", auth_token.0.sub.as_str()))
        } else if let Some(share_token) = data.share_token.clone() {
            let mut resp = db
                .query("SELECT VALUE user_id FROM shared WHERE link_token = $shared_token;")
                .bind(("shared_token", share_token.clone()))
                .await
                .map_err(|e| {
                    eprintln!("Failed to fetch shared data: {}", e);
                    (Status::NotFound, "Object not found")
                })?;

            let objects: Vec<Thing> = resp.take(0).map_err(|e| {
                eprintln!("Failed to parse user: {}", e);
                (Status::InternalServerError, "Database error while fetching user")
            })?;

            let result = match objects.len() {
                1 => objects.into_iter().next().unwrap(),
                0 => return Err((Status::NotFound, "User not found")),
                _ => return Err((Status::InternalServerError, "Multiple users found")),
            };

            let share_result = check_if_shared(
                &db,
                CheckObjectShare {
                    link_token: Some(share_token),
                    share_user_id: None,
                    share_id: None,
                    object_id: Some(data.object_id.clone()),
                    folder_id: None
                }
            ).await;
            println!("SHARE CHECK: {:?}", share_result);
            if let Err(err) = share_result {
                return Err((Status::BadRequest, err));
            }



            result
        } else {
            return Err((Status::BadRequest, "No credentials provided"));
        }
    };







    let mut resp = db
        .query("SELECT * FROM files where user_id = $user_id AND id = $id")
        .bind(("user_id", user_id.clone()))
        .bind(("id", data.object_id.clone()))
        .await
        .map_err(|e| {
            eprintln!("Failed to delete folder: {}", e);
            (Status::NotFound, "Object not found")
        })?;

    let objects:Vec<ObjectType> = match resp.take::<Vec<ObjectType>>(0) {
        Ok(data) => {
            data
        }
        Err(e) => {
            eprintln!("Failed to parse user: {}", e);
            return Err((Status::InternalServerError, "Database error while fetching user"));
        }
    };

    let file_name = objects[0].name.clone();

    // CLIENT TO CREATE PUBLIC LINKS
    let endpoint_url = std::env::var("AWS_ENDPOINT_PUBLIC_URL")
        .expect("AWS_ENDPOINT_PUBLIC_URL must be set");
    let region = std::env::var("AWS_REGION")
        .unwrap_or_else(|_| "eu-central-1".to_string());

    let region_provider = RegionProviderChain::first_try(Region::new(region.clone()));

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    let s3_config = aws_sdk_s3::config::Builder::from(&config)
        .endpoint_url(endpoint_url)
        .force_path_style(true)
        .build();

    let client = Client::from_conf(s3_config);
    // END

    println!("{}/{}", user_id.id.to_raw(), data.object_id.id.to_raw().clone());

    let resp = client
        .get_object()
        .bucket(env::var("AWS_BUCKET").unwrap())
        .key(format!("{}/{}", user_id.id.to_raw(), data.object_id.id.to_raw().clone()))
        .response_content_disposition(&format!("attachment; filename={:#?}", file_name))
        .presigned(
            PresigningConfig::builder()
                .expires_in(Duration::from_secs(5 * 60))
                .build().unwrap()
        )
        .await.unwrap();

    println!("{:?}", resp);
    //let url = resp.uri().replace("http://192.168.0.10:9000", "https://cloudminio.raix.site");


    Ok((Status::Ok, format!("{:?}", resp.uri())))
}