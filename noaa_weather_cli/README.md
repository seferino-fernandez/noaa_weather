# NOAA Weather CLI

`noaa-weather` is a typed command-line interface for version 3.11.0 of the NOAA Weather API.

## Installation

With Homebrew:

```bash
brew tap seferino-fernandez/tools
brew install noaa-weather
```

From a repository checkout with Cargo:

```bash
cargo install --path noaa_weather_cli
```

Every command group, including NOAA Weather Radio and Terminal Aerodrome Forecasts, is always built in; there are no optional features.

## Output contract

Structured commands render concise tables by default. Use the global `--json` flag (or `--format json`) for pretty JSON, or `--output <PATH>` to write output to a file. Use `--output -` to select standard output explicitly. `--color`, `--width` and `--time-zone` are global too; [the documentation](../docs/README.md) lists what each accepts.

```bash
noaa-weather glossary
noaa-weather glossary --json
noaa-weather stations latest-observation --station-id KJFK --output observation.txt
noaa-weather alerts active --area AZ --json --output -
```

Text and JSON files replace their destination atomically only after a successful request and rendering pass. Textual output always ends in exactly one newline.

Office PDF and image downloads are deliberately different: `--output <PATH>` is required, `--json` and `--output -` are rejected, empty responses are not saved, and binary bytes are never emitted to standard output.

## Typed arguments

Identifiers, coordinates, and intervals are validated before any request is made, so a malformed value is a usage error with exit code 2 and a specific message (`invalid station id "K PHX": must be 3 to 16 ASCII letters or digits`). Lower-case codes are accepted and normalized.

| Kind | Shape | Example |
| --- | --- | --- |
| Point | one `LAT,LON` positional in decimal degrees | `points metadata 39.7456,-97.0892` |
| Grid cell | one `OFFICE/X,Y` positional | `gridpoints forecast TOP/31,80` |
| Station, office, zone, call sign, product code | NOAA code | `--station-id KPHX`, `--id PSR`, `--zone-id CAZ043` |
| Time (`--start`, `--end`, `--time`, `--issued`, `--effective`, `--start-time`, `--end-time`) | RFC 3339 timestamp or relative age `Nm`/`Nh`/`Nd` | `--start 2026-08-30T12:00:00Z`, `--start 6h` |
| Date (`--date`) | `YYYY-MM-DD` | `--date 2026-08-30` |
| Interval (`--published`, `--arrived`, `--created`, `--time`, `--interval` on radar) | ISO 8601 interval | `--published 2026-08-30T00:00:00Z/PT1H`, `--published PT1H` |
| Limit | `1` to `500` (`1` to `50000` for radar queues) | `--limit 10` |

Relative ages are resolved against the current time when the command starts. Office arguments list the known forecast offices in `--help` as a hint; any well-formed code, including regional (`WRH`) and national (`NWS`) headquarters, is accepted.

## Command groups

| Command | Description | Example |
| --- | --- | --- |
| `glossary` | NWS glossary terms | `noaa-weather glossary` |
| `alerts` | Alerts, warnings, and watches | `noaa-weather alerts area --area CA` |
| `gridpoints` | Forecast and raw grid data | `noaa-weather gridpoints forecast TOP/31,80` |
| `points` | Point metadata | `noaa-weather points metadata 40.7128,-74.0060` |
| `stations` | Observation stations and observations | `noaa-weather stations latest-observation --station-id KJFK` |
| `zones` | Zone metadata and forecasts | `noaa-weather zones forecast --type forecast --id CAZ006` |
| `offices` | Office metadata, briefings, and weather stories | `noaa-weather offices briefing --id PSR` |
| `radar` | Radar stations, queues, and SPGDS | `noaa-weather radar spgds` |
| `radio` | Transmitters and broadcast transcripts | `noaa-weather radio transmitter KEC94` |
| `aviation` | SIGMETs and center weather advisories | `noaa-weather aviation sigmets --atsu KKCI` |
| `products` | NWS text products | `noaa-weather products latest --type-id AFD --location-id PSR` |

Fetch the current TAF identifiers, then request one forecast by its issue time (NOAA addresses a TAF by UTC date and `HHMM` minute, which the CLI derives from the timestamp):

```bash
noaa-weather stations terminal-aerodrome-forecasts --station-id KPHX
noaa-weather stations terminal-aerodrome-forecast \
  --station-id KPHX --issued 2026-08-30T22:54:00Z
```

Human TAF output distinguishes CAVOK, unchanged, unavailable, no-significant, cancelled, and missing states. It includes exact weather codes with descriptions, normalized wind/visibility/cloud values, convective cloud types, and temperature extrema. Add `--json` for the same normalized forecast meaning as semantic JSON; IWXXM namespaces and wrapper elements are not exposed.

## NOAA 3.11 commands

```bash
# Typed glossary terms
noaa-weather glossary

# Office briefing and weather-story metadata
noaa-weather offices briefing --id PSR
noaa-weather offices weather-stories --id PSR

# Explicit binary downloads
noaa-weather offices briefing-download --id PSR --document-id <ID> --output briefing.pdf
noaa-weather offices briefing-download-latest --id PSR --output briefing.pdf
noaa-weather offices weather-story-image --id PSR --story-id <ID> --output story.png

# Radar telemetry
noaa-weather radar spgds
noaa-weather radar spgds --published '2026-08-30T00:00:00Z/2026-08-30T01:00:00Z'

# Weather Radio transmitter metadata
noaa-weather radio transmitters
noaa-weather radio transmitters --cursor <CURSOR>
noaa-weather radio transmitter KEC94
noaa-weather radio zone AZC013
```

The broadcast commands:

```bash
noaa-weather radio station KEC94
noaa-weather radio point 33.4484,-112.0740
```

## Location workflow

Start with point metadata:

```bash
noaa-weather points metadata 33.7629,-118.1889
```

Use the returned forecast office and grid coordinates as one `OFFICE/X,Y` value for forecasts or gridpoint stations:

```bash
noaa-weather gridpoints forecast-hourly LOX/155,32
noaa-weather gridpoints stations LOX/155,32
```

Recent observations with relative times:

```bash
noaa-weather stations observations --station-id KLAX --start 6h --end 1h --limit 5
```

The deprecated point-stations endpoint and `points stations` command were removed. The forecast CLI also manages NOAA's quantitative forecast feature flags internally; callers should not pass `--feature-flags`.

See the [command guides](../docs/README.md) or run:

```bash
noaa-weather <command> --help
```
