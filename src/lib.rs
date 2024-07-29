#![allow(rustdoc::bare_urls)]
#![doc = include_str!("../README.md")]

use std;

use aws_config::Region;

mod templates;
mod squire;
mod aws;

/// Generates the HTML file required with template.
///
/// # Arguments
///
/// * `config` - Configuration settings passed as flags or env vars.
/// * `region` - Reference to the AWS region object.
/// * `metadata` - Project's metadata information stored as a struct.
///
/// # Returns
///
/// Returns a `String` response of the HTML page.
async fn generate_html(
    config: &squire::settings::Config,
    region: &Region,
    metadata: &squire::constant::MetaData
) -> String {
    let jinja = templates::environment();
    let template_string = format!("list-s3-{}", config.style);
    let list_object = jinja.get_template(template_string.as_str()).unwrap();
    let html_data = list_object.render(minijinja::context!(
        bucket_name => config.bucket,
        region_name => region.to_string(),
        folder_names => config.filter,
        ignore_objects => config.ignore,
        proxy_server => config.proxy.to_string(),
        cargo_version => metadata.pkg_version
    ));
    html_data.unwrap()
}

/// Initializes the application and uploads the generated HTML to S3.
pub async fn initiate() {
    let metadata = squire::constant::build_info();
    let config = squire::parser::arguments(&metadata);

    let region = aws::get_region(&config).await;
    let aws_client = aws::get_client(&region).await;

    let mut bucket_names = Vec::new();
    match aws::get_buckets(&aws_client).await {
        Some(buckets) => {
            for bucket in buckets {
                bucket_names.push(bucket.name.unwrap());
            }
        }
        None => {
            eprintln!("Failed to fetch S3 buckets.");
            std::process::exit(1)
        }
    }
    if !bucket_names.contains(&config.bucket) {
        eprintln!("\n{:?}\n\tBucket name is invalid.\n\tExpected one of {:?}\n",
                  &config.bucket, bucket_names);
        std::process::exit(1)
    }
    let data = generate_html(&config, &region, &metadata).await;
    aws::upload_object(&aws_client, &config.bucket, &data, &config.object).await;
}
