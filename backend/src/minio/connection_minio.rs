use aws_sdk_s3::Client;

pub async fn get_connection_minio() -> Client {

    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let client = aws_sdk_s3::Client::new(&config);

    /*
    let resp = client.list_buckets().send().await.unwrap();

    println!("{:?}", resp);

    let resp = client.list_objects()
        .bucket("cloud")
        .send().await.unwrap();

    println!("{:?}", resp);


     */

    return client;
}
