# NOAA Weather CLI

## Installation

### Installation

#### Homebrew

Install the `noaa-weather` CLI tool using Homebrew:

```bash
brew tap seferino-fernandez/tools
brew install noaa-weather
```

#### Cargo

Install the `noaa-weather` CLI tool using Cargo:

```bash
cargo install noaa-weather
```

## CLI Commands

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

## Output Formats

All commands support multiple output formats:

-   **Human-readable tables** (default) - Perfect for terminal viewing
-   **JSON** (`--json`) - Machine-readable for scripting and integration
-   **File output** (`--output file.txt`) - Save results to a file

## General Examples

### Get Weather Alerts for California

```bash
noaa-weather alerts area --area CA
```

### Retrieve Metadata for Multiple Weather Stations

```bash
noaa-weather stations list --state CA,NV --limit 50
```

### Get Aviation Weather for Specific Airport

```bash
noaa-weather aviation cwas --cwsu-id ZLA
```

Get latest observations from a weather station:

```bash
noaa-weather stations latest-observation --station-id KJFK
```

Get aviation weather (SIGMETs):

````bash
noaa-weather aviation sigmets
```s

For detailed documentation on each command, run:

```bash
noaa-weather <command> --help
````

## Get Information for your location

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
noaa-weather gridpoints forecast-hourly --forecast-office-id LOX --x 155 --y 32
```
