use crate::guards::auth_type::AuthToken;
use crate::types::storage::folder::Folder;
use crate::types::storage::object::ObjectType;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{post, State};
use std::sync::Arc;
use surrealdb::engine::any::Any;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use uuid::Uuid;
use surrealdb::sql::Datetime;
use crate::types::share::shared::{Shared, PERMISSIONS};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateShare {
    pub description: Option<String>,
    pub file_id: Option<Thing>,
    pub folder_id: Option<Thing>,
    pub permissions: Vec<String>,
    pub share_user_id: Option<Thing>,
    pub expiration_time: Option<chrono::DateTime<chrono::Utc>>
}

pub enum ShareReturn {
    ObjectType(ObjectType),
    Folder(Folder),
}
#[post("/share/create_share", data = "<data>")]
pub async fn create_share(
    data: Json<CreateShare>,
    db: &State<Arc<Surreal<Any>>>,
    auth_token: AuthToken,
) -> Result<(Status, Json<Shared>), (Status, &'static str)> {
    if !((data.file_id.is_some() && data.folder_id.is_none())
        || (data.file_id.is_none() && data.folder_id.is_some())) {
        return Err((Status::BadRequest, "No object has been specified to be shared."));
    }

    let permissions = PERMISSIONS.read().unwrap().clone();
    if !data.permissions.iter().all(|x| permissions.contains(x)) {
        return Err((Status::BadRequest, "Permissions are incorrect."));
    }

        let link_token:String = Uuid::new_v4().to_string();
        let expiration_time:Option<Datetime> = match data.expiration_time {
            Some(date) => {
                Some(Datetime::from(date))
            }
            None => {
                None
            }
        };

        let mut query = "".to_string();
        query += "{";

        if data.file_id.is_some() {
            query.push_str("LET $if_exist = select id from $file_id;");
        } else {
            query.push_str("LET $if_exist = select id from $folder_id;");
        }
        query.push_str("
        if $if_exist {
        let $response = insert into shared (id, description, expiration_date, file_id, folder_id, link_token, permissions, share_user_id, user_id)
            values ($id, $description, $expiration_date, $file_id, $folder_id, $link_token, $permissions, $share_user_id, $user_id); ");

        if data.file_id.is_some() {
            query.push_str("update files set share_id += $share_id_thing
            where id = $file_id
            and user_id = $user_id;");
        } else {
            query.push_str("update files set share_id += $share_id_thing
            where id = $folder_id
            and user_id = $user_id;");
        }

        query.push_str("return $response; }}");

        println!("{}", query);

        let share_id = Uuid::now_v7().to_string();

        let mut result = db
            .query(query)
            .bind(("id", share_id.clone()))
            .bind(("share_id_thing", Thing::from(("shared", share_id.to_string().as_ref()))))
            .bind(("description", data.description.clone()))
            .bind(("expiration_date", expiration_time))
            .bind(("file_id", data.file_id.clone()))
            .bind(("folder_id", data.folder_id.clone()))
            .bind(("link_token", link_token))
            .bind(("permissions", data.permissions.clone()))
            .bind(("share_user_id", data.share_user_id.clone()))
            .bind(("user_id", Thing::from(("users", auth_token.0.sub.as_str()))))
            .await
            .map_err(|err| {
                eprintln!("{}", err);
                (Status::InternalServerError, "Internal server error")
            })?;

        let shared:Shared = match result.take::<Vec<Shared>>(0) {
            Ok(object) => {
                let obj: Shared = match object.get(0) {
                    None => {
                        return Err((Status::InternalServerError, "Occurred an error while trying to share data."));
                    }
                    Some(obj) => {
                        obj.to_owned()
                    }
                };
                obj
            }
            Err(err) => {
                eprintln!("{}", err);
                return Err((Status::InternalServerError, "Occurred an error while trying to share data."));
            }
        };

    Ok((Status::Ok, Json(shared)))
}

