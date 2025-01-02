use chief::chief_cli;

/// Main function for the Chief CLI application.
///
/// This function initializes the command-line interface and processes commands
/// related to managing web applications, including running in development or
/// production modes and running tests. The Chief CLI also supports building
/// the application with an optional release flag and cleaning the project
/// directory. Users can easily navigate through these commands to streamline
/// their web application management workflow, enhancing productivity and
/// ensuring a smooth development experience.
///
/// <small>End Fun Doc</small>
fn main() {
    let app_name = "chief";
    let app_version = "1.0.0";
    chief_cli(app_name, app_version);
}
