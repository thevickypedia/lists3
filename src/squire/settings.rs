use std::process::exit;

/// Represents the configuration parameters for RuStream.
pub struct Config {
    pub bucket: String,
    pub region: String,
    pub prefix: Vec<String>,
    pub ignore: Vec<String>,
    pub filename: String
}


/// Extracts the argument and parses it as a `Vec<String>`
///
/// # Returns
///
/// Returns an `Option<Vec<String>>` if the value is available.
///
/// # Panics
///
/// If the value is present, but it is an invalid data-type.
fn parse_vec(value: &str) -> Option<Vec<String>> {
    match serde_json::from_str::<Vec<String>>(value) {
        Ok(parsed) => Some(parsed),
        Err(err) => {
            if value.is_empty() {
                return None;
            }
            eprintln!("{:?}", err);
            exit(1)
        }
    }
}

pub fn parse_config(
    bucket: String,
    region: String,
    prefix: String,
    ignore: String,
    filename: String
) -> Config {

    if bucket.is_empty() {
        eprintln!("\n--bucket\n\tBucket name is mandatory!!\n");
        exit(1)
    }
    let parsed_prefix = parse_vec(&prefix).unwrap_or(Vec::new());
    let parsed_ignore = parse_vec(&ignore).unwrap_or(Vec::new());
    Config {
        bucket,
        region,
        prefix: parsed_prefix,
        ignore: parsed_ignore,
        filename
    }
}
