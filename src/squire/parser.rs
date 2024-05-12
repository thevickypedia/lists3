use std::env;
use std::process::exit;

use crate::squire::{constant, settings};

/// Parses and returns the command-line arguments.
///
/// # Returns
///
/// A String notion of the argument, `env_file` if present.
pub fn arguments(
    metadata: &constant::MetaData
) -> settings::Config {
    let args: Vec<String> = env::args().collect();

    let mut version = false;
    let mut bucket = String::new();
    let mut region = String::new();
    let mut prefix = String::new();
    let mut ignore = String::new();
    let mut filename = String::new();
    let mut proxy = String::new();
    let mut style = String::new();

    // Loop through the command-line arguments and parse them.
    let mut i = 1; // Start from the second argument (args[0] is the program name).
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                let helper = "lists3 takes the following arguments\n\n\
                --bucket: Bucket name for which listing has to be created\tMANDATORY!!\n\
                --region: Region name where the bucket is present\t\tFallback: Default Region\n\
                --prefix: S3 prefix name to filter (eg: '[\"github/\"]')\tFallback: []\n\
                --ignore: Objects to be ignored (eg: '[\"github/.DS_Store\"]')\tFallback: []\n\
                --filename: Object name to upload in s3 (eg: list.html)\tFallback: list\n\
                --proxy: Proxy server's path (eg: https://example.com/proxy)\tFallback: https://jarvis.vigneshrao.com/proxy\n\
                --style: Styling for the UI (eg: vanilla)\tFallback: bootstrap\n\
                --version: Get the package version.\n".to_string();
                println!("Usage: {} [OPTIONS]\n\n{}", args[0], helper);
                exit(0)
            }
            "-V" | "-v" | "--version" => {
                version = true;
            }
            "--bucket" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    bucket = args[i].clone();
                } else {
                    eprintln!("--bucket requires a value.");
                    exit(1)
                }
            }
            "--region" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    region = args[i].clone();
                } else {
                    eprintln!("--region requires a value.");
                    exit(1)
                }
            }
            "--prefix" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    prefix = args[i].clone();
                } else {
                    eprintln!("--prefix requires a value.");
                    exit(1)
                }
            }
            "--ignore" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    ignore = args[i].clone();
                } else {
                    eprintln!("--ignore requires a value.");
                    exit(1)
                }
            }
            "--filename" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    filename = args[i].clone();
                } else {
                    eprintln!("--filename requires a value.");
                    exit(1)
                }
            }
            "--proxy" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    proxy = args[i].clone();
                } else {
                    eprintln!("--proxy requires a value.");
                    exit(1)
                }
            }
            "--style" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    style = args[i].clone();
                } else {
                    eprintln!("--style requires a value.");
                    exit(1)
                }
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                exit(1)
            }
        }
        i += 1;
    }
    if version {
        println!("{} {}", &metadata.pkg_name, &metadata.pkg_version);
        exit(0)
    }
    if filename.is_empty() {
        filename = "list".to_string()
    }
    settings::parse_config(bucket, region, prefix, ignore, filename, proxy, style)
}
