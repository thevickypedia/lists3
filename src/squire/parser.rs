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
    let mut filter = String::new();
    let mut ignore = String::new();
    let mut object = String::new();
    let mut proxy = String::new();
    let mut style = String::new();

    // Loop through the command-line arguments and parse them.
    let mut i = 1; // Start from the second argument (args[0] is the program name).
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                let options: Vec<_> = vec![
                    ("--bucket | -b", "Bucket name for which listing has to be created"),
                    ("--region | -r", "Region name where the bucket is present"),
                    ("--filter | -f", "S3 prefix to filter (eg: '[\"github/\"]')"),
                    ("--ignore | -i", "Objects to be ignored (eg: '[\"github/.DS_Store\"]')"),
                    ("--object | -o", "Object name to upload in s3 (eg: list.html)"),
                    ("--proxy | -p", "Proxy server's path (eg: https://example.com/proxy)"),
                    ("--style | -s", "Styling for the UI (eg: vanilla)"),
                    ("--version | -v", "Get the package version.")
                ].iter().cloned().collect();
                let longest_key = options.iter().map(|(k, _)| k.len()).max().unwrap_or(0);
                let pretext = "\n* ";
                let choices: String = options.iter().map(|(k, v)| {
                    format!("{}{} {}→ {}", pretext, k, "·".repeat(longest_key - k.len() + 8), v)
                }).collect::<Vec<_>>().join("");
                println!("Usage: {} [OPTIONS]\n\nlists3 takes the following arguments\n{}", args[0], choices);
                exit(0)
            }
            "-V" | "-v" | "--version" => {
                version = true;
            }
            "-b" | "--bucket" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    bucket = args[i].clone();
                } else {
                    eprintln!("--bucket requires a value.");
                    exit(1)
                }
            }
            "-r" | "--region" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    region = args[i].clone();
                } else {
                    eprintln!("--region requires a value.");
                    exit(1)
                }
            }
            "-f" | "--filter" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    filter = args[i].clone();
                } else {
                    eprintln!("--filter requires a value.");
                    exit(1)
                }
            }
            "-i" | "--ignore" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    ignore = args[i].clone();
                } else {
                    eprintln!("--ignore requires a value.");
                    exit(1)
                }
            }
            "-o" | "--object" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    object = args[i].clone();
                } else {
                    eprintln!("--object requires a value.");
                    exit(1)
                }
            }
            "-p" | "--proxy" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    proxy = args[i].clone();
                } else {
                    eprintln!("--proxy requires a value.");
                    exit(1)
                }
            }
            "-s" | "--style" => {
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
    if object.is_empty() {
        object = "list".to_string()
    }
    settings::parse_config(bucket, region, filter, ignore, object, proxy, style)
}
