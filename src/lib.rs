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
