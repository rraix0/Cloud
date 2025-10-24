use aws_sdk_s3::Client;
use aws_sdk_s3::config::{BehaviorVersion, Region};
use aws_config::meta::region::RegionProviderChain;

pub async fn get_connection_minio() -> Client {
    // Load environment variables
    let endpoint_url = std::env::var("AWS_ENDPOINT_URL")
        .expect("AWS_ENDPOINT_URL must be set");
    let region = std::env::var("AWS_REGION")
        .unwrap_or_else(|_| "eu-central-1".to_string());

    // Configure region
    let region_provider = RegionProviderChain::first_try(Region::new(region.clone()));

    // Load base config
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    // Create S3-specific config with force_path_style enabled
    let s3_config = aws_sdk_s3::config::Builder::from(&config)
        .endpoint_url(endpoint_url)
        .force_path_style(true)  // This is the key setting!
        .build();

    let client = Client::from_conf(s3_config);

    // Test connection
    let resp = client.list_buckets().send().await.unwrap();
    println!("{:?}", resp);

    let resp = client.list_objects_v2()
        .bucket("cloud")
        .send().await.unwrap();
    println!("{:?}", resp);

    client
}

/*
use aws_sdk_s3::Client;

pub async fn get_connection_minio() -> Client {

    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let client = aws_sdk_s3::Client::new(&config);


    let resp = client.list_buckets().send().await.unwrap();

    println!("{:?}", resp);


    let resp = client.list_objects()
        .bucket("cloud")
        .send().await.unwrap();

    println!("{:?}", resp);

    return client;
}



 */