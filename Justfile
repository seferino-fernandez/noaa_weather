default:
  @just --list

# Run all tests
test:
    cargo test

# Lint the code with Clippy
lint:
    cargo clippy

# Format the code using rustfmt with the nightly toolchain
fmt:
    cargo +nightly fmt --all

# Build the project in debug mode
build:
    cargo build

# Build the project in release mode for production
release:
    cargo build --release

# Clean the project directory
clean:
    cargo clean

# Run the basic usage example
example-basic:
    cargo run --example basic_usage --manifest-path noaa_weather_client/Cargo.toml

# Run the weather alerts example
example-alerts:
    cargo run --example weather_alerts --manifest-path noaa_weather_client/Cargo.toml

# Run all examples
examples: example-basic example-alerts
