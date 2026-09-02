# Stations

Station ids are 3 to 16 letters or digits and are upper-cased before the request. Time flags (`--start`, `--end`, `--time`, `--issued`) accept an RFC 3339 timestamp or a relative age such as `6h`, `30m`, or `2d`, resolved when the command starts. Malformed values are usage errors (exit code 2).

Get station metadata:

```sh
noaa-weather stations metadata --id <ID>
```

List stations, optionally filtering by ID or state/marine area. `--cursor` accepts the opaque pagination cursor from a previous page's `pagination.next` value in `--json` output.

```sh
noaa-weather stations list [--id <ID1,ID2...>] [--state <ST1,ST2...>] [--limit <1-500>] [--cursor <CURSOR>]
```

Get the latest observation for a station.

```sh
noaa-weather stations latest-observation --station-id <ID> [--require-quality-controlled]
```

List historical observations for a station. `--cursor` accepts the opaque pagination cursor from a previous page's `pagination.next` value in `--json` output.

```sh
noaa-weather stations observations --station-id <ID> [--start <TIME>] [--end <TIME>] [--limit <1-500>] [--cursor <CURSOR>]
noaa-weather stations observations --station-id KPHX --start 6h --end 1h
```

Get a specific observation by time.

```sh
noaa-weather stations observation --station-id <ID> --time <TIME>
```

Get all available Terminal Aerodrome Forecasts (TAFs) for a station.

```sh
noaa-weather stations terminal-aerodrome-forecasts --station-id <ID>
```

Get a specific Terminal Aerodrome Forecast (TAF) for a station by its issue time.

```sh
noaa-weather stations terminal-aerodrome-forecast --station-id <ID> --issued <TIME>
noaa-weather stations terminal-aerodrome-forecast --station-id KPHX --issued 2026-08-30T22:54:00Z
```

NOAA addresses a TAF by its UTC issue date and `HHMM` minute, which are the final two path segments of an `id` returned by `terminal-aerodrome-forecasts`; `--issued` takes the same instant as one timestamp and seconds are dropped. The human table presents normalized forecast meaning: report state and validity, base/change semantics, CAVOK, wind, visibility, exact weather code plus description, cloud layers and convective types, vertical visibility, and temperature extrema. Omitted values in a change group display as unchanged; IWXXM nil reasons display as unavailable or no significant conditions instead of being collapsed into `N/A`.

Use `--json` with either command for pretty JSON. A specific TAF serializes to the same semantic model used by the table, without IWXXM namespaces or XML wrapper fields.
