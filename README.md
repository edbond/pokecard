# Project Overview

This project is an application written in Rust. It is a card trading system that uses image recognition to identify different cards. The application is well-structured and uses several notable libraries and techniques.

## Libraries Used

- **Diesel:** An ORM and query builder for Rust. This is used to manage the application's database connections and queries.
- **Dotenv:** A library that loads environment variables from a `.env` file. This is used to manage database connection details.
- **Anyhow:** A library for flexible error handling. This is used throughout the project.
- **Reqwest:** An ergonomic, asynchronous HTTP client for Rust. This is used to make requests to external services.
- **Futures:** A library that provides the foundations for asynchronous programming in Rust. This is used to manage asynchronous tasks within the project.
- **Rayon:** A data parallelism library for Rust. This is used to handle parallel processing of card images.
- **Tokio:** A runtime for writing reliable, asynchronous, and slim applications with the Rust programming language.
- **Serde:** A framework for serializing and deserializing Rust data structures efficiently and generically.
- **Opencv:** A library that provides bindings to the OpenCV computer vision library. This is used for the image recognition functionality in the project.

## Running the Project

Before running the project, ensure that you have the necessary environment variables set in a `.env` file, as this project uses `dotenv` to load these.

To run the project, use the command `cargo run` from the project's root directory.

Please note that as this is a Rust project, you will need to have Rust and Cargo installed on your system. If you do not have these installed, you can do so by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

## Contributing

Contributions are welcome. Please feel free to open an issue or submit a pull request.