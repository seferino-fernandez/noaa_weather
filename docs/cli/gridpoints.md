# Gridpoints

[NOAA Gridpoints Documentation](https://weather-gov.github.io/api/gridpoints)

Every gridpoint command takes one positional grid cell as `OFFICE/X,Y` (for example `TOP/31,80`), the same shape NOAA uses in its URLs. Find it with `noaa-weather points metadata <LAT,LON>`. A malformed value is a usage error (exit code 2).

Forecast commands automatically request NOAA's quantitative temperature and wind values; there is no caller-supplied `--feature-flags` option. Under those feature flags NOAA answers in its own pipeline units — Celsius and km/h — whichever `units` the request asks for, so the API's `units` parameter changes nothing but the narrative text this CLI does not print. Choose what you read with the global `--units <us|si>` instead.

## Gridpoint

```sh
noaa-weather gridpoints gridpoint <OFFICE/X,Y>
```

## Forecast

```sh
noaa-weather gridpoints forecast <OFFICE/X,Y>
```

## Hourly Forecast

```sh
noaa-weather gridpoints forecast-hourly <OFFICE/X,Y>
```

## Stations

```sh
noaa-weather gridpoints stations <OFFICE/X,Y> [--limit <1-500>]
```
