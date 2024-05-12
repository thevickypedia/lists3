use std::error::Error;
use std::process::exit;

use aws_config::meta::region::RegionProviderChain;
use aws_config::Region;
use aws_sdk_s3::Client;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::Bucket;

use crate::squire;

pub async fn get_client(
    region: &Region
) -> Client {
    let config = aws_config::from_env()
        .region(region.clone())
        .load().await;
    Client::new(&config)
}

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

pub async fn upload_object(
    client: &Client,
    bucket_name: &String,
    data: &String,
    file_name: &String,
) {
    let bytes = data.to_string().into_bytes();
    let bytes_static: &'static [u8] = unsafe { std::mem::transmute(bytes.as_slice()) };
    let body = ByteStream::from_static(bytes_static);
    match client
        .put_object()
        .bucket(&bucket_name.to_string())
        .key(&file_name.to_string())  // object name
        .body(body)
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
