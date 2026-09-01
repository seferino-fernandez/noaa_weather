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

## Semantic TAF forecasts

`stations::get_terminal_aerodrome_forecast` decodes NOAA's IWXXM XML privately and returns forecast meaning rather than the XML element tree. The model provides typed timestamps, ordered base/change groups, canonical meters/knots/feet/Celsius values, exact weather codes plus parsed phenomena, cloud types, temperatures, and explicit forecast, cancellation, missing, CAVOK, unchanged, and unavailable states. Serializing the result produces semantic JSON without namespace or XML-wrapper fields.

```rust,no_run
use noaa_weather_client::{Client, apis::stations};
use noaa_weather_client::models::terminal_aerodrome_forecast::{
    ForecastReport, ForecastWeather,
};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::builder("my-weather-app/2.0 (weather@example.com)").build()?;
let taf = stations::get_terminal_aerodrome_forecast(
    &client,
    "KPHX",
    "2026-08-30",
    "2254",
)
.await?;

if let ForecastReport::Forecast { valid_period, .. } = taf.report() {
    println!("valid from {} to {}", valid_period.start(), valid_period.end());
}
if let Some(base) = taf.base_forecast() {
    if let ForecastWeather::Phenomena { items } = base.conditions().weather() {
        for weather in items {
            println!("{}: {:?}", weather.code(), weather.phenomena());
        }
    }
}
# Ok(())
# }
```

The IWXXM wire structs and decoder are implementation details. Consumers should use the accessors and non-exhaustive semantic enums under `models::terminal_aerodrome_forecast`.

## Quick start

```rust,no_run
use noaa_weather_client::{Client, apis::{alerts, points}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder("my-weather-app/2.0 (weather@example.com)").build()?;

    let point = points::get_point(&client, 39.7456, -97.0892).await?;
    println!("Forecast office: {:?}", point.properties.forecast_office);

    let active = alerts::get_active_alerts(
        &client,
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
use noaa_weather_client::{Client, apis::gridpoints};
use noaa_weather_client::models::{GridpointForecastUnits, NwsForecastOfficeId};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::builder("my-weather-app/2.0 (weather@example.com)").build()?;
let forecast = gridpoints::get_gridpoint_forecast(
    &client,
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
use noaa_weather_client::{Client, apis::{glossary, offices, radar}};
use noaa_weather_client::models::{NwsForecastOfficeId, NwsOfficeId};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::builder("my-weather-app/2.0 (weather@example.com)").build()?;

let terms = glossary::get_glossary(&client).await?;
let briefing = offices::get_forecast_office_briefing(
    &client,
    &NwsOfficeId::NwsForecastOfficeId(NwsForecastOfficeId::Psr),
)
.await?;
let spgds = radar::get_radar_spgds(&client, None).await?;
# let _ = (terms, briefing, spgds);
# Ok(())
# }
```

With the default `radio` feature, transmitter metadata is also typed:

```rust,no_run
# #[cfg(feature = "radio")]
use noaa_weather_client::{Client, apis::radio};

# #[cfg(feature = "radio")]
# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::builder("my-weather-app/2.0 (weather@example.com)").build()?;
let transmitters = radio::get_radio_transmitters(&client, None).await?;
let station = radio::get_radio_transmitter(&client, "KEC94").await?;
# let _ = (transmitters, station);
# Ok(())
# }
```

## Binary office media

Briefing documents and weather-story images return `BinaryPayload`. It retains reference-counted `Bytes`, the validated media type, and the final URL after redirects without decoding the body as text.

```rust,no_run
use noaa_weather_client::{Client, apis::offices};
use noaa_weather_client::models::{NwsForecastOfficeId, NwsOfficeId};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::builder("my-weather-app/2.0 (weather@example.com)").build()?;
let pdf = offices::get_latest_forecast_office_briefing_document(
    &client,
    &NwsOfficeId::NwsForecastOfficeId(NwsForecastOfficeId::Psr),
)
.await?;
tokio::fs::write("briefing.pdf", pdf.as_bytes()).await?;
# Ok(())
# }
```

The client follows up to five redirects itself, refuses `https` to `http` downgrades, and drops the API key when a hop leaves the configured origin. A redirect that cannot be followed is `Error::Protocol` with `ProtocolError::Redirect`.

## Error handling

Every endpoint returns the same compact, non-generic `Error` type. HTTP response and protocol details are boxed so the common result type stays small. `ResponseContent` retains status, URL, raw `Bytes`, content type, a parsed NOAA `ProblemDetail` when available, the `Retry-After` delay, the `X-Correlation-Id` and `X-Request-Id` headers, and how many attempts were made.

Helpers on `Error` answer the common questions without matching variants: `status()`, `is_not_found()`, `is_rate_limited()`, `retry_after()`, `problem()`, `is_retryable()`, and `attempts()`.

```rust,no_run
use noaa_weather_client::{Client, Error, apis::points};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::builder("my-weather-app/2.0 (weather@example.com)").build()?;
match points::get_point(&client, 0.0, 0.0).await {
    Ok(point) => println!("{point:?}"),
    Err(error) if error.is_not_found() => eprintln!("no forecast for that point"),
    Err(Error::Response(response)) => {
        eprintln!(
            "HTTP {} from {} after {} attempts",
            response.status(),
            response.url(),
            response.attempts()
        );
        if let Some(problem) = response.problem_detail() {
            eprintln!("{}: {}", problem.title, problem.detail);
        }
    }
    Err(error) => eprintln!("{error}"),
}
# Ok(())
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

## Client configuration and authentication

`Client::builder(user_agent)` is the only way to obtain a `Client`. NOAA asks every caller for a distinctive `User-Agent`, ideally with contact information, so the builder requires one. Every other setting has a production default:

| Setting | Default |
| --- | --- |
| `base_url` | `https://api.weather.gov` |
| `timeout` (per attempt) | 30 s |
| `connect_timeout` | 10 s |
| `retry` | 3 attempts, 500 ms base delay, 20 s cap |
| `max_response_bytes` | 32 MiB |

```rust,no_run
use std::time::Duration;

use noaa_weather_client::{Client, RetryPolicy};

# fn example() -> Result<(), noaa_weather_client::BuildError> {
let client = Client::builder("my-weather-app/2.0 (weather@example.com)")
    .timeout(Duration::from_secs(15))
    .retry(RetryPolicy::default().max_attempts(5))
    .max_response_bytes(8 * 1024 * 1024)
    .build()?;
# let _ = client;
# Ok(())
# }
```

`Client` is `Clone`, `Send`, and `Sync`; clones share one connection pool. `build()` validates the user agent and base URL and returns `BuildError` instead of failing at request time.

Retries cover HTTP 429, 500, 502, 503, and 504 responses and connection, timeout, and body-read failures. Delays grow exponentially with jitter, and a server `Retry-After` header is honored when it fits under the policy's `max_delay`; a longer `Retry-After` stops the loop and is reported through `Error::retry_after()`. Use `RetryPolicy::none()` to disable retries.

Responses are decompressed transparently and buffered up to `max_response_bytes`. A declared `Content-Length` above the cap is refused before any of the body is read; a streamed body is read only until it passes the cap. Either case fails with `ProtocolError::ResponseTooLarge`.

For proxies, custom TLS roots, or pool tuning, pass a pre-configured `reqwest::ClientBuilder` through `ClientBuilder::reqwest_builder`. The client's own timeout, connect timeout, `User-Agent`, gzip, and redirect settings are always applied on top.

The API is free and does not normally require a key. `ClientBuilder::api_key`, when set, sends `X-Api-Key` to the configured origin only and never prints the key in `Debug` output. NOAA's published 3.11 specification names the security-scheme header `API-Key`, while its description and one response header use `X-Api-Key`; do not rely on this experimental mechanism without validating it against NOAA.

## Resources

- [NOAA Weather API documentation](https://www.weather.gov/documentation/services-web-api)
- [NOAA Weather API source](https://github.com/weather-gov/api)
- [National Weather Service](https://www.weather.gov/)

Licensed under the [MIT License](../LICENSE.md).
