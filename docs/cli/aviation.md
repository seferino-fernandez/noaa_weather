# Aviation

## Get a specific Center Weather Advisory (CWA)

```bash
noaa_weather_cli aviation cwa --cwsu-id <CWSU_ID> --date <DATE>
```

## Get all current CWAs for a Center Weather Service Unit (CWSU)

```bash
noaa_weather_cli aviation cwas --cwsu-id <CWSU_ID>
```

## Get metadata for a Center Weather Service Unit (CWSU)

```bash
noaa_weather_cli aviation cwsu --cwsu-id <CWSU_ID>
```

## Get a specific SIGMET/AIRMET product identified by ATSU, date, and time

```bash
noaa_weather_cli aviation sigmet --atsu <ATSU> --date <DATE> --time <TIME>
```

## Query available SIGMET/AIRMET products with filters

```bash
noaa_weather_cli aviation sigmets --atsu <ATSU> --date <DATE> --start <START> --end <END> --sequence <SEQUENCE>
```
