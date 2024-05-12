use std::process::exit;
use url::Url;

/// Represents the configuration parameters for ``lists3``.
pub struct Config {
    pub bucket: String,
    pub region: String,
    pub prefix: Vec<String>,
    pub ignore: Vec<String>,
    pub filename: String,
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
/// If the value is present, but it is an invalid data-type.
fn parse_vec(value: &str) -> Option<Vec<String>> {
    if value.is_empty() {
        return None;
    }
    match serde_json::from_str::<Vec<String>>(value) {
        Ok(parsed) => Some(parsed),
        Err(err) => {
            eprintln!("{:?}", err);
            exit(1)
        }
    }
}

fn parse_url(string: &str) -> Url {
    if string.is_empty() {
        return Url::parse("https://jarvis.vigneshrao.com/proxy").unwrap()
    }
    Url::parse(string).unwrap_or_else(|err| {
        eprintln!("Parse error: {:?}", err);
        exit(1)
    })
}

pub fn parse_config(
    bucket: String,
    region: String,
    prefix: String,
    ignore: String,
    filename: String,
    proxy: String,
    style: String
) -> Config {
    if bucket.is_empty() {
        eprintln!("\n--bucket\n\tBucket name is mandatory!!\n");
        exit(1)
    }
    let parsed_prefix = parse_vec(&prefix).unwrap_or_default();
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
        prefix: parsed_prefix,
        ignore: parsed_ignore,
        filename,
        proxy: parsed_url,
        style: parsed_style
    }
}
