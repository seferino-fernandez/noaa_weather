# Aviation

## Get a specific Center Weather Advisory (CWA)

```bash
noaa-weather aviation cwa --cwsu-id <CWSU_ID> --date <DATE>
```

## Get all current CWAs for a Center Weather Service Unit (CWSU)

```bash
noaa-weather aviation cwas --cwsu-id <CWSU_ID>
```

## Get metadata for a Center Weather Service Unit (CWSU)

```bash
noaa-weather aviation cwsu --cwsu-id <CWSU_ID>
```

## Get a specific SIGMET/AIRMET product identified by ATSU, date, and time

```bash
noaa-weather aviation sigmet --atsu <ATSU> --date <DATE> --time <TIME>
```

## Query available SIGMET/AIRMET products with filters

```bash
noaa-weather aviation sigmets --atsu <ATSU> --date <DATE> --start <START> --end <END> --sequence <SEQUENCE>
```
