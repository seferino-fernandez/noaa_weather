# NOAA Weather CLI & Client

[![Build Status](https://github.com/seferino-fernandez/noaa_weather/actions/workflows/pull-request-validation.yml/badge.svg)](https://github.com/seferino-fernandez/noaa_weather/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org)

A Rust-idiomatic client library and command-line interface for version 3.11.0 of the [NOAA Weather API](https://www.weather.gov/documentation/services-web-api). Get forecasts, alerts, observations, office briefings and weather stories, radar telemetry, aviation products, and NOAA Weather Radio data directly from the National Weather Service.

> **Note**: This project uses data published by NOAA/NWS but is otherwise unaffiliated with the National Weather Service and is not an official NOAA/NWS library.

## Features

- **Weather Alerts** - Active alerts, warnings, and watches by location or zone
- **Gridpoint Forecasts** - Detailed 12-hour and hourly forecasts with weather parameters
- **Observations** - Current conditions from weather stations nationwide
- **Radar Data** - Radar stations, servers, queues, and SPGDS telemetry
- **Aviation Weather** - SIGMETs, AIRMETs, and Center Weather Advisories
- **NWS Offices** - Office information, headlines, briefings, and weather stories
- **Text Products** - Area Forecast Discussions, watches, and all NWS text products
- **Zone Information** - Forecast zones, counties, and geographic areas
- **Point Data** - Get weather data for any latitude/longitude coordinate
- **NOAA Weather Radio** - Transmitter metadata and broadcast transcripts (enabled by default; opt out with `default-features = false`)
- **Glossary** - Typed NWS glossary terms

## Quick Start

### Installation

#### Homebrew

Install the `noaa-weather` CLI tool using Homebrew:

```bash
brew tap seferino-fernandez/tools
brew install noaa-weather
```

#### From Source

```bash
git clone https://github.com/seferino-fernandez/noaa_weather.git
cd noaa_weather
cargo install --path noaa_weather_cli
```

#### As a Library

```bash
cargo add noaa_weather_client
```

The default features include NOAA Weather Radio support. To build only the JSON API surface, disable default features:

```bash
cargo add noaa_weather_client --no-default-features
```

## Documentation

See the [CLI guide](noaa_weather_cli/README.md), [client guide](noaa_weather_client/README.md), and [3.11 migration report](notes/noaa-v3.11.0-migration.md).

## Development

### Prerequisites

- Rust 2024 edition or later
- Cargo

### Setup

```bash
git clone https://github.com/seferino-fernandez/noaa_weather.git
cd noaa_weather
```

### Running Tests

```bash
just verify
```

### Linting and Formatting

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

## Contributing

I welcome any and all contributions!

### Reporting Issues

Please use our [GitHub Issues](https://github.com/seferino-fernandez/noaa_weather/issues) to report bugs or request features.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Additional Resources

- [NOAA Weather API Documentation](https://www.weather.gov/documentation/services-web-api)
- [NOAA Weather API GitHub](https://github.com/weather-gov/api)
- [National Weather Service](https://www.weather.gov/)
