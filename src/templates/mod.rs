mod list_vanilla;
mod list_bootstrap;

/// Loads the HTML templates' content into a Jinja Environment
///
/// # Returns
///
/// Returns the constructed `Arc` for the `Environment` object, that holds the central configuration state for templates.
/// It is also the container for all loaded templates.
pub fn environment() -> minijinja::Environment<'static> {
    let mut env = minijinja::Environment::new();
    env.add_template_owned("list-s3-vanilla", list_vanilla::get_content()).unwrap();
    env.add_template_owned("list-s3-bootstrap", list_bootstrap::get_content()).unwrap();
    env
}
