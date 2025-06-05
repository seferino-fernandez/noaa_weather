# Zones

List zones with various filters.

```sh
noaa_weather_cli zones list [--id <ID,...>] [--area <CODE,...>] [--region <CODE,...>] [--type <TYPE,...>] [--point <LAT,LON>] [--include-geometry] [--limit <N>] [--effective <ISO_TIME>]
```

Get metadata for a specific zone.

```sh
noaa_weather_cli zones metadata --type <TYPE> --id <ID> [--effective <ISO_TIME>]
```

Get the text forecast for a specific zone.

```sh
noaa_weather_cli zones forecast --type <TYPE> --id <ID>
```

List observation stations within a forecast zone.

```sh
noaa_weather_cli zones stations --id <ID> [--limit <N>] [--cursor <C>]
```

List observations for a forecast zone.

```sh
noaa_weather_cli zones observations --id <ID> [--start <ISO_TIME>] [--end <ISO_TIME>] [--limit <N>]
```
