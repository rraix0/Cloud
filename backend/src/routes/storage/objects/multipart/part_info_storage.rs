use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use aws_sdk_s3::types::CompletedMultipartUpload;
use surrealdb::sql::Thing;
use tokio::task::JoinHandle;


#[derive(Clone)]
pub struct PartInfoStorageData {
    pub completed_multipart_upload: CompletedMultipartUpload,
    pub object_id: String,
    pub key: String,
    pub title: String,
    pub folder: Thing,
    pub size: u64,
    pub parts_size: HashMap<i32, i64>,
}

pub struct PartInfoStorage {
    pub data: PartInfoStorageData,
    pub auto_destruction: JoinHandle<()>
}
pub type PartInfoHashMap = HashMap<(String, String), PartInfoStorage>;
pub type SharedStorage = Arc<Mutex<PartInfoHashMap>>;

pub static STORAGE: LazyLock<SharedStorage> = LazyLock::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});