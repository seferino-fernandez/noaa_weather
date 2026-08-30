# Points

Get point metadata for a specific latitude and longitude:

```sh
noaa-weather points metadata <LATITUDE> <LONGITUDE>
```

Note: Use `--` before negative longitude values to prevent them from being interpreted as flags:

```sh
noaa-weather points metadata 39.7456 -- -97.0892
```

The deprecated `/points/{latitude},{longitude}/stations` operation is not exposed. Resolve point metadata first, then use its gridpoint with `gridpoints stations` or query stations directly.
