use std::process::exit;
use url::Url;

/// Represents the configuration parameters for ``lists3``.
pub struct Config {
    pub bucket: String,
    pub region: String,
    pub filter: Vec<String>,
    pub ignore: Vec<String>,
    pub object: String,
    pub proxy: Url,
    pub style: String
}


/// Extracts the argument and parses it as a `Vec<String>`
///
/// # Returns
///
/// Returns an `Option<Vec<String>>` if the value is available.
///
/// # Panics
///
/// This function will print an error message and terminate the program if the value is present, but it is an invalid data-type.
fn parse_vec(value: &str) -> Option<Vec<String>> {
    if value.is_empty() {
        return None;
    }
    match serde_json::from_str::<Vec<String>>(value) {
        Ok(mut parsed) => {
            for elem in &mut parsed {
                if !elem.ends_with('/') {
                    elem.push('/');
                }
            }
            Some(parsed)
        }
        Err(err) => {
            eprintln!("{:?}", err);
            exit(1)
        }
    }
}

/// Extracts the argument and parses it as a `Url`
///
/// # Returns
///
/// Returns a `Url` if the value is available.
///
/// # Panics
///
/// This function will print an error message and terminate the program if the value is not a valid URL.
fn parse_url(string: &str) -> Url {
    if string.is_empty() {
        return Url::parse("https://jarvis.vigneshrao.com/proxy").unwrap()
    }
    Url::parse(string).unwrap_or_else(|err| {
        eprintln!("Parse error: {:?}", err);
        exit(1)
    })
}

/// Parses the configuration for the application and returns a `Config` struct.
///
/// # Arguments
///
/// * `bucket` - The name of the S3 bucket. This is a mandatory field.
/// * `region` - The AWS region where the bucket is located.
/// * `filter` - A string representing filters to be applied, which will be parsed into a vector.
/// * `ignore` - A string representing patterns to be ignored, which will be parsed into a vector.
/// * `object` - The name of the object to be operated on.
/// * `proxy` - The proxy URL to be used, which will be parsed.
/// * `style` - The style option for the output. Expected values are "bootstrap" or "vanilla".
///
/// # Returns
///
/// Returns a `Config` struct populated with the parsed values.
///
/// # Panics
///
/// This function will print an error message and terminate the program if the `bucket` field is empty or if the `style`
/// option is invalid.
pub fn parse_config(
    bucket: String,
    region: String,
    filter: String,
    ignore: String,
    object: String,
    proxy: String,
    style: String
) -> Config {
    if bucket.is_empty() {
        eprintln!("\n--bucket\n\tBucket name is mandatory!!\n");
        exit(1)
    }
    let parsed_filter = parse_vec(&filter).unwrap_or_default();
    let parsed_ignore = parse_vec(&ignore).unwrap_or_default();
    let parsed_url = parse_url(&proxy);

    let styling = vec!["bootstrap".to_string(), "vanilla".to_string()];
    let parsed_style;
    if style.is_empty() {
        parsed_style = styling.first().unwrap().to_string()
    } else if styling.contains(&style) {
        parsed_style = style.clone().to_lowercase()
    } else {
        eprintln!("\n{:?}\n\tStyling option is invalid.\n\tExpected one of {:?}", style, styling);
        exit(1)
    }

    Config {
        bucket,
        region,
        filter: parsed_filter,
        ignore: parsed_ignore,
        object,
        proxy: parsed_url,
        style: parsed_style
    }
}
