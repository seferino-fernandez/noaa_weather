# Zones

List zones with various filters.

```sh
noaa-weather zones list [--id <ID,...>] [--area <CODE,...>] [--region <CODE,...>] [--type <TYPE,...>] [--point <LAT,LON>] [--include-geometry] [--limit <N>] [--effective <ISO_TIME>]
```

Get metadata for a specific zone.

```sh
noaa-weather zones metadata --type <TYPE> --id <ID> [--effective <ISO_TIME>]
```

Get the text forecast for a specific zone.

```sh
noaa-weather zones forecast --type <TYPE> --id <ID>
```

List observation stations within a forecast zone.

```sh
noaa-weather zones stations --id <ID> [--limit <N>] [--cursor <C>]
```

List observations for a forecast zone.

```sh
noaa-weather zones observations --id <ID> [--start <ISO_TIME>] [--end <ISO_TIME>] [--limit <N>]
```
