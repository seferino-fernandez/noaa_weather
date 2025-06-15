# 🌦️ NOAA Weather CLI & Client

[![Build Status](https://github.com/seferino-fernandez/noaa_weather/actions/workflows/pull-request-validation.yml/badge.svg)](https://github.com/seferino-fernandez/noaa_weather/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org)

A comprehensive Rust client library and command-line interface for the [NOAA Weather API](https://www.weather.gov/documentation/services-web-api). Get real-time weather forecasts, alerts, observations, radar data, and aviation weather products directly from the National Weather Service.

> **Note**: This project uses data published by NOAA/NWS but is otherwise unaffiliated with the National Weather Service and is not an official NOAA/NWS library.

## ✨ Features

-   🚨 **Weather Alerts** - Active alerts, warnings, and watches by location or zone
-   🗺️ **Gridpoint Forecasts** - Detailed forecasts with hourly data and weather parameters
-   🌡️ **Observations** - Current conditions from weather stations nationwide
-   📡 **Radar Data** - Access to radar stations, servers, and real-time data
-   ✈️ **Aviation Weather** - SIGMETs, AIRMETs, and Center Weather Advisories
-   🏢 **NWS Offices** - Office information, headlines, and products
-   📋 **Text Products** - Access to all NWS text-based weather products
-   🌍 **Zone Information** - Forecast zones, counties, and geographic areas
-   📍 **Point Data** - Get weather data for any latitude/longitude coordinate

## 🚀 Quick Start

### Installation

#### From Source

```bash
git clone https://github.com/seferino-fernandez/noaa_weather.git
cd noaa_weather
cargo install --path noaa_weather_cli
```

#### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
noaa_weather_client = "0.1.0"
```

### CLI Usage

Get current weather alerts:

```bash
noaa-weather alerts active
```

Get forecast for a specific location:

```bash
noaa-weather points metadata "39.7456,-97.0892"
```

Get latest observations from a weather station:

```bash
noaa-weather stations latest-observation --station-id KJFK
```

Get aviation weather (SIGMETs):

```bash
noaa-weather aviation sigmets
```

### Library Usage

```rust
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::apis::points;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Configuration::new();

    // Get point metadata for coordinates
    let point_data = points::get_point(&config, "39.7456,-97.0892").await?;
    println!("Forecast office: {:?}", point_data.properties.forecast_office);

    Ok(())
}
```

## 📖 Documentation

### Authentication

From the [NOAA Weather API Documentation](https://www.weather.gov/documentation/services-web-api):

> A User Agent is required to identify your application.
> This string can be anything, and the more unique to your application the less likely it will be affected by a security event.
> If you include contact information (website or email), we can contact you if your string is associated to a security event.
> This will be replaced with an API key in the future.
>
> User-Agent: (myweatherapp.com, contact@myweatherapp.com)

### CLI Commands

| Command      | Description                 | Example                                                          |
| ------------ | --------------------------- | ---------------------------------------------------------------- |
| `alerts`     | Weather alerts and warnings | `noaa-weather alerts active --area CA`                           |
| `gridpoints` | Detailed forecast data      | `noaa-weather gridpoints forecast --office-id TOP --x 31 --y 80` |
| `points`     | Point metadata and stations | `noaa-weather points metadata "40.7128,-74.0060"`                |
| `stations`   | Weather station data        | `noaa-weather stations latest-observation --station-id KJFK`     |
| `zones`      | Zone forecasts and info     | `noaa-weather zones forecast --type forecast --id CAZ006`        |
| `radar`      | Radar stations and data     | `noaa-weather radar stations`                                    |
| `aviation`   | Aviation weather products   | `noaa-weather aviation sigmets --atsu KKCI`                      |
| `products`   | NWS text products           | `noaa-weather products types`                                    |
| `offices`    | NWS office information      | `noaa-weather offices metadata --office-id TOP`                  |

### Output Formats

All commands support multiple output formats:

-   **Human-readable tables** (default) - Perfect for terminal viewing
-   **JSON** (`--json`) - Machine-readable for scripting and integration
-   **File output** (`--output file.txt`) - Save results to a file

### General Examples

#### Get Weather Alerts for California

```bash
noaa-weather alerts area --area CA
```

#### Retrieve Metadata for Multiple Weather Stations

```bash
noaa-weather stations list --state CA,NV --limit 50
```

#### Get Aviation Weather for Specific Airport

```bash
noaa-weather aviation cwas --cwsu-id ZLA
```

For detailed documentation on each command, run:

```bash
noaa-weather <command> --help
```

### Get Information for your location

First, get the latitude and longitude of the current location:

```bash
curl "http://ip-api.com/json?fields=lat,lon"
```

Then you can:

Get your location's point metadata:

```bash
noaa-weather points metadata "33.7629,-118.1889"
```

With the point metadata, you can get your location's forecast office ID, and gridpoint coordinates.

Get your location's hourly forecast:

```bash
noaa-weather gridpoints forecast-hourly --office-id LOX --x 155 --y 32
```

## 🏗️ Development

### Prerequisites

-   Rust 2024 edition or later
-   Cargo

### Setup

```bash
git clone https://github.com/seferino-fernandez/noaa_weather.git
cd noaa_weather
```

### Running Tests

```bash
cargo test
```

### Linting and Formatting

```bash
cargo clippy
cargo fmt
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Development Guidelines

-   Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
-   Ensure all tests pass: `cargo test`
-   Run clippy: `cargo clippy`
-   Format code: `cargo fmt`
-   Update documentation for new features

### Reporting Issues

Please use our [GitHub Issues](https://github.com/seferino-fernandez/noaa_weather/issues) to report bugs or request features.

## 📜 License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## 🙏 Acknowledgments

-   [National Weather Service](https://www.weather.gov/) for providing the excellent API
-   [NOAA](https://www.noaa.gov/) for weather data and services
-   The Rust community for excellent crates and tooling

## 📚 Additional Resources

-   [NOAA Weather API Documentation](https://www.weather.gov/documentation/services-web-api)
-   [NOAA Weather API GitHub](https://github.com/weather-gov/api)
-   [National Weather Service](https://www.weather.gov/)
-   [CLI Documentation](docs/cli/)

---
