# Offices

Office IDs accept forecast offices plus regional headquarters (`ARH`, `CRH`, `ERH`, `PRH`, `SRH`, `WRH`) and national headquarters (`NWS`), matching NOAA's `NWSOfficeId` union.

## Get metadata for a NWS office

```sh
noaa-weather offices metadata --id <ID>
```

## Get news headlines for an office

```sh
noaa-weather offices headlines --id <ID>
```

## Get a specific headline by its ID

```sh
noaa-weather offices headline --id <ID> --headline-id <HEADLINE_ID>
```

## Get active briefing metadata

```sh
noaa-weather offices briefing --id <OFFICE>
```

## Download a briefing PDF

Binary downloads require the global `--output` option and do not support `--json`.

```sh
noaa-weather offices briefing-download --id <OFFICE> --document-id <ID> --output briefing.pdf
noaa-weather offices briefing-download-latest --id <OFFICE> --output briefing.pdf
```

## Get active weather-story metadata

```sh
noaa-weather offices weather-stories --id <OFFICE>
```

## Download a weather-story image

```sh
noaa-weather offices weather-story-image --id <OFFICE> --story-id <ID> --output story.png
```
