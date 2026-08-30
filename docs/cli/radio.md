# Radio

Radio commands are available when the default `radio` feature is enabled.

List transmitter metadata, optionally continuing from an opaque pagination cursor:

```sh
noaa-weather radio transmitters [--cursor <CURSOR>]
```

Get one transmitter by call sign:

```sh
noaa-weather radio transmitter <CALL_SIGN>
```

Get transmitters serving a county zone:

```sh
noaa-weather radio zone <ZONE_ID>
```

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

Structured transmitter commands use tables by default and support the global `--json` option. Broadcast commands retain their readable transcript output and also support JSON.
