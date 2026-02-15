use std::error::Error;
use std::process::exit;

use aws_config::meta::region::RegionProviderChain;
use aws_config::Region;
use aws_sdk_s3::Client;
use aws_sdk_s3::types::{CorsConfiguration, CorsRule};
use aws_sdk_s3::primitives::{ByteStream, SdkBody};
use aws_sdk_s3::types::Bucket;
use url::Url;
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
        .bucket(bucket_name.to_string())
        .key(file_name.to_string())  // object name
        .body(byte_stream)
        .content_type("text/html")
        .send()
        .await {
        Ok(_) => println!("{:?} has been uploaded as HTML to {:?}", &file_name, &bucket_name),
        Err(err) => {
            eprintln!("Unable to upload to S3: {:?}", err.source().unwrap());
            exit(1)
        }
    }
}

/// Updates CORS configuration to include the website as allowed origin.
///
/// # Arguments
///
/// * `client` - A reference to the AWS S3 client.
/// * `bucket_name` - The name of the bucket to upload the object to.
/// * `website` - Website URL to add to CORS as allowed origin.
pub async fn update_cors(
    client: &Client,
    bucket_name: &String,
    website: &Url
) {
    let origin = website.to_string().strip_suffix("/").unwrap().to_string();

    // Build the new rule
    let new_rule = match CorsRule::builder()
        .allowed_origins(&origin)
        .allowed_methods("GET")
        .max_age_seconds(3000)
        .build()
    {
        Ok(rule) => rule,
        Err(err) => {
            eprintln!("Unable to create CORS rule: {:?}", err.source().unwrap());
            exit(1)
        },
    };

    // Get existing CORS config (if any)
    let existing_rules = match client
        .get_bucket_cors()
        .bucket(bucket_name)
        .send()
        .await
    {
        Ok(output) => output.cors_rules.unwrap_or_default(),
        Err(_) => {
            // No existing config or other error — treat as empty
            Vec::new()
        }
    };

    // Check if equivalent rule already exists
    let rule_exists = existing_rules.iter().any(|rule| {
        rule.allowed_origins()
            .contains(&origin)
        &&
        rule.allowed_methods()
            .contains(&"GET".to_string())
        &&
        rule.max_age_seconds() == Some(3000)
    });

    if rule_exists {
        println!("CORS rule already exists for {}", origin);
        return;
    }

    // Merge new rule
    let mut updated_rules = existing_rules;
    updated_rules.push(new_rule);

    let cors_config = match CorsConfiguration::builder()
        .set_cors_rules(Some(updated_rules))
        .build()
    {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Unable to create CORS config: {:?}", err.source().unwrap());
            exit(1)
        },
    };

    match client
        .put_bucket_cors()
        .bucket(bucket_name)
        .cors_configuration(cors_config)
        .send()
        .await
    {
        Ok(_) => println!("CORS configuration has been updated for: {:?}", origin),
        Err(err) => {
            eprintln!("Unable to update CORS configuration: {:?}", err.source().unwrap());
            exit(1)
        },
    }
}
