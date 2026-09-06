default:
    @just --list

# Run all tests with all and default features, plus doctests
test:
	cargo nextest run --all-targets --all-features
	cargo nextest run
	cargo test --doc

# Check every feature independently, including the no-feature build
feature-matrix:
	cargo hack check --workspace --each-feature --no-dev-deps

# Run the tests that need a real terminal, and so cannot run under a harness
test-live:
	cargo nextest run --all-features --run-ignored all -E 'binary(color)'

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

# Print a static CLI completion script; redirect it to your shell's completion directory
completions shell="zsh":
    cargo run --quiet -p noaa_weather_cli -- completions {{shell}}

# Print a static MCP-server completion script; redirect it to your shell's completion directory
completions-mcp shell="zsh":
    cargo run --quiet -p noaa_weather_mcp -- completions {{shell}}

# Generate CLI man pages into <dir> (created if missing; default ./man)
man dir="man":
    cargo run --quiet -p noaa_weather_cli -- man {{dir}}

# Generate MCP-server man pages into <dir> (created if missing; default ./man)
man-mcp dir="man":
    cargo run --quiet -p noaa_weather_mcp -- man {{dir}}

# Install both command-line binaries
install: install-cli install-mcp

# Install the noaa-weather CLI tool
install-cli:
    cargo install --path crates/noaa-weather-cli

# Install the noaa-weather MCP server
install-mcp:
    cargo install --path crates/noaa-weather-mcp

# Build the project in release mode for production
release:
    cargo build --release

# Build with dependency metadata embedded, then scan for known vulnerabilities
audit:
    cargo auditable build --release
    cargo audit bin target/release/noaa-weather
    cargo audit bin target/release/noaa-weather-mcp

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
    cargo run --example basic_usage --manifest-path crates/noaa-weather-client/Cargo.toml

# Run the weather alerts example
example-alerts:
    cargo run --example weather_alerts --manifest-path crates/noaa-weather-client/Cargo.toml

# Run all examples
examples: example-basic example-alerts

# Compile every example without making live NOAA requests
check-examples:
    cargo build --workspace --examples --all-features

# Capture representative live NOAA responses for completeness tests
fixtures:
    crates/noaa-weather-client/tests/fixtures/capture.sh

# Regenerate the CLI guides' human-summary property tables
shown-omitted-docs:
    UPDATE_SHOWN_OMITTED_DOCS=1 cargo test -p noaa_weather_cli --lib summary_tests::shown_omitted_docs::generated_shown_and_omitted_docs_are_current

# Run the pull request validation acceptance gate
verify:
    cargo fmt --all --check
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    cargo clippy -p noaa_weather_client --no-default-features -- -D warnings
    just feature-matrix
    just check-examples
    cargo nextest run --workspace --all-targets --all-features
    cargo test --doc --workspace
    RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features
    cargo build --release
