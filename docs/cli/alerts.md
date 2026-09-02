# Alerts

The deprecated `active` query flag on the general alerts endpoint is not exposed. Use the dedicated active-alert commands below.

Zone ids (`--zone`, `--zone-id`) and points (`--point <LAT,LON>`) are validated before any request is made; a malformed value is a usage error (exit code 2).

## List Active Alerts

```sh
noaa-weather alerts active [--area <CODE,...>] [--point <LAT,LON>] [--zone <ID,...>] [--severity <LEVEL,...>] ...
```

## Get Active Alerts for a Specific Area (State/Territory or Marine Area)

```sh
noaa-weather alerts area --area <AREA>
```

## Get Count of Active Alerts

```sh
noaa-weather alerts count
```

## Get Active Alerts for a Marine Region

```sh
noaa-weather alerts marine-region --marine-region <REGION>
```

## Get Active Alerts for a Zone

```sh
noaa-weather alerts zone --zone-id <ZONE_ID>
```

## List Alerts, Including Past Ones

`--start` and `--end` accept an RFC 3339 timestamp or a relative age such as `6h`, `30m`, or `2d`, resolved when the command starts. `--cursor` accepts the opaque pagination cursor from a previous page's `pagination.next` value in `--json` output.

```sh
noaa-weather alerts list [--start <TIME>] [--end <TIME>] [--limit <1-500>] [--cursor <CURSOR>] ...
noaa-weather alerts list --start 6h --status actual
```

## Get a Single Alert by ID

```sh
noaa-weather alerts alert --id <ID>
```

## List Available Alert Types

```sh
noaa-weather alerts types
```
