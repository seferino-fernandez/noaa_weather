# Aviation

CWSU and ATSU identifiers are validated before any request is made (3 to 4 and exactly 4 letters or digits respectively); a malformed value is a usage error (exit code 2).

## Get a specific Center Weather Advisory (CWA)

```bash
noaa-weather aviation cwa --cwsu-id <CWSU_ID> --date <YYYY-MM-DD> --sequence <N>
```

## Get all current CWAs for a Center Weather Service Unit (CWSU)

```bash
noaa-weather aviation cwas --cwsu-id <CWSU_ID>
```

## Get metadata for a Center Weather Service Unit (CWSU)

```bash
noaa-weather aviation cwsu --cwsu-id <CWSU_ID>
```

## Get a specific SIGMET/AIRMET product identified by ATSU and issue time

NOAA addresses a product by its UTC issue date and `HHMM` minute; `--issued` takes one RFC 3339 timestamp and the CLI splits it (seconds are dropped).

```bash
noaa-weather aviation sigmet --atsu <ATSU> --issued <TIME>
noaa-weather aviation sigmet --atsu KKCI --issued 2025-04-18T14:30:00Z
```

## Query available SIGMET/AIRMET products with filters

`--start` and `--end` accept an RFC 3339 timestamp or a relative age such as `6h`.

```bash
noaa-weather aviation sigmets [--atsu <ATSU>] [--date <YYYY-MM-DD>] [--start <TIME>] [--end <TIME>] [--sequence <SEQUENCE>]
```
