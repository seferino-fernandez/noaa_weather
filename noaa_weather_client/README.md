# NOAA Weather Client Library

A comprehensive Rust client library for the [NOAA Weather API](https://www.weather.gov/documentation/services-web-api). This crate provides type-safe access to weather forecasts, alerts, observations, radar data, and aviation weather products from the National Weather Service.

## Features

-   🌡️ **Complete API Coverage** - All NOAA Weather API endpoints
-   🚨 **Weather Alerts** - Active alerts, warnings, and watches
-   🗺️ **Gridpoint Forecasts** - Detailed forecasts with hourly data
-   📡 **Radar Data** - Real-time radar information
-   ✈️ **Aviation Weather** - SIGMETs, AIRMETs, and Center Weather Advisories
-   🏢 **NWS Offices** - Office information and products
-   📍 **Point Data** - Weather data for any coordinates
-   🔄 **Async/Await** - Built on `tokio` and `reqwest`
-   🛡️ **Type Safety** - Comprehensive data models with serde

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
noaa_weather_client = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

### Basic Example

```rust
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::apis::{points, alerts};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Configuration::new();

    // Get point metadata for coordinates (latitude, longitude)
    let point = points::get_point(&config, "39.7456,-97.0892").await?;
    println!("Forecast office: {:?}", point.properties.forecast_office);

    // Get active weather alerts
    let alerts = alerts::get_active_alerts(&config, None, None, None, None, None, None, None).await?;
    println!("Active alerts: {}", alerts.features.len());

    Ok(())
}
```

### Weather Alerts

```rust
use noaa_weather_client::apis::{configuration::Configuration, alerts};

let config = Configuration::new();

// Get all active alerts
let active_alerts = alerts::get_active_alerts(&config, None, None, None, None, None, None, None).await?;

// Get alerts for a specific area (state/territory)
let ca_alerts = alerts::get_active_alerts_for_area(&config, "CA").await?;

// Get alerts for a specific zone
let zone_alerts = alerts::get_active_alerts_for_zone(&config, "CAZ006").await?;
```

### Weather Observations

```rust
use noaa_weather_client::apis::{configuration::Configuration, stations};

let config = Configuration::new();

// Get latest observation for a station
let observation = stations::get_latest_observation(&config, "KJFK", None).await?;
println!("Temperature: {:?}", observation.properties.temperature);

// Get station metadata
let station = stations::get_observation_station(&config, "KJFK").await?;
println!("Station name: {:?}", station.properties.name);
```

### Forecasts

```rust
use noaa_weather_client::apis::{configuration::Configuration, gridpoints};

let config = Configuration::new();

// Get forecast for specific gridpoint
let forecast = gridpoints::get_gridpoint_forecast(&config, "TOP", 31, 80, None).await?;

// Get hourly forecast
let hourly = gridpoints::get_gridpoint_forecast_hourly(&config, "TOP", 31, 80, None).await?;
```

## Configuration

The `Configuration` struct provides default settings that work out of the box:

```rust
use noaa_weather_client::apis::configuration::Configuration;

// Use default configuration
let config = Configuration::new();

// Or customize the client
let config = Configuration {
    base_path: "https://api.weather.gov".to_owned(),
    user_agent: Some("my-app/1.0".to_owned()),
    client: reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?,
    ..Default::default()
};
```

## Error Handling

All API functions return `Result<T, Error<E>>` where `E` is the specific error type for that endpoint:

```rust
use noaa_weather_client::apis::{configuration::Configuration, points};

let config = Configuration::new();

match points::get_point(&config, "invalid-point").await {
    Ok(point_data) => println!("Success: {:?}", point_data),
    Err(error) => {
        eprintln!("Error: {}", error);
        // Handle specific error types
        match error {
            noaa_weather_client::apis::Error::ResponseError(response) => {
                eprintln!("HTTP {}: {}", response.status, response.content);
            }
            _ => eprintln!("Other error: {}", error),
        }
    }
}
```

## API Coverage

This client covers all major NOAA Weather API endpoints:

| Module       | Description                 | Key Functions                                             |
| ------------ | --------------------------- | --------------------------------------------------------- |
| `alerts`     | Weather alerts and warnings | `get_active_alerts`, `get_alert`                          |
| `points`     | Point metadata and stations | `get_point`, `get_point_stations`                         |
| `gridpoints` | Detailed forecasts          | `get_gridpoint_forecast`, `get_gridpoint_forecast_hourly` |
| `stations`   | Weather stations            | `get_observation_station`, `get_latest_observation`       |
| `zones`      | Forecast zones              | `get_zone`, `get_zone_forecast`                           |
| `offices`    | NWS offices                 | `get_office`, `get_office_headlines`                      |
| `radar`      | Radar data                  | `get_radar_stations`, `get_radar_servers`                 |
| `aviation`   | Aviation weather            | `get_sigmets`, `get_center_weather_advisories`            |
| `products`   | Text products               | `get_product_types`, `get_products`                       |

## Rate Limiting

The NOAA Weather API doesn't require authentication but has rate limits. This client:

-   Reuses HTTP connections for efficiency
-   Provides proper User-Agent headers
-   Handles rate limit responses gracefully

## Contributing

We welcome contributions! Please see our [Contributing Guidelines](https://github.com/seferino-fernandez/noaa_weather/blob/main/CONTRIBUTING.md).

## License

This project is licensed under the MIT License - see the [LICENSE.md](https://github.com/seferino-fernandez/noaa_weather/blob/main/LICENSE.md) file for details.

## Disclaimer

This project uses data published by NOAA/NWS but is otherwise unaffiliated with the National Weather Service and is not an official NWS library.

## Resources

-   [NOAA Weather API Documentation](https://www.weather.gov/documentation/services-web-api)
-   [NOAA Weather API GitHub](https://github.com/weather-gov/api)
-   [National Weather Service](https://www.weather.gov/)
