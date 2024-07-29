use std::error::Error;
use std::process::exit;

use aws_config::meta::region::RegionProviderChain;
use aws_config::Region;
use aws_sdk_s3::Client;
use aws_sdk_s3::primitives::{ByteStream, SdkBody};
use aws_sdk_s3::types::Bucket;

use crate::squire;

/// Creates an AWS S3 client using the specified region.
///
/// # Arguments
///
/// * `region` - A reference to the AWS region to be used for the client.
///
/// # Returns
///
/// Returns an instance of `Client` configured for the specified region.
pub async fn get_client(
    region: &Region
) -> Client {
    let config = aws_config::from_env()
        .region(region.clone())
        .load().await;
    Client::new(&config)
}

/// Determines the AWS region to be used based on the provided configuration.
///
/// # Arguments
///
/// * `config` - Configuration settings passed as flags or env vars.
///
/// # Returns
///
/// Returns an instance of `Region` based on the configuration or the default provider.
pub async fn get_region(
    config: &squire::settings::Config
) -> Region {
    if config.region.is_empty() {
        let region_provider = RegionProviderChain::default_provider()
            .or_else("us-east-1");
        // let region_name = region_provider.region().await.unwrap().to_string();
        region_provider.region().await.unwrap()
    } else {
        Region::new(config.region.to_string())
    }
}

/// Retrieves a list of all S3 buckets in the configured region.
///
/// # Arguments
///
/// * `client` - A reference to the AWS S3 client.
///
/// # Returns
///
/// Returns an `Option<Vec<Bucket>>` containing the list of buckets if successful.
pub async fn get_buckets(
    client: &Client
) -> Option<Vec<Bucket>> {
    match client.list_buckets().send().await {
        Ok(output) => output.buckets,
        Err(error) => {
            eprintln!("{:?}", error.source().unwrap());
            exit(1)
        }
    }
}

/// Uploads an object to the specified S3 bucket.
///
/// # Arguments
///
/// * `client` - A reference to the AWS S3 client.
/// * `bucket_name` - The name of the bucket to upload the object to.
/// * `data` - The content of the object to be uploaded.
/// * `file_name` - The name of the object to be uploaded.
pub async fn upload_object(
    client: &Client,
    bucket_name: &String,
    data: &String,
    file_name: &String,
) {
    let byte_stream = ByteStream::new(SdkBody::from(data.to_owned()));
    match client
        .put_object()
        .bucket(&bucket_name.to_string())
        .key(&file_name.to_string())  // object name
        .body(byte_stream)
        .content_type("text/html")
        .send()
        .await {
        Ok(_) => println!("{:?} has been uploaded as HTML", &file_name),
        Err(err) => {
            eprintln!("Unable to upload to S3: {:?}", err.source().unwrap());
            exit(1)
        }
    }
}
