# Zones

Zone ids are validated against NOAA's shape (state or marine area prefix, `C` or `Z`, three digits) before any request is made; a malformed value is a usage error (exit code 2). `--effective`, `--start`, and `--end` accept an RFC 3339 timestamp or a relative age such as `1d`.

List zones with various filters.

```sh
noaa-weather zones list [--id <ID,...>] [--area <CODE,...>] [--region <CODE,...>] [--type <TYPE,...>] [--point <LAT,LON>] [--include-geometry <true|false>] [--limit <1-500>] [--effective <TIME>]
```

Get metadata for a specific zone.

```sh
noaa-weather zones metadata --type <TYPE> --id <ID> [--effective <TIME>]
```

Get the text forecast for a specific zone.

```sh
noaa-weather zones forecast --type <TYPE> --id <ID>
```

List observation stations within a forecast zone. `--cursor` is accepted because the NOAA API declares it, but NOAA currently publishes a broken `pagination.next` link for this operation (it points at the generic `/stations` listing past the end of the results), so do not feed that cursor back; without `--limit` the first page already lists every station in the zone.

```sh
noaa-weather zones stations --id <ID> [--limit <1-500>] [--cursor <CURSOR>]
```

List observations for a forecast zone.

```sh
noaa-weather zones observations --id <ID> [--start <TIME>] [--end <TIME>] [--limit <1-500>]
```
