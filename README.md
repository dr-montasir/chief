# Chief

[<img alt="github" src="https://img.shields.io/badge/github-dr%20montasir%20/%20chief-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">](https://github.com/dr-montasir/chief)[<img alt="crates.io" src="https://img.shields.io/crates/v/chief.svg?style=for-the-badge&color=fc8d62&logo=rust" height="22">](https://crates.io/crates/chief)[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-chief-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="22">](https://docs.rs/chief)[<img alt="license" src="https://img.shields.io/badge/license-apache_2.0-4a98f7.svg?style=for-the-badge&labelColor=555555&logo=apache" height="22">](https://choosealicense.com/licenses/apache-2.0)[<img alt="license" src="https://img.shields.io/badge/license-mit-4a98f7.svg?style=for-the-badge&labelColor=555555" height="22">](https://choosealicense.com/licenses/mit)

**Chief Development Tools** provides versatile functionalities for managing web applications. Depending on your requirements, you can choose between different installation methods. Whether you want to utilize the command-line interface (CLI) for seamless application management or integrate logging and environment variable handling into your project, Chief offers flexible options tailored to your development needs. This empowers developers to enhance their workflow and streamline application processes efficiently.

## Table of Contents

- [Features](#features)
- [Getting Started](#getting-started)
- [Commands](#commands)
- [Usage Examples](#usage-examples)
- [Logging](#logging)
- [Environment Variables](#environment-variables)
- [License](#license)
- [Contributing](#contributing)
- [Author](#author)

## Features

- **CLI for Application Management**: Easily run applications in development or production mode and execute tests.
- **Integrated Logging**: Utilize the `log` crate for logging at various levels and configure with the `simplelog` crate for easy setup.
- **Environment Variable Management**: Access standard environment variables effortlessly.

## Getting Started

### Installation for Running and Testing Only

If your primary focus is to run commands (such as `run dev`, `run prod`) and execute tests without needing to manage environment variables or logging, you can install Chief directly using:

```terminal
cargo install chief
```

This command installs the CLI tool, enabling you to run your application and execute tests easily.

### Installation for Logging and Environment Management

**For developers** who only need to manage environment variables and logging functionalities, you can add Chief as a dependency to your project without the CLI commands using:

**To include Environment Variable Management and Dotenv:**

```terminal
cargo add chief
```

**Or add `chief` to your `Cargo.toml` file:**

```toml
[dependencies]
chief = "MAJOR.MINOR.PATCH" # Replace with the latest version
```

### Building from Source

To build the `Chief` Development Tools from source, follow these commands:

```terminal
git clone https://github.com/dr-montasir/chief.git
cd chief
cargo build --release
```

## Commands

The following subcommands are available in the CLI:

- `run`: Runs the application.

  - `run dev`: Runs the application in development mode.
  - `run prod`: Runs the application in production mode.

- `test`: Runs the tests for the application.

## Usage Examples

To run the application in development mode:

```terminal
chief run dev
```

To run the application in production mode:

```terminal
chief run prod
```

To execute tests:

```terminal
chief test
```

## Logging

Chief Development Tools uses the `log` crate to facilitate logging functionalities. You can set up logging simply in your main application:

```rust
use chief::log::{info, warn, error};
use chief::simplelog::{Config, LevelFilter, SimpleLogger};

fn main() {
    // Initialize the SimpleLogger
    SimpleLogger::init(LevelFilter::Info, Config::default()).unwrap();

    info!("This is an info message.");
    warn!("This is a warning message.");
    error!("This is an error message.");
}
```

## Environment Variables

Accessing environment variables in your application can be done using the `chief::{dotenv, env}` modules. Here is a simple way to get an environment variable:

```text
# .env file
MY_ENV_VARIABLE=my_variable
```

```rust
use chief::{dotenv, env};

fn main() {
    dotenv().ok();
    if let Ok(value) = env::var("MY_ENV_VARIABLE") {
        println!("My environment variable is: {}", value);
    } else {
        println!("Environment variable not set.");
    }
}
// Output:
// My environment variable is: my_variable
```

## License

This project is licensed under either of the following licenses:

- MIT License
- Apache License, Version 2.0

You may choose either license for your purposes.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue for any feature requests or bug reports.

## Author

[Dr. Montasir Mirghani](https://github.com/dr-montasir)
