# NOAA Weather client and CLI

## Development

### Running Tests

Run tests:

```sh
cargo test
```

### Linting and Formatting

Run Clippy:

```sh
cargo clippy
```

Run Rustfmt:

```sh
cargo fmt
```

## Usage

View help for the CLI:

```sh
cargo run -- --help
```

Fetch weather data for Phoenix, AZ and save to a file:

```sh
cargo run weather city phoenix az --json -o weather.json
```

## Resources

- [NOAA Weather API](https://www.weather.gov/documentation/services-web-api)
