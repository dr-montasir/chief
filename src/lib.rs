#![doc(
    html_logo_url = "https://github.com/dr-montasir/chief/raw/HEAD/logo.svg?sanitize=true",
    html_root_url = "https://docs.rs/chief/latest/chief"
)]

//! This library provides access to standard environment variables and logging utilities.

/// Re-exporting the `dotenv` crate for loading environment variables from a `.env` file.
///
/// The `dotenv` crate allows you to load environment variables from a `.env` file into the
/// environment for your application. This is particularly useful for managing configuration
/// settings in a development environment without hardcoding values in your source code.
///
/// # Usage
///
/// Call the `dotenv()` function at the beginning of your application to load the environment
/// variables defined in your `.env` file:
///
/// ```rust
/// use chief::dotenv;
///
/// fn main() {
///     dotenv().ok(); // Load environment variables from the .env file
///
///     // Now you can access variables using chief::env
///     let my_var = chief::env::var("MY_VARIABLE").unwrap_or_default();
///     println!("MY_VARIABLE: {}", my_var);
/// }
/// ```
pub use dotenv::dotenv; // Keep original without prefix.

/// Re-exporting the `std::env` module for accessing environment variables.
///
/// The `std::env` module provides functions for accessing and manipulating the environment
/// of your Rust application. This can include retrieving environment variables,
/// getting the current working directory, and managing process-specific information.
///
/// # Usage
///
/// You can access environment variables as follows:
///
/// ```rust
/// use chief::env;
///
/// fn main() {
///     // Accessing an environment variable
///     let my_var = env::var("MY_VARIABLE").unwrap_or_else(|_| {
///         eprintln!("MY_VARIABLE is not set.");
///         String::from("default_value")
///     });
///
///     println!("MY_VARIABLE: {}", my_var);
/// }
/// ```
pub use std::env; // Keep original without prefix.

/// Retrieves the value of an environment variable, providing a default value if the variable is not set.
///
/// The `env_var` function allows you to access environment variables in a safe way,
/// returning a default value if the specified variable is not found. This is useful
/// for providing reasonable defaults in case the environment variable is not defined.
///
/// # Parameters
///
/// - `var_name`: The name of the environment variable to retrieve.
/// - `default`: The default value to return if the environment variable is not set.
///
/// # Returns
///
/// Returns the value of the environment variable as a `String`. If the variable is not set,
/// the provided default value will be returned.
///
/// # Usage
///
/// You can retrieve an environment variable with a default value as follows:
///
/// ```rust
/// use chief::env_var;
///
/// fn main() {
///     // Retrieving an environment variable with a default value
///     let my_var = env_var("MY_VARIABLE", "default_value");
///
///     println!("MY_VARIABLE: {}", my_var);
/// }
/// ```
pub fn env_var(var_name: &str, default: &str) -> String {
    env::var(var_name).unwrap_or_else(|_| default.to_string())
}

/// Loads environment variables from a `.env` file into the application's environment.
///
/// The `load_dotenv` function loads environment variables defined in your `.env` file,
/// making them available to your application. This is particularly useful for managing
/// configuration settings without hardcoding sensitive values directly into your source code.
///
/// # Usage
///
/// Call the `load_dotenv` function at the beginning of your application to ensure that
/// the environment variables are loaded before they are accessed:
///
/// ```rust
/// use chief::{load_dotenv, env_var};
///
/// fn main() {
///     // Load environment variables from the .env file
///     load_dotenv();
///
///     // Use env_var or access directly through std::env
///     let my_var = env_var("MY_VARIABLE", "default_value");
///     println!("MY_VARIABLE: {}", my_var);
/// }
/// ```
pub fn load_dotenv() {
    dotenv().ok(); // Load environment variables from a .env file
}

/// Re-exporting the `log` crate for logging functionalities.
///
/// The `log` crate provides a lightweight logging facade for Rust. Users can set up loggers to capture various
/// log levels and target outputs (e.g., stderr, files, etc.).
///
/// # Usage
///
/// To use the logging features, set the desired log level early in your application, as shown below:
///
/// ```rust
/// use chief::log::{info, warn, error};
///
/// fn main() {
///     info!("This is an info message.");
///     warn!("This is a warning message.");
///     error!("This is an error message.");
/// }
/// ```
///
pub use log;

/// Re-exporting the `simplelog` crate for simplified logging setup.
///
/// The `simplelog` crate provides a simple and straightforward logger implementation.
///
/// # Usage
///
/// You can initialize the logger in your `main` function like this:
///
/// ```rust
/// use chief::log::{info, error};           // Import necessary macros
/// use chief::simplelog::{Config, LevelFilter, SimpleLogger};
///
/// fn main() {
///     SimpleLogger::init(LevelFilter::Info, Config::default()).unwrap(); // Initialize the logger
///     info!("This is an info message.");
///     error!("This is an error message.");
/// }
/// ```
///
pub use simplelog;

use clap::{Arg, Command};
/// ### chief_cli()
///
/// Initializes the Chief command-line interface for managing web applications.
///
/// This function creates a command-line interface using the `clap` library, allowing users
/// to run, test, build, and clean their web applications. It accepts two parameters: the
/// name of the application and its version. The CLI supports subcommands for running the
/// application in different modes (development and production), running tests, building
/// the application (with an option for release mode), and cleaning the project directory.
///
/// **Important:** The `name` parameter passed to `chief_cli` must match the `name` specified
/// in your `Cargo.toml` file. This ensures consistency across your project and allows the
/// CLI to correctly identify the application when commands are executed. For example, if
/// your `Cargo.toml` contains:
///
/// ```toml
/// [package]
/// name = "example"
/// version = "0.1.0"
/// ```
///
/// You should call `chief_cli` with `name` set to `"example"` and `ver` set to `"0.1.0"`.
///
/// When using the `--version` flag in the command line, it will return the output in the format:
/// `<project name> <version>`, reflecting the values provided in the `chief_cli` function.
///
/// ### Parameters
///
/// - `name`: A static string slice representing the name of the application.
/// - `ver`: A static string slice representing the version of the application.
///
/// ### Example
///
/// ```rust
/// use chief::chief_cli;
///
/// fn main() {
///     let app_name = "example";  // Any project name can be here.
///     let app_version = "0.1.0";
///     chief_cli(app_name, app_version);
/// }
/// ```
///
/// ### Command Usage
///
/// The following commands are available through the Chief CLI:
///
/// - `run`
///   - `dev`: Runs the application in development mode.
///   - `prod`: Runs the application in production mode.
/// - `test`: Runs the tests for the application.
/// - `build`
///   - `--release`: Builds the application in release mode (optional flag).
/// - `clean`: Cleans the project directory.
/// - `help`: Print this message or the help of the given subcommand(s).
///
/// ### Returns
///
/// This function does not return a value. It executes the specified commands based on user input
/// and handles any errors that may occur during the execution of the commands. If an invalid command
/// is provided, it will print a message indicating that no valid command was provided.
///
/// <small>End Fun Doc</small>
pub fn chief_cli(name: &'static str, ver: &'static str) {
    let app = Command::new(name)
        .version(ver)
        .about("A CLI for managing web applications")
        .subcommand(
            Command::new("run")
                .about("Runs the application")
                .subcommand(Command::new("dev").about("Run in development mode"))
                .subcommand(Command::new("prod").about("Run in production mode")),
        )
        .subcommand(Command::new("test").about("Runs the tests"))
        .subcommand(
            Command::new("build").about("Builds the application").arg(
                Arg::new("release")
                    .short('r')
                    .long("release")
                    .help("Build in release mode")
                    .action(clap::ArgAction::SetTrue), // Use ArgAction to indicate it's a flag
            ),
        )
        .subcommand(Command::new("clean").about("Cleans the project"));

    let matches = app.get_matches();

    if let Some(matches) = matches.subcommand_matches("run") {
        if matches.subcommand_matches("dev").is_some() {
            println!("Running the app in development mode...");
            let status = std::process::Command::new("cargo")
                .args(&["watch", "-x", "run"])
                .status()
                .expect("Failed to execute cargo watch");

            if !status.success() {
                eprintln!("Exited with code: {}", status.code().unwrap_or(-1));
                std::process::exit(status.code().unwrap_or(-1));
            }
        } else if matches.subcommand_matches("prod").is_some() {
            println!("Running your framework in production mode...");
            let run_status = std::process::Command::new("cargo")
                .args(&["run", "--release"])
                .status()
                .expect("Failed to execute cargo run");

            if !run_status.success() {
                eprintln!("Exited with code: {}", run_status.code().unwrap_or(-1));
                std::process::exit(run_status.code().unwrap_or(-1));
            }
        }
    } else if matches.subcommand_matches("test").is_some() {
        println!("Running your tests...");
        let status = std::process::Command::new("cargo")
            .arg("test")
            .status()
            .expect("Failed to execute cargo test");

        if !status.success() {
            eprintln!("Exited with code: {}", status.code().unwrap_or(-1));
            std::process::exit(status.code().unwrap_or(-1));
        }
    } else if let Some(matches) = matches.subcommand_matches("build") {
        let release = matches.get_flag("release"); // Check if the release flag is present
        if release {
            println!("Building the application in release mode...");
            let status = std::process::Command::new("cargo")
                .args(&["build", "--release"])
                .status()
                .expect("Failed to execute cargo build --release");

            if !status.success() {
                eprintln!("Exited with code: {}", status.code().unwrap_or(-1));
                std::process::exit(status.code().unwrap_or(-1));
            }
        } else {
            println!("Building the application...");
            let status = std::process::Command::new("cargo")
                .arg("build")
                .status()
                .expect("Failed to execute cargo build");

            if !status.success() {
                eprintln!("Exited with code: {}", status.code().unwrap_or(-1));
                std::process::exit(status.code().unwrap_or(-1));
            }
        }
    } else if matches.subcommand_matches("clean").is_some() {
        println!("Cleaning the project...");
        let status = std::process::Command::new("cargo")
            .arg("clean")
            .status()
            .expect("Failed to execute cargo clean");

        if !status.success() {
            eprintln!("Exited with code: {}", status.code().unwrap_or(-1));
            std::process::exit(status.code().unwrap_or(-1));
        }
    } else {
        println!("No valid command was provided.");
    }
}
