use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::process::exit;

use aws_config::Region;
use tokio::fs;

mod templates;
mod squire;
mod aws;

async fn generate_html(
    config: &squire::settings::Config,
    region: &Region
) {
    let jinja = templates::environment();
    let template_string = format!("list-s3-{}", config.style);
    let list_object = jinja.get_template(template_string.as_str()).unwrap();
    let html_data = list_object.render(minijinja::context!(
        bucket_name => config.bucket,
        region_name => region.to_string(),
        folder_names => config.prefix,
        ignore_objects => config.ignore,
        proxy_server => config.proxy.to_string(),
    ));
    let mut file = match File::create(&config.filename) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("{:?}", err.source());
            exit(1)
        }
    };
    match file.write_all(html_data.unwrap().as_bytes()) {
        Ok(_) => println!("{:?} has been generated as HTML", &config.filename),
        Err(err) => {
            eprintln!("Failed to write data into {}: {:?}", &config.filename, err);
            exit(1)
        }
    }
}

#[tokio::main]
async fn main() {
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
            exit(1)
        }
    }
    if !bucket_names.contains(&config.bucket) {
        eprintln!("\n{:?}\n\tBucket name is invalid.\n\tExpected one of {:?}\n",
                  &config.bucket, bucket_names);
        exit(1)
    }
    generate_html(&config, &region).await;
    aws::upload_object(&aws_client, &config.bucket, &config.filename).await;
    match fs::remove_file(&config.filename).await {
        Ok(_) => println!("{:?} has been removed", &config.filename),
        Err(err) => eprintln!("{:?}", err)
    }
}
