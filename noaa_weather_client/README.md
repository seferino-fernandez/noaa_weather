# NOAA Weather Client Library

A comprehensive Rust client library for the [NOAA Weather API](https://www.weather.gov/documentation/services-web-api). This crate provides type-safe access to weather forecasts, alerts, observations, radar data, and aviation weather products from the National Weather Service.

## Features

- **Complete API Coverage** - All NOAA Weather API endpoints
- **Weather Alerts** - Active alerts, warnings, and watches
- **Gridpoint Forecasts** - Detailed 12-hour and hourly forecasts
- **Radar Data** - Real-time radar information
- **Aviation Weather** - SIGMETs, AIRMETs, and Center Weather Advisories
- **NWS Offices** - Office information and products
- **Point Data** - Weather data for any coordinates
- **NOAA Weather Radio** - Broadcast transcripts (opt-in via `radio` feature)
- **NWS Text Products** - Area Forecast Discussions, watches, and more
- **Async/Await** - Built on `tokio` and `reqwest`
- **Type Safety** - Comprehensive data models with serde

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
noaa_weather_client = "1.0.0"
```

To enable NOAA Weather Radio support (requires the `quick-xml` dependency):

```toml
[dependencies]
noaa_weather_client = { version = "1.0.0", features = ["radio"] }
```

### Running Examples

You can run the provided examples to see the library in action:

```bash
# Using Just (recommended)
just example-basic     # Run basic usage example
just example-alerts    # Run weather alerts example
just examples          # Run all examples

# Or using Cargo directly
cargo run --example basic_usage --manifest-path noaa_weather_client/Cargo.toml
cargo run --example weather_alerts --manifest-path noaa_weather_client/Cargo.toml
```

### Basic Example

```rust,no_run
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::apis::{points, alerts};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Configuration::default();

    // Get point metadata for coordinates (latitude, longitude)
    let point = points::get_point(&config, 39.7456, -97.0892).await?;
    println!("Forecast office: {:?}", point.properties.forecast_office);

    // Get active weather alerts using struct parameters
    let alert_params = alerts::ActiveAlertsParams::default();
    let alerts = alerts::get_active_alerts(&config, alert_params).await?;
    println!("Active alerts: {}", alerts.features.len());

    Ok(())
}
```

### Weather Alerts

```rust,ignore
use noaa_weather_client::apis::{configuration::Configuration, alerts};
use noaa_weather_client::models::{AreaCode, AlertSeverity};

let config = Configuration::default();

// Get all active alerts
let alert_params = alerts::ActiveAlertsParams::default();
let active_alerts = alerts::get_active_alerts(&config, alert_params).await?;

// Get alerts for a specific area (state/territory) with filters
let ca_alerts = alerts::get_active_alerts_for_area(&config, &AreaCode::CA).await?;

// Get alerts for a specific zone
let zone_alerts = alerts::get_active_alerts_for_zone(&config, "CAZ006").await?;

// Get severe weather alerts only
let severe_params = alerts::ActiveAlertsParams {
    severity: Some(vec![AlertSeverity::Severe, AlertSeverity::Extreme]),
    ..Default::default()
};
let severe_alerts = alerts::get_active_alerts(&config, severe_params).await?;
```

### Weather Observations

```rust,ignore
use noaa_weather_client::apis::{configuration::Configuration, stations};

let config = Configuration::default();

// Get latest observation for a station
let observation = stations::get_latest_observations(&config, "KJFK", None).await?;
println!("Temperature: {:?}", observation.properties.temperature);

// Get station metadata
let station = stations::get_observation_station(&config, "KJFK", None).await?;
println!("Station name: {:?}", station.properties.name);
```

### Forecasts

```rust,ignore
use noaa_weather_client::apis::{configuration::Configuration, gridpoints};
use noaa_weather_client::models::NwsForecastOfficeId;

let config = Configuration::default();

// Get forecast for specific gridpoint
let forecast = gridpoints::get_gridpoint_forecast(
    &config,
    NwsForecastOfficeId::TOP,
    31,
    80,
    None, // feature_flags
    None  // units
).await?;

// Get hourly forecast
let hourly = gridpoints::get_gridpoint_forecast_hourly(
    &config,
    NwsForecastOfficeId::TOP,
    31,
    80,
    None, // feature_flags
    None  // units
).await?;
```

## Configuration

The `Configuration` struct provides default settings that work out of the box:

```rust,ignore
use noaa_weather_client::apis::configuration::Configuration;

// Use default configuration
let config = Configuration::default();

// Or customize the client
let config = Configuration {
    base_path: "https://api.weather.gov".to_owned(),
    user_agent: Some("my-app/1.0".to_owned()),
    api_key: None,
    client: reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?,
};
```

## Error Handling

All API functions return `Result<T, Error<E>>` where `E` is the specific error type for that endpoint:

```rust,ignore
use noaa_weather_client::apis::{configuration::Configuration, points};

let config = Configuration::default();

match points::get_point(&config, 0.0, 0.0).await {
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

| Module       | Description                 | Key Functions                                                                        |
| ------------ | --------------------------- | ------------------------------------------------------------------------------------ |
| `alerts`     | Weather alerts and warnings | `get_active_alerts`, `get_alert`                                                     |
| `points`     | Point metadata and stations | `get_point`, `get_point_stations`                                                    |
| `gridpoints` | Detailed forecasts          | `get_gridpoint_forecast`, `get_gridpoint_forecast_hourly`                            |
| `stations`   | Weather stations            | `get_observation_station`, `get_latest_observations`                                 |
| `zones`      | Forecast zones              | `get_zone`, `get_zone_forecast`                                                      |
| `offices`    | NWS offices                 | `get_office`, `get_office_headlines`                                                 |
| `radar`      | Radar data                  | `get_radar_stations`, `get_radar_servers`                                            |
| `aviation`   | Aviation weather            | `get_sigmets`, `get_center_weather_advisories`                                       |
| `products`   | Text products               | `get_product_types`, `get_products_query`, `get_latest_product_by_type_and_location` |
| `radio`\*    | Weather Radio broadcasts    | `get_point_radio`, `get_area_radio`                                                  |

\* Requires the `radio` feature flag.

## Authentication

The NOAA Weather API does not require authentication, but NOAA recommends a unique User-Agent to identify your application. An optional API key can be provided via the `api_key` field on `Configuration`, which is sent as an `X-Api-Key` header.

```rust,ignore
let config = Configuration {
    api_key: Some("your-api-key".to_owned()),
    ..Default::default()
};
```

From the [NOAA Weather API Documentation](https://www.weather.gov/documentation/services-web-api):

> A User Agent is required to identify your application.
> This string can be anything, and the more unique to your application the less likely it will be affected by a security event.
> If you include contact information (website or email), we can contact you if your string is associated to a security event.
> This will be replaced with an API key in the future.
>
> User-Agent: (myweatherapp.com, contact@myweatherapp.com)

## License

This project is licensed under the MIT License - see the [LICENSE.md](../LICENSE.md) file for details.

## Disclaimer

This project uses data published by NOAA/NWS but is otherwise unaffiliated with the National Weather Service and is not an official NWS library.

## Resources

- [NOAA Weather API Documentation](https://www.weather.gov/documentation/services-web-api)
- [NOAA Weather API GitHub](https://github.com/weather-gov/api)
- [National Weather Service](https://www.weather.gov/)
