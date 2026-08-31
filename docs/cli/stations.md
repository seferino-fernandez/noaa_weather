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

The date and time are the final two path segments of an `id` returned by `terminal-aerodrome-forecasts`. The human table presents normalized forecast meaning: report state and validity, base/change semantics, CAVOK, wind, visibility, exact weather code plus description, cloud layers and convective types, vertical visibility, and temperature extrema. Omitted values in a change group display as unchanged; IWXXM nil reasons display as unavailable or no significant conditions instead of being collapsed into `N/A`.

Use `--json` with either command for pretty JSON. A specific TAF serializes to the same semantic model used by the table, without IWXXM namespaces or XML wrapper fields.

Both TAF commands require the `xml` feature. Normal builds include them because the default `radio` feature enables `xml`. A `--no-default-features` CLI build omits both TAF commands and `quick-xml`; build with `--no-default-features --features xml` to retain TAF support without radio.
