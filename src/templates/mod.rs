mod list;

/// Loads the HTML templates' content into a Jinja Environment
///
/// # Returns
///
/// Returns the constructed `Arc` for the `Environment` object, that holds the central configuration state for templates.
/// It is also the container for all loaded templates.
pub fn environment() -> minijinja::Environment<'static> {
    let mut env = minijinja::Environment::new();
    env.add_template_owned("list-s3", list::get_content()).unwrap();
    env
}
