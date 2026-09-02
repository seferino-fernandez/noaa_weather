# Radar

Interval flags (`--published`, `--arrived`, `--created`, `--time`, `--interval`) take an ISO 8601 time interval in any of its four forms: `start/end`, `start/duration`, `duration/end`, or a bare `duration` (for example `2026-08-30T00:00:00Z/PT1H` or `PT1H`). Radar station ids are four or five letters or digits (for example `KFSX` or the profiler `HWPA2`). Malformed values are usage errors (exit code 2).

## SPGDS telemetry

Get radar SPGDS telemetry, optionally restricted by a publication interval:

```sh
noaa-weather radar spgds [--published <INTERVAL>]
```

## Data queues

Get queue data for `rds` or `tds`. The limit must be between 1 and 50,000 and defaults to 10.

```sh
noaa-weather radar data-queue --host <rds|tds> [--limit <LIMIT>]
```

Additional queue filters include `--arrived`, `--created`, `--published` (intervals), `--station` (radar station id), `--type`, `--feed`, and `--resolution`.

## Servers

```sh
noaa-weather radar server --id <ID> [--reporting-host <HOST>]
noaa-weather radar servers [--reporting-host <HOST>]
```

## Stations and alarms

```sh
noaa-weather radar station --station-id <ID> [--reporting-host <HOST>] [--host <rds|tds>]
noaa-weather radar station-alarms --station-id <ID>
noaa-weather radar stations [--station-type <TYPE>...] [--reporting-host <HOST>] [--host <rds|tds>]
```

## Wind profilers

```sh
noaa-weather radar wind-profiler --id <ID> [--time <INTERVAL>] [--interval <INTERVAL>]
```

Typed radar results render as tables by default and support the global `--json` option. Wind-profiler data has no stable typed response model, so that command always emits pretty JSON; `--json` is accepted but redundant there.
