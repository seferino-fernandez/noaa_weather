default:
    @just --list

# Run all tests: full feature matrix, default features, and doctests
test:
	cargo nextest run --all-targets --all-features
	cargo nextest run
	cargo test --doc

# Lint the code with Clippy
lint:
    cargo clippy

# Format code with rustfmt and apply Clippy's auto-fixable suggestions
format:
    cargo fmt --all
    cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged

# Build the project in debug mode
build:
    cargo build

# Install the noaa-weather CLI tool
install: install-cli

# Install the noaa-weather CLI tool
install-cli:
    cargo install --path noaa_weather_cli

# Build the project in release mode for production
release:
    cargo build --release

# Build with dependency metadata embedded, then scan for known vulnerabilities
audit:
    cargo auditable build --release
    cargo audit bin target/release/noaa-weather

# Remove the target directory and all build artifacts
clean:
    cargo clean

# Generate and open the project's API documentation in a browser
docs:
    cargo doc --open

# Generate an HTML coverage report from nextest runs and open it in a browser
coverage:
    cargo llvm-cov nextest --open --html

# Run the basic usage example
example-basic:
    cargo run --example basic_usage --manifest-path noaa_weather_client/Cargo.toml

# Run the weather alerts example
example-alerts:
    cargo run --example weather_alerts --manifest-path noaa_weather_client/Cargo.toml

# Run all examples
examples: example-basic example-alerts
