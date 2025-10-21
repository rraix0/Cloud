use std::time::Duration;
use surrealdb::engine::any;
use surrealdb::engine::any::Any;
use surrealdb::opt::auth::Database;
use surrealdb::Surreal;
use tokio::time::sleep;
use std::env;

pub async fn get_connection() -> Surreal<Any> {
    let db: Surreal<Any>;

    loop {
        match any::connect("ws://192.168.0.10:8000/rpc").await {
            Ok(client) => {
                db = client;
                break;
            }
            Err(e) => {
                println!("Couldn't connect to database: {:?}", e);
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
    loop {
        match db.signin(Database {
            namespace: env::var("SURREAL_NAMESPACE").unwrap().as_str(),
            database: env::var("SURREAL_DATABASE").unwrap().as_str(),
            username: env::var("SURREAL_USERNAME").unwrap().as_str(),
            password: env::var("SURREAL_PASSWORD").unwrap().as_str(),
        }).await {
            Ok(_) => {
                println!("Successfully signed into database!");
                break;
            }
            Err(e) => {
                println!("Couldn't connect to database: {:?}", e);
                sleep(Duration::from_secs(5)).await;
            }
        }
    }

    return db;
}