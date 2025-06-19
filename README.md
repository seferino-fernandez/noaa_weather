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

or add to your `Cargo.toml` manually:

```toml
[dependencies]
noaa_weather_client = "0.1.0"
```

## 📖 Documentation

See the [noaa_weather_cli](noaa_weather_cli/README.md) and [noaa_weather_client](noaa_weather_client/README.md) documentation for more details.

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

I welcome any and all contributions!

### Reporting Issues

Please use our [GitHub Issues](https://github.com/seferino-fernandez/noaa_weather/issues) to report bugs or request features.

## 📜 License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## 📚 Additional Resources

-   [NOAA Weather API Documentation](https://www.weather.gov/documentation/services-web-api)
-   [NOAA Weather API GitHub](https://github.com/weather-gov/api)
-   [National Weather Service](https://www.weather.gov/)

---
