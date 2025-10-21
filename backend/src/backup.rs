mod cors;
mod db;
mod guards;
mod routes;
mod services;
mod surreal;
mod types;
mod minio;

use std::sync::LazyLock;

use dotenvy::dotenv;
use rocket::{Build, Rocket, launch, routes};
use surrealdb::{Surreal, engine::remote::ws::Client};
use tracing::info;

use crate::{
    cors::Cors,
    db::init,
    routes::{
        features,
        tests::create::create_test,
        users::{
            add_permission::add_permission, login::login, register::register,
            remove_permission::remove_permission,
        },
    },
};

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);
#[launch]
pub async fn rocket() -> Rocket<Build> {
    dotenv().unwrap();
    tracing_subscriber::fmt::init();
    init::init().await.expect("Something went wrong!");
    info!("Server initialized!");
    rocket::build().attach(Cors).mount(
        "/api/",
        routes![
            register,
            login,
            features::load::load_features,
            features::set::set_feature,
            add_permission,
            remove_permission,
            create_test
        ],
    )
}
