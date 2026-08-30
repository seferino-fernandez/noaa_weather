# NOAA Weather Client Library

An asynchronous, typed Rust client for version 3.11.0 of the [NOAA Weather API](https://www.weather.gov/documentation/services-web-api). The default feature set exposes 64 public `get_*` endpoint functions for forecasts, alerts, observations, offices, radar, aviation, text products, zones, glossary terms, and NOAA Weather Radio.

This project uses NOAA/NWS data but is not an official NOAA/NWS library.

## Install

```bash
cargo add noaa_weather_client
```

The 1.3 client had an empty default feature set. This release changes the client default to `radio`, which also enables XML support for broadcast transcripts and Terminal Aerodrome Forecasts (TAFs). For a smaller JSON-only build:

```bash
cargo add noaa_weather_client --no-default-features
```

The complete feature matrix is:

- Default: `radio` enables transmitter metadata, broadcasts, both TAF APIs, and `xml`/`quick-xml`.
- `default-features = false`: omits radio, both TAF APIs, and `quick-xml`.
- `default-features = false, features = ["xml"]`: retains both TAF APIs and `quick-xml` without radio.

For TAF support without radio:

```bash
cargo add noaa_weather_client --no-default-features --features xml
```

## Quick start

```rust,no_run
use noaa_weather_client::apis::{alerts, configuration::Configuration, points};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Configuration::default();

    let point = points::get_point(&config, 39.7456, -97.0892).await?;
    println!("Forecast office: {:?}", point.properties.forecast_office);

    let active = alerts::get_active_alerts(
        &config,
        alerts::ActiveAlertsParams::default(),
    )
    .await?;
    println!("Active alerts: {}", active.features.len());

    Ok(())
}
```

## Forecast values in 3.11

Textual forecasts always request NOAA's quantitative temperature and wind formats. Callers no longer pass feature flags, and `temperature`, `wind_speed`, and `wind_gust` use `QuantitativeValue` models.

```rust,no_run
use noaa_weather_client::apis::{configuration::Configuration, gridpoints};
use noaa_weather_client::models::{GridpointForecastUnits, NwsForecastOfficeId};

# async fn example() -> Result<(), noaa_weather_client::apis::Error> {
let config = Configuration::default();
let forecast = gridpoints::get_gridpoint_forecast(
    &config,
    NwsForecastOfficeId::Top,
    31,
    80,
    Some(GridpointForecastUnits::Us),
)
.await?;
# let _ = forecast;
# Ok(())
# }
```

## New 3.11 data families

```rust,no_run
use noaa_weather_client::apis::{configuration::Configuration, glossary, offices, radar};
use noaa_weather_client::models::{NwsForecastOfficeId, NwsOfficeId};

# async fn example() -> Result<(), noaa_weather_client::apis::Error> {
let config = Configuration::default();

let terms = glossary::get_glossary(&config).await?;
let briefing = offices::get_forecast_office_briefing(
    &config,
    &NwsOfficeId::NwsForecastOfficeId(NwsForecastOfficeId::Psr),
)
.await?;
let spgds = radar::get_radar_spgds(&config, None).await?;
# let _ = (terms, briefing, spgds);
# Ok(())
# }
```

With the default `radio` feature, transmitter metadata is also typed:

```rust,no_run
# #[cfg(feature = "radio")]
use noaa_weather_client::apis::{configuration::Configuration, radio};

# #[cfg(feature = "radio")]
# async fn example() -> Result<(), noaa_weather_client::apis::Error> {
let config = Configuration::default();
let transmitters = radio::get_radio_transmitters(&config, None).await?;
let station = radio::get_radio_transmitter(&config, "KEC94").await?;
# let _ = (transmitters, station);
# Ok(())
# }
```

## Binary office media

Briefing documents and weather-story images return `BinaryPayload`. It retains reference-counted `Bytes`, the validated media type, and the final URL after redirects without decoding the body as text.

```rust,no_run
use noaa_weather_client::apis::{configuration::Configuration, offices};
use noaa_weather_client::models::{NwsForecastOfficeId, NwsOfficeId};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let config = Configuration::default();
let pdf = offices::get_latest_forecast_office_briefing_document(
    &config,
    &NwsOfficeId::NwsForecastOfficeId(NwsForecastOfficeId::Psr),
)
.await?;
tokio::fs::write("briefing.pdf", pdf.as_bytes()).await?;
# Ok(())
# }
```

The configured `reqwest::Client` controls redirect behavior. A client that does not follow the latest-document redirect receives `Error::Response` for the 302 response.

## Error handling

Every endpoint returns the same compact, non-generic `Error` type. HTTP response and protocol details are boxed so the common result type stays small. `ResponseContent` retains status, URL, raw `Bytes`, content type, and a parsed NOAA `ProblemDetail` when available.

```rust,no_run
use noaa_weather_client::apis::{Error, configuration::Configuration, points};

# async fn example() {
let config = Configuration::default();
match points::get_point(&config, 0.0, 0.0).await {
    Ok(point) => println!("{point:?}"),
    Err(Error::Response(response)) => {
        eprintln!("HTTP {} from {}", response.status(), response.url());
        if let Some(problem) = response.problem_detail() {
            eprintln!("{}: {}", problem.title, problem.detail);
        }
    }
    Err(error) => eprintln!("{error}"),
}
# }
```

## API coverage

| Module | Selected functions |
| --- | --- |
| `alerts` | `get_active_alerts`, `get_alerts`, `get_alert` |
| `aviation` | `get_sigmets`, `get_center_weather_advisories` |
| `glossary` | `get_glossary` |
| `gridpoints` | `get_gridpoint_forecast`, `get_gridpoint_forecast_hourly` |
| `offices` | `get_forecast_office_briefing`, `get_forecast_office_weather_stories` |
| `points` | `get_point` |
| `products` | `get_products_query`, `get_latest_product_by_type_and_location` |
| `radar` | `get_radar_stations`, `get_radar_data_queue`, `get_radar_spgds` |
| `radio`* | `get_radio_transmitters`, `get_radio_transmitter`, `get_area_radio` |
| `stations` | `get_observation_station`, `get_latest_observations` |
| `zones` | `get_zone`, `get_zone_forecast`, `get_stations_by_zone` |

\* Requires the `radio` feature.

The deprecated `/points/{latitude},{longitude}/stations` operation is intentionally not exposed. Use point metadata to obtain the gridpoint, then call `gridpoints::get_gridpoint_stations`, or query the station endpoints directly.

All eight `/offices/{officeId}` functions accept `NwsOfficeId`, covering forecast offices, regional headquarters (`ARH`, `CRH`, `ERH`, `PRH`, `SRH`, `WRH`), and national headquarters (`NWS`).

## Configuration and authentication

`Configuration::default()` uses `https://api.weather.gov`. Set a distinctive User-Agent with contact information for production applications:

```rust,no_run
use noaa_weather_client::apis::configuration::Configuration;

let config = Configuration {
    user_agent: Some("my-weather-app/2.0 (weather@example.com)".to_owned()),
    ..Default::default()
};
```

The API is free and does not normally require a key. `Configuration::api_key`, when set, currently sends `X-Api-Key`. NOAA's published 3.11 specification names the security-scheme header `API-Key`, while its description and one response header use `X-Api-Key`; do not rely on this experimental mechanism without validating it against NOAA.

## Resources

- [NOAA Weather API documentation](https://www.weather.gov/documentation/services-web-api)
- [NOAA Weather API source](https://github.com/weather-gov/api)
- [National Weather Service](https://www.weather.gov/)

Licensed under the [MIT License](../LICENSE.md).
