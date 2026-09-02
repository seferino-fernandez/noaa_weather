# Points

Get point metadata for a latitude and longitude, given as one `LAT,LON` value in decimal degrees:

```sh
noaa-weather points metadata <LAT,LON>
noaa-weather points metadata 39.7456,-97.0892
```

Because the pair is one argument, negative longitudes need no `--` separator. Latitude must be within -90..=90 and longitude within -180..=180; values are rounded to four decimals, and a malformed pair is a usage error (exit code 2).

The deprecated `/points/{latitude},{longitude}/stations` operation is not exposed. Resolve point metadata first, then use its gridpoint with `gridpoints stations` or query stations directly.
