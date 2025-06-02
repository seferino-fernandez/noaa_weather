# Stations

Get station metadata:

```sh
noaa_weather_cli stations metadata --station-id <ID>
```

List stations, optionally filtering by ID or state.

```sh
noaa_weather_cli stations list [--id <ID1,ID2...>] [--state <ST1,ST2...>] [--limit <N>] [--cursor <C>]
```

Get the latest observation for a station.

```sh
noaa_weather_cli stations latest-observation --station-id <ID> [--require-qc]
```

List historical observations for a station.

```sh
noaa_weather_cli stations observations --station-id <ID> [--start <ISO_TIME>] [--end <ISO_TIME>] [--limit <N>]
```

Get a specific observation by time.

```sh
noaa_weather_cli stations observation --station-id <ID> --time <ISO_TIME>
```

Get all available Terminal Aerodrome Forecasts (TAFs) for a station.

```sh
noaa_weather_cli stations terminal-aerodrome-forecasts --station-id <ID>
```

Get a specific Terminal Aerodrome Forecast (TAF) for a station by date and time.

```sh
noaa_weather_cli stations terminal-aerodrome-forecast --station-id <ID> --date <YYYY-MM-DD> --time <HHMM>
```
