# NOAA Weather API 3.11.0 migration

This report records the evidence and caller-visible decisions behind the move from NOAA Weather API 3.6.0 to 3.11.0. Compatibility with the 3.6 client surface is intentionally not retained; this is a 2.0-level library migration. Package versions remain managed by release-plz.

## Evidence and method

The comparison uses the immutable specifications checked into this repository:

| NOAA API version | File | SHA-256 | OpenAPI dialect |
| --- | --- | --- | --- |
| 3.6.0 | `notes/noaa-v3.6.0-openapi.json` | `8ac7c6b0a3be65f69d4e50f644e88ea2f9045a44551ca75c0c7338eec79a5482` | 3.0.3 |
| 3.11.0 | `notes/noaa-v3.11.0-openapi.json` | `e6b81cb88b8ed4169d463da3194d4c8817548884f42df34d7441f0e0791338d8` | 3.1.2 |

Counts below were derived from normalized JSON objects. An operation is an HTTP method entry under a path. A changed shared path or schema means its normalized object differs; this intentionally includes OpenAPI 3.0-to-3.1 representation changes.

| Object | 3.6.0 | 3.11.0 | Added | Removed | Changed in place |
| --- | ---: | ---: | ---: | ---: | ---: |
| Paths | 60 | 69 | 9 | 0 | 5 shared paths |
| Operations | 60 | 69 | 9 | 0 | 5 operations on shared paths |
| Component schemas | 104 | 108 | 4 | 0 | 21 shared schemas |

The four added component schemas are `NWSConnectDocumentMetadata`, `OfficeBriefing`, `OfficeWeatherStory`, and `OfficeWeatherStoryCollection`. Much of the 21-schema in-place delta is mechanical OpenAPI 3.1 JSON Schema notation: `nullable: true` becomes a union with `null`, and singular `example` becomes `examples`. Caller-relevant changes are called out separately below.

## Confirmed specification changes

### Added operations

NOAA 3.11 adds nine `GET` operations and removes none:

| Path | Operation ID | Client function |
| --- | --- | --- |
| `/offices/{officeId}/briefing` | `office_briefing` | `get_forecast_office_briefing` |
| `/offices/{officeId}/briefing/download/latest` | `office_briefing_download_latest` | `get_latest_forecast_office_briefing_document` |
| `/offices/{officeId}/briefing/download/{briefingId}` | `office_briefing_download` | `get_forecast_office_briefing_document` |
| `/offices/{officeId}/weatherstories` | `office_weatherstory` | `get_forecast_office_weather_stories` |
| `/offices/{officeId}/weatherstories/download/{imageId}` | `office_weatherstory_image` | `get_forecast_office_weather_story_image` |
| `/radar/spgds` | `radar_spgds` | `get_radar_spgds` |
| `/radio` | `transmitters` | `get_radio_transmitters` |
| `/radio/{callSign}` | `transmitter` | `get_radio_transmitter` |
| `/zones/{type}/{zoneId}/radio` | `transmitter_zone` | `get_radio_transmitters_for_county_zone` |

The zone transmitter operation constrains `type` to `county`. The client therefore hard-codes that path segment instead of exposing a meaningless caller choice.

`GET /glossary` exists in both specifications but was missing from the old library. This release adds `get_glossary` too. The implementation therefore has ten newly exposed functions—one previously omitted operation plus the nine spec additions—and 64 public `get_*` endpoint functions with default features. This distinction resolves the otherwise ambiguous phrase “nine newly implemented operations.”

### Modified paths and queries

Five shared path objects changed:

- The `ObservationsStationFeatureFlags` header was removed from `/gridpoints/{wfo}/{x},{y}/stations`, `/stations`, `/stations/{stationId}`, and `/zones/forecast/{zoneId}/stations`. The component parameter itself was deleted.
- `/radar/queues/{host}` raises `limit.maximum` from 500 to 50,000 and constrains `host` to `rds` or `tds`.

No operation was removed from the 3.11 document. In particular, the deprecated point-stations operation and deprecated alerts `active` query remain present in both source specs; their removal from this library is an intentional design choice, not a claimed 3.11 spec deletion.

### Material schema changes

- `Alert.note` is new and nullable. The Rust model uses `Option<Option<String>>` so absent, explicit `null`, and a supplied string remain distinguishable.
- The four new office schemas describe briefing metadata, weather-story metadata, collection shape, and binary media links.
- Several existing nullable fields move to native OpenAPI 3.1 null unions, including alert times/text, forecast trends and gusts, quantitative values, geometry, observation fields, and zone/radar fields.
- `AstronomicalData` twilight, sunrise, sunset, and transit timestamps become explicitly nullable.
- `ObservationCollectionGeoJson.pagination` is represented as a property alongside `features` in 3.11.

The 3.11 spec still describes forecast `temperature`, `windSpeed`, and `windGust` as old scalar-or-quantitative unions. It also tells callers in prose to enable the quantitative feature flags. The library chooses the forward format exclusively, as described under design decisions.

## Deprecation inventory

Both specs contain exactly 13 machine-declared `deprecated: true` objects; 3.11 adds or removes none of these declarations.

Five are entire operations:

- `GET /icons`
- `GET /icons/{set}/{timeOfDay}/{first}`
- `GET /icons/{set}/{timeOfDay}/{first}/{second}`
- `GET /thumbnails/satellite/{area}`
- `GET /points/{latitude},{longitude}/stations`

One is the `active` query parameter on `GET /alerts`; NOAA directs consumers to the dedicated `/alerts/active` endpoints.

Seven are schema properties:

- `Gridpoint12hForecastPeriod.temperatureUnit`
- `Gridpoint12hForecastPeriod.icon`
- `GridpointHourlyForecastPeriod.temperatureUnit`
- `GridpointHourlyForecastPeriod.icon`
- `Observation.icon`
- `Zone.cwa`
- `Zone.forecastOffices`

Seven additional schema nodes use deprecation language only in `description`; they do **not** carry the OpenAPI `deprecated` keyword:

- 12-hour and hourly forecast `temperature`
- 12-hour and hourly forecast `windSpeed`
- 12-hour and hourly forecast `windGust`
- `UnitOfMeasure`, for namespaced unit strings

This machine-declared/prose-only split matters to generators and automated diff tools.

## Implemented Rust surface

### New operations and models

The client exports:

- `GlossaryResponse` and `GlossaryTerm`.
- `NwsConnectDocumentMetadata`, `OfficeBriefing`, `OfficeBriefingResponse`, `OfficeWeatherStory`, and `OfficeWeatherStoryCollection`.
- `RadarSpgdsResponse` plus entry, status, LDM, disk, uptime, throughput, and gateway telemetry models.
- With `radio`, `RadioTransmitter` and `RadioTransmitterCollection`.
- `BinaryPayload` for validated PDF and image bytes, media type, and final redirect URL.

### Breaking design decisions

These choices deliberately optimize the API for a compact, consistent, Rust-idiomatic 2.0 surface:

- Every endpoint now returns one non-generic `Error`. The old endpoint-specific error enums and `Error<E>` types are removed.
- `ResponseContent` and `ProtocolError` are boxed in `Error`; raw response bodies use reference-counted `Bytes`. This keeps the common `Result<T, Error>` type compact while retaining status, URL, media type, raw bytes, and typed NOAA problem details.
- Binary office endpoints return `BinaryPayload` rather than attempting UTF-8 conversion.
- All eight office operations accept the complete `NwsOfficeId` union: forecast offices, regional headquarters (`ARH`, `CRH`, `ERH`, `PRH`, `SRH`, `WRH`), and national headquarters (`NWS`). They are not narrowed to forecast-office IDs.
- `get_point_stations` and the CLI `points stations` command are removed. Resolve a point to its gridpoint and call `get_gridpoint_stations`, or use station queries.
- `GetAlertsParams.active` and the general alerts `--active` flag are removed. The dedicated active-alert functions and commands remain.
- Caller-supplied station `Feature-Flags` arguments are removed from the four affected operations.
- Forecast callers no longer pass feature flags. The client always sends `forecast_temperature_qv,forecast_wind_speed_qv` for 12-hour and hourly forecasts.
- Forecast `temperature`, `wind_speed`, and `wind_gust` now use `QuantitativeValue`; the three legacy scalar/quantitative union model types are removed.
- `Alert.note` preserves absent/null/value semantics.
- The seven machine-deprecated icon, unit, and zone fields are removed from Rust models.
- Radar queue limits are accepted only in NOAA's 3.11 range, 1 through 50,000; the CLI enforces this before a request.
- Optional features are split into `radio` and `xml`, and the client default changes from empty in 1.3 to `radio`. Normal defaults include radio, both Terminal Aerodrome Forecast APIs, and `xml`/`quick-xml`. A `--no-default-features` client or CLI build omits radio, both TAF APIs, and `quick-xml`; `--no-default-features --features xml` retains both TAF APIs without radio.

## Live compatibility observations encoded by tests

These are service-shape observations captured in tolerant model and endpoint tests. They are not additional guarantees made by the checked-in OpenAPI document:

- Office briefing metadata has appeared directly, wrapped under `briefing`, and as an explicit null briefing. Weather stories have appeared as a bare array or wrapped collection. Optional fields and unknown fields are accepted, and live story order may be zero despite the spec minimum of one.
- SPGDS has no response schema in the 3.11 document. Observed telemetry can be sparse, dynamically keyed, and scalar-flexible: a nominal field may arrive as a string, number, or boolean. The typed model normalizes those scalars to strings without inventing numeric/boolean semantics.
- Radio frequency is modeled as the string supplied by the service. Transmitter, SAME-code, and county arrays preserve service order and duplicates rather than sorting or deduplicating them.
- The latest office briefing operation returns a redirect. Tests cover a relative redirect followed to PDF by the default client and preservation of the 302 as `Error::Response` when a caller configures a no-follow redirect policy.

The executable CLI integration suite also exercises the free live NOAA API. Those tests assert stable parse/output contracts and discover office media identifiers at runtime instead of assuming volatile counts, ordering, timestamps, or media availability.

## API key header discrepancy

Both checked-in specs define the `apiKeyAuth` security scheme with header name `API-Key`. In the same object, the prose says `X-Api-Key`, and an alerts response header is also named `X-Api-Key`. The existing client implementation sends `X-Api-Key` when `Configuration::api_key` is set.

This is a confirmed spec/internal naming mismatch, not evidence that NOAA requires a key: the API remains free, and the spec describes the key system as experimental. Applications should use a distinctive User-Agent and should not depend on API-key behavior without validating it against NOAA.

## Upgrade guide

### Forecast feature flags and quantitative values

Before:

```rust,ignore
let forecast = gridpoints::get_gridpoint_forecast(
    &config,
    NwsForecastOfficeId::Top,
    31,
    80,
    Some(vec!["forecast_temperature_qv".into()]),
    Some(GridpointForecastUnits::Us),
).await?;
```

After:

```rust,ignore
let forecast = gridpoints::get_gridpoint_forecast(
    &config,
    NwsForecastOfficeId::Top,
    31,
    80,
    Some(GridpointForecastUnits::Us),
).await?;

let temperature: Option<&QuantitativeValue> = forecast
    .properties
    .periods
    .first()
    .and_then(|period| period.temperature.as_deref());
```

### Shared errors

Before:

```rust,ignore
let result: Result<PointGeoJson, Error<PointError>> =
    points::get_point(&config, latitude, longitude).await;
```

After:

```rust,ignore
let result: Result<PointGeoJson, Error> =
    points::get_point(&config, latitude, longitude).await;

if let Err(Error::Response(response)) = result {
    eprintln!("{} {}", response.status(), response.url());
}
```

### Point stations

Before:

```rust,ignore
let stations = points::get_point_stations(&config, latitude, longitude).await?;
```

After, first resolve point metadata and then use its gridpoint coordinates:

```rust,ignore
let stations = gridpoints::get_gridpoint_stations(
    &config,
    NwsForecastOfficeId::Lox,
    155,
    32,
    None,
).await?;
```

CLI equivalent:

```sh
# Removed
noaa-weather points stations 33.7629 -- -118.1889

# Replacement after reading point metadata
noaa-weather points metadata 33.7629 -- -118.1889
noaa-weather gridpoints stations --forecast-office-id LOX -x 155 -y 32
```

### New CLI commands

Structured endpoints render tables by default and pretty JSON with the global `--json` flag:

```sh
noaa-weather glossary
noaa-weather glossary --json
noaa-weather offices briefing --id PSR
noaa-weather offices weather-stories --id PSR --json
noaa-weather radar spgds [--published <INTERVAL>]
noaa-weather radio transmitters [--cursor <CURSOR>]
noaa-weather radio transmitter <CALL_SIGN>
noaa-weather radio zone <ZONE_ID>
```

Binary office media requires an explicit global output path, rejects `--json`, and is never written to standard output:

```sh
noaa-weather offices briefing-download --id PSR --document-id <ID> --output briefing.pdf
noaa-weather offices briefing-download-latest --id PSR --output briefing.pdf
noaa-weather offices weather-story-image --id PSR --story-id <ID> --output story.png
```

## Fact classification

- **Confirmed spec facts:** hashes, counts, operation additions, unchanged deprecation inventory, parameter/range changes, and component-schema changes above are derived from the two checked-in documents.
- **Live observations:** tolerant office, SPGDS, radio, and redirect behavior is evidence encoded by implementation tests from observed wire shapes.
- **Design decisions/inferences:** removals, forced quantitative flags, compact shared errors, feature layout, exact model permissiveness, and binary-output policy are library choices made from that evidence. They should not be read as claims that NOAA removed an operation or promises every tolerated wire form.
