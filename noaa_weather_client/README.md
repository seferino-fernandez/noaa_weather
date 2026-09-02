# NOAA Weather Client Library

An asynchronous, typed Rust client for version 3.11.0 of the [NOAA Weather API](https://www.weather.gov/documentation/services-web-api). One `Client` exposes eleven endpoint handles (`client.alerts()`, `client.points()`, ...) covering all 64 NOAA operations (65 methods including the composed `points().forecast_for`): forecasts, alerts, observations, offices, radar, aviation, text products, zones, glossary terms, and NOAA Weather Radio.

This project uses NOAA/NWS data but is not an official NOAA/NWS library.

GeoJSON endpoints return `Feature<T>` for one resource and
`FeatureCollection<T>` for lists. These shared envelopes retain feature ids,
geometry, collection metadata, and valid pagination links while each
endpoint-specific model remains in `properties`.

## Install

```bash
cargo add noaa_weather_client
```

Every endpoint, including NOAA Weather Radio and Terminal Aerodrome Forecasts, is always compiled in; there are no endpoint features. The one optional feature is `schemars`, which derives `JsonSchema` for every query struct and typed value so the request surface can be published as JSON Schema (for example to an MCP server):

```bash
cargo add noaa_weather_client --features schemars
```

## Quick start

```rust,no_run
use noaa_weather_client::{Client, Coordinates, apis::alerts::ActiveAlertsQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder("my-weather-app/2.0 (weather@example.com)").build()?;

    // One call resolves the point to its grid cell and fetches the forecast.
    let forecast = client
        .points()
        .forecast_for(Coordinates::new(39.7456, -97.0892)?)
        .await?;
    for period in forecast.properties.periods.iter().flatten().take(3) {
        println!("{:?}: {:?}", period.name, period.short_forecast);
    }

    let active = client
        .alerts()
        .active(&ActiveAlertsQuery {
            area: vec!["KS".parse()?],
            ..Default::default()
        })
        .await?;
    println!("Active alerts in Kansas: {}", active.features.len());

    Ok(())
}
```

## How requests are shaped

- **Handles.** `Client::alerts()`, `points()`, `gridpoints()`, `stations()`, `zones()`, `offices()`, `products()`, `aviation()`, `radar()`, `radio()`, and `glossary()` return borrowed `Copy` handles. Each method is one NOAA operation and documents the path it calls.
- **Typed path values.** Required path segments are typed: `StationId`, `OfficeId`, `ZoneId`, `GridpointId` (`OFFICE/x,y`), `CwsuId`, `AtsuId`, `CallSign`, `ProductId`, `ProductTypeCode`, `RadarStationId`, `AlertId`, and `Coordinates`. They validate on `parse()` and report an `InvalidValue` before any request is made. Server-issued ids (headline, briefing, image, radar server, profiler) stay `&str`.
- **Query structs.** Every operation with optional parameters takes one `*Query` struct with plain `pub` fields. Build it with struct-update syntax so unset filters stay absent:

  ```rust,no_run
  use noaa_weather_client::{Client, StationId, apis::stations::ObservationsQuery};

  # async fn example() -> Result<(), Box<dyn std::error::Error>> {
  let client = Client::builder("my-weather-app/2.0 (weather@example.com)").build()?;
  let station: StationId = "KPHX".parse()?;
  let recent = client
      .stations()
      .observations(&station, &ObservationsQuery {
          start: Some("2026-08-30T00:00:00Z".parse()?),
          limit: Some(12),
          ..Default::default()
      })
      .await?;
  # let _ = recent;
  # Ok(())
  # }
  ```

  List fields are `Vec<T>` (sent as one comma-separated value), instants are `Option<jiff::Timestamp>` (sent as RFC 3339), periods are `Option<Interval>` (ISO 8601 intervals), limits are `Option<u16>`. Query structs also derive `Serialize`/`Deserialize` in camelCase for JSON tooling; the wire encoding is separate and never produced from serde.
- **Date and time segments.** `stations().taf(&station, issued)` and `aviation().sigmet(&atsu, issued)` take one `jiff::Timestamp` and send its UTC date and `HHMM` minute (seconds are dropped). `stations().observation_at` sends RFC 3339 UTC. `aviation().cwa` and `sigmets_for_atsu_on` take a `jiff::civil::Date`.
- **Composition.** `points().forecast_for(coordinates)` is the one composed convenience: it calls `points().get`, converts the response to a `GridpointId`, and calls `gridpoints().forecast`. A point without grid coordinates is `Error::Invalid`.

## Pagination

Paged responses expose NOAA's opaque next-page token through
`FeatureCollection::next_cursor()`. Copy that cursor into the query to walk
pages yourself:

```rust,no_run
use noaa_weather_client::{Client, apis::alerts::AlertsQuery};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::builder("my-weather-app/2.0 (weather@example.com)").build()?;
let mut query = AlertsQuery {
    limit: Some(100),
    ..Default::default()
};

loop {
    let page = client.alerts().search(&query).await?;
    for alert in &page {
        println!("{}", alert.event.as_deref().unwrap_or("unknown alert"));
    }
    let Some(cursor) = page.next_cursor() else {
        break;
    };
    query.cursor = Some(cursor);
}
# Ok(())
# }
```

For a bounded automatic walk, `Alerts::list_all`, `Stations::list_all`, and
`Stations::observations_all` concatenate pages in NOAA's order. The nonzero
argument is the maximum number of pages to fetch:

```rust,no_run
use std::num::NonZeroU16;

use noaa_weather_client::{Client, apis::alerts::AlertsQuery};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::builder("my-weather-app/2.0 (weather@example.com)").build()?;
let alerts = client
    .alerts()
    .list_all(&AlertsQuery::default(), NonZeroU16::new(4).unwrap())
    .await?;
println!("{} alerts", alerts.len());
# Ok(())
# }
```

Three NOAA operations publish broken `pagination.next` links and cannot be
walked this way:

- Gridpoint stations link to `/stations?id[]=…&cursor=…`, which returns an
  empty page rather than the next gridpoint page. Its query deliberately has
  no cursor; raise `limit` instead.
- Zone stations publish the same incorrect `/stations?id[]=…&cursor=…` link.
  Do not feed it back into `ZoneStationsQuery::cursor`; raise `limit` instead.
- Zone observations link to one station's
  `/stations/{id}/observations` rather than the zone. Their query deliberately
  has no cursor; use `Stations::observations_all` per station for history.

## NOAA path → handle method

| NOAA path | Handle method |
| --- | --- |
| `GET /alerts` | `alerts().search(&AlertsQuery)` |
| `GET /alerts/active` | `alerts().active(&ActiveAlertsQuery)` |
| `GET /alerts/active/count` | `alerts().active_count()` |
| `GET /alerts/active/zone/{zoneId}` | `alerts().active_for_zone(&ZoneId)` |
| `GET /alerts/active/area/{area}` | `alerts().active_for_area(&AreaCode)` |
| `GET /alerts/active/region/{region}` | `alerts().active_for_marine_region(MarineRegionCode)` |
| `GET /alerts/types` | `alerts().types()` |
| `GET /alerts/{id}` | `alerts().get(&AlertId)` |
| `GET /aviation/cwsus/{cwsuId}` | `aviation().cwsu(&CwsuId)` |
| `GET /aviation/cwsus/{cwsuId}/cwas` | `aviation().cwas(&CwsuId)` |
| `GET /aviation/cwsus/{cwsuId}/cwas/{date}/{sequence}` | `aviation().cwa(&CwsuId, Date, u32)` |
| `GET /aviation/sigmets` | `aviation().sigmets(&SigmetsQuery)` |
| `GET /aviation/sigmets/{atsu}` | `aviation().sigmets_for_atsu(&AtsuId)` |
| `GET /aviation/sigmets/{atsu}/{date}` | `aviation().sigmets_for_atsu_on(&AtsuId, Date)` |
| `GET /aviation/sigmets/{atsu}/{date}/{time}` | `aviation().sigmet(&AtsuId, Timestamp)` |
| `GET /glossary` | `glossary().terms()` |
| `GET /gridpoints/{wfo}/{x},{y}` | `gridpoints().get(&GridpointId)` |
| `GET /gridpoints/{wfo}/{x},{y}/forecast` | `gridpoints().forecast(&GridpointId, &ForecastQuery)` |
| `GET /gridpoints/{wfo}/{x},{y}/forecast/hourly` | `gridpoints().forecast_hourly(&GridpointId, &ForecastQuery)` |
| `GET /gridpoints/{wfo}/{x},{y}/stations` | `gridpoints().stations(&GridpointId, &GridpointStationsQuery)` |
| `GET /offices/{officeId}` | `offices().get(&OfficeId)` |
| `GET /offices/{officeId}/headlines` | `offices().headlines(&OfficeId)` |
| `GET /offices/{officeId}/headlines/{headlineId}` | `offices().headline(&OfficeId, &str)` |
| `GET /offices/{officeId}/briefing` | `offices().briefing(&OfficeId)` |
| `GET /offices/{officeId}/briefing/download/latest` | `offices().latest_briefing_document(&OfficeId)` |
| `GET /offices/{officeId}/briefing/download/{briefingId}` | `offices().briefing_document(&OfficeId, &str)` |
| `GET /offices/{officeId}/weatherstories` | `offices().weather_stories(&OfficeId)` |
| `GET /offices/{officeId}/weatherstories/download/{imageId}` | `offices().weather_story_image(&OfficeId, &str)` |
| `GET /points/{point}` | `points().get(Coordinates)` |
| `GET /points/{point}` + `/gridpoints/.../forecast` | `points().forecast_for(Coordinates)` |
| `GET /points/{point}/radio` | `radio().for_point(Coordinates)` |
| `GET /products` | `products().search(&ProductsQuery)` |
| `GET /products/locations` | `products().locations()` |
| `GET /products/types` | `products().types()` |
| `GET /products/{productId}` | `products().get(&ProductId)` |
| `GET /products/types/{typeId}` | `products().by_type(&ProductTypeCode)` |
| `GET /products/types/{typeId}/locations` | `products().locations_for_type(&ProductTypeCode)` |
| `GET /products/locations/{locationId}/types` | `products().types_for_location(&OfficeId)` |
| `GET /products/types/{typeId}/locations/{locationId}` | `products().by_type_and_location(&ProductTypeCode, &OfficeId)` |
| `GET /products/types/{typeId}/locations/{locationId}/latest` | `products().latest(&ProductTypeCode, &OfficeId)` |
| `GET /radar/servers` | `radar().servers(&RadarServersQuery)` |
| `GET /radar/servers/{id}` | `radar().server(&str, &RadarServerQuery)` |
| `GET /radar/stations` | `radar().stations(&RadarStationsQuery)` |
| `GET /radar/stations/{stationId}` | `radar().station(&RadarStationId, &RadarStationQuery)` |
| `GET /radar/stations/{stationId}/alarms` | `radar().station_alarms(&RadarStationId)` |
| `GET /radar/queues/{host}` | `radar().queue(&RadarQueueHost, &RadarQueueQuery)` |
| `GET /radar/profilers/{stationId}` | `radar().wind_profiler(&str, &WindProfilerQuery)` |
| `GET /radar/spgds` | `radar().spgds(&SpgdsQuery)` |
| `GET /radio` | `radio().transmitters(&TransmittersQuery)` |
| `GET /radio/{callSign}` | `radio().transmitter(&CallSign)` |
| `GET /radio/{callSign}/broadcast` | `radio().broadcast(&CallSign)` |
| `GET /zones/county/{zoneId}/radio` | `radio().transmitters_for_county(&ZoneId)` |
| `GET /stations` | `stations().list(&StationsQuery)` |
| `GET /stations/{stationId}` | `stations().get(&StationId)` |
| `GET /stations/{stationId}/observations` | `stations().observations(&StationId, &ObservationsQuery)` |
| `GET /stations/{stationId}/observations/latest` | `stations().latest_observation(&StationId, &LatestObservationQuery)` |
| `GET /stations/{stationId}/observations/{time}` | `stations().observation_at(&StationId, Timestamp)` |
| `GET /stations/{stationId}/tafs` | `stations().tafs(&StationId)` |
| `GET /stations/{stationId}/tafs/{date}/{time}` | `stations().taf(&StationId, Timestamp)` |
| `GET /zones` | `zones().list(&ZonesQuery)` |
| `GET /zones/{type}` | `zones().list_of_type(ZoneType, &ZonesQuery)` |
| `GET /zones/{type}/{zoneId}` | `zones().get(ZoneType, &ZoneId, &ZoneQuery)` |
| `GET /zones/{type}/{zoneId}/forecast` | `zones().forecast(ZoneType, &ZoneId)` |
| `GET /zones/forecast/{zoneId}/observations` | `zones().observations(&ZoneId, &ZoneObservationsQuery)` |
| `GET /zones/forecast/{zoneId}/stations` | `zones().stations(&ZoneId, &ZoneStationsQuery)` |

The deprecated `/points/{point}/stations` operation is intentionally not exposed. Use `points().get` to obtain the gridpoint, then `gridpoints().stations`, or query the station endpoints directly.

## Semantic TAF forecasts

`stations().taf` decodes NOAA's IWXXM XML privately and returns forecast meaning rather than the XML element tree. The model provides typed timestamps, ordered base/change groups, canonical meters/knots/feet/Celsius values, exact weather codes plus parsed phenomena, cloud types, temperatures, and explicit forecast, cancellation, missing, CAVOK, unchanged, and unavailable states. Serializing the result produces semantic JSON without namespace or XML-wrapper fields.

```rust,no_run
use noaa_weather_client::{Client, StationId};
use noaa_weather_client::models::terminal_aerodrome_forecast::{
    ForecastReport, ForecastWeather,
};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::builder("my-weather-app/2.0 (weather@example.com)").build()?;
let station: StationId = "KPHX".parse()?;
let taf = client
    .stations()
    .taf(&station, "2026-08-30T22:54:00Z".parse()?)
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

## Forecast values in 3.11

Textual forecasts always request NOAA's quantitative temperature and wind formats. Callers never pass feature flags, and `temperature`, `wind_speed`, and `wind_gust` use `QuantitativeValue` models.

```rust,no_run
use noaa_weather_client::{Client, GridpointId, apis::gridpoints::{ForecastQuery, ForecastUnits}};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::builder("my-weather-app/2.0 (weather@example.com)").build()?;
let grid: GridpointId = "TOP/31,80".parse()?;
let forecast = client
    .gridpoints()
    .forecast(&grid, &ForecastQuery { units: Some(ForecastUnits::Us) })
    .await?;
# let _ = forecast;
# Ok(())
# }
```

## Binary office media

Briefing documents and weather-story images return `BinaryPayload`. It retains reference-counted `Bytes`, the validated media type, and the final URL after redirects without decoding the body as text.

```rust,no_run
use noaa_weather_client::{Client, OfficeId};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::builder("my-weather-app/2.0 (weather@example.com)").build()?;
let office: OfficeId = "PSR".parse()?;
let pdf = client.offices().latest_briefing_document(&office).await?;
tokio::fs::write("briefing.pdf", pdf.as_bytes()).await?;
# Ok(())
# }
```

The client follows up to five redirects itself, refuses `https` to `http` downgrades, and drops the API key when a hop leaves the configured origin. A redirect that cannot be followed is `Error::Protocol` with `ProtocolError::Redirect`.

`OfficeId` accepts forecast offices, regional headquarters (`ARH`, `CRH`, `ERH`, `PRH`, `SRH`, `WRH`), and national headquarters (`NWS`); `OfficeId::KNOWN` lists the forecast offices for completion hints without restricting the value.

## Error handling

Every handle method returns the same compact, non-generic `Error` type. HTTP response and protocol details are boxed so the common result type stays small. `ResponseContent` retains status, URL, raw `Bytes`, content type, a parsed NOAA `ProblemDetail` when available, the `Retry-After` delay, the `X-Correlation-Id` and `X-Request-Id` headers, and how many attempts were made. `Error::Invalid` carries the `InvalidValue` from a typed value that failed validation.

Helpers on `Error` answer the common questions without matching variants: `status()`, `is_not_found()`, `is_rate_limited()`, `retry_after()`, `problem()`, `is_retryable()`, and `attempts()`.

```rust,no_run
use noaa_weather_client::{Client, Coordinates, Error};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::builder("my-weather-app/2.0 (weather@example.com)").build()?;
match client.points().get(Coordinates::new(0.0, 0.0)?).await {
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
