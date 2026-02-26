# Points

Get point metadata for a specific latitude and longitude:

```sh
noaa-weather points metadata <LATITUDE> <LONGITUDE>
```

Note: Use `--` before negative longitude values to prevent them from being interpreted as flags:

```sh
noaa-weather points metadata 39.7456 -- -97.0892
```

Get observation stations near a specific point:

```sh
noaa-weather points stations <LATITUDE> <LONGITUDE>
```
