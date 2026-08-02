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
use surrealdb::{Response, Surreal};
use uuid::Uuid;
use surrealdb::sql::Datetime;
use crate::types::share::shared::{Shared, PERMISSIONS};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeleteShare {
    pub id: Thing,
}
#[post("/share/delete_share", data = "<data>")]
pub async fn delete_share(
    data: Json<DeleteShare>,
    db: &State<Arc<Surreal<Any>>>,
    auth_token: AuthToken,
) -> Result<(Status, ()), (Status, &'static str)> {
    let query = "
        let $file_id = (select VALUE file_id from $shared_id).first();
        let $folder_id = (select VALUE folder_id from $shared_id).first();

        if ($file_id) {
            let $index = (select VALUE share_id from $file_id).first().find_index($shared_id);
            update $file_id set share_id = array::remove(share_id, $index) where user_id = $user_id;
        } else {
            let $index = (select VALUE share_id from $folder_id).first().find_index($shared_id);
            update $folder_id set share_id = array::remove(share_id, $index) where user_id = $user_id;
        };

        delete $shared_id where user_id = $user_id;
    ".to_string();

    db
        .query(query)
        .bind(("shared_id", data.id.clone()))
        .bind(("user_id", Thing::from(("users", auth_token.0.sub.as_str()))))
        .await
        .map_err(|err| {
            eprintln!("{}", err);
            (Status::InternalServerError, "Internal server error")
        })?;

    Ok((Status::Ok, ()))
}

