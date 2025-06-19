# Stations

Get station metadata:

```sh
noaa-weather stations metadata --station-id <ID>
```

List stations, optionally filtering by ID or state.

```sh
noaa-weather stations list [--id <ID1,ID2...>] [--state <ST1,ST2...>] [--limit <N>] [--cursor <C>]
```

Get the latest observation for a station.

```sh
noaa-weather stations latest-observation --station-id <ID> [--require-qc]
```

List historical observations for a station.

```sh
noaa-weather stations observations --station-id <ID> [--start <ISO_TIME>] [--end <ISO_TIME>] [--limit <N>]
```

Get a specific observation by time.

```sh
noaa-weather stations observation --station-id <ID> --time <ISO_TIME>
```

Get all available Terminal Aerodrome Forecasts (TAFs) for a station.

```sh
noaa-weather stations terminal-aerodrome-forecasts --station-id <ID>
```

Get a specific Terminal Aerodrome Forecast (TAF) for a station by date and time.

```sh
noaa-weather stations terminal-aerodrome-forecast --station-id <ID> --date <YYYY-MM-DD> --time <HHMM>
```
