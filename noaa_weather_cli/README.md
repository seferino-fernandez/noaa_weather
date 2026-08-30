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

## Output contract

Structured commands render concise tables by default. Use the global `--json` flag for pretty JSON or `--output <PATH>` to write structured output to a file:

```bash
noaa-weather glossary
noaa-weather glossary --json
noaa-weather stations latest-observation --station-id KJFK --output observation.txt
```

Office PDF and image downloads are deliberately different: `--output <PATH>` is required, `--json` is rejected, and binary bytes are never emitted to standard output.

## Command groups

| Command | Description | Example |
| --- | --- | --- |
| `glossary` | NWS glossary terms | `noaa-weather glossary` |
| `alerts` | Alerts, warnings, and watches | `noaa-weather alerts area --area CA` |
| `gridpoints` | Forecast and raw grid data | `noaa-weather gridpoints forecast --forecast-office-id TOP -x 31 -y 80` |
| `points` | Point metadata | `noaa-weather points metadata 40.7128 -- -74.0060` |
| `stations` | Observation stations and observations | `noaa-weather stations latest-observation --station-id KJFK` |
| `zones` | Zone metadata and forecasts | `noaa-weather zones forecast --type forecast --id CAZ006` |
| `offices` | Office metadata, briefings, and weather stories | `noaa-weather offices briefing --id PSR` |
| `radar` | Radar stations, queues, and SPGDS | `noaa-weather radar spgds` |
| `radio` | Transmitters and broadcast transcripts | `noaa-weather radio transmitter KEC94` |
| `aviation` | SIGMETs and center weather advisories | `noaa-weather aviation sigmets --atsu KKCI` |
| `products` | NWS text products | `noaa-weather products latest --type-id AFD --location-id PSR` |

Normal defaults include radio and XML support, so transmitter/broadcast commands and both Terminal Aerodrome Forecast (TAF) commands are present. From a repository checkout, choose a smaller feature surface explicitly:

```bash
# JSON endpoints only: no radio, TAF commands, or quick-xml
cargo install --path noaa_weather_cli --no-default-features

# Keep both TAF commands and quick-xml, but omit radio
cargo install --path noaa_weather_cli --no-default-features --features xml
```

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

The existing broadcast commands remain available:

```bash
noaa-weather radio station KEC94
noaa-weather radio point 33.4484 -- -112.0740
```

## Location workflow

Start with point metadata:

```bash
noaa-weather points metadata 33.7629 -- -118.1889
```

Use the returned forecast office and grid coordinates for forecasts or gridpoint stations:

```bash
noaa-weather gridpoints forecast-hourly --forecast-office-id LOX -x 155 -y 32
noaa-weather gridpoints stations --forecast-office-id LOX -x 155 -y 32
```

The deprecated point-stations endpoint and `points stations` command were removed. The forecast CLI also manages NOAA's quantitative forecast feature flags internally; callers should not pass `--feature-flags`.

See the [command guides](../docs/README.md) or run:

```bash
noaa-weather <command> --help
```
