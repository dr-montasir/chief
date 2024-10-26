use clap::Command;
use std::process::{exit, Command as StdCommand}; // Use a different alias for the std::process::Command

/// Main function for the Chief CLI application.
///
/// This function initializes the command-line interface and processes commands
/// related to managing web applications, including running in development or
/// production modes and running tests.
fn main() {
    let matches = Command::new("Chief CLI") // Command::new for the CLI app
        .version("0.1.0")
        .about("A CLI for managing web applications")
        .subcommand(
            Command::new("run") // Root command for running the application
                .about("Runs the application")
                .subcommand(Command::new("dev").about("Run in development mode")) // Subcommand for development mode
                .subcommand(Command::new("prod").about("Run in production mode")), // Subcommand for production mode
        )
        .subcommand(Command::new("test").about("Runs the tests")) // Test command
        .get_matches();

    // Handle the 'run dev' command
    if let Some(matches) = matches.subcommand_matches("run") {
        if matches.subcommand_matches("dev").is_some() {
            println!("Running the app in development mode...");

            let status = StdCommand::new("cargo") // Spawn a command to run cargo
                .args(&["watch", "-x", "run"]) // Runs cargo watch -x run
                .status()
                .expect("Failed to execute cargo watch");

            if !status.success() {
                eprintln!("Exited with code: {}", status.code().unwrap_or(-1));
                exit(status.code().unwrap_or(-1));
            }
        } else if matches.subcommand_matches("prod").is_some() {
            println!("Running your framework in production mode...");

            let run_status = StdCommand::new("cargo") // Spawn a command to run cargo
                .args(&["run", "--release"]) // Runs cargo run
                .status()
                .expect("Failed to execute cargo run");

            if !run_status.success() {
                eprintln!("Exited with code: {}", run_status.code().unwrap_or(-1));
                exit(run_status.code().unwrap_or(-1));
            }
        }
    }
    // Handle the 'test' command
    else if matches.subcommand_matches("test").is_some() {
        println!("Running your tests...");

        let status = StdCommand::new("cargo") // Spawn a command to run cargo
            .arg("test") // Runs cargo test
            .status()
            .expect("Failed to execute cargo test");

        if !status.success() {
            eprintln!("Exited with code: {}", status.code().unwrap_or(-1));
            exit(status.code().unwrap_or(-1));
        }
    }
    // If no command was provided or recognized
    else {
        println!("No valid command was provided.");
    }
}
