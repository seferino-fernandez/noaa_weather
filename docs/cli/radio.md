# Radio

Get the NOAA Weather Radio broadcast for a transmitter station by call sign:

```sh
noaa-weather radio station <CALL_SIGN>
```

Get the NOAA Weather Radio broadcast for a geographic point:

```sh
noaa-weather radio point <LATITUDE> <LONGITUDE>
```

Note: Use `--` before negative longitude values to prevent them from being interpreted as flags:

```sh
noaa-weather radio point 33.4484 -- -112.0740
```
