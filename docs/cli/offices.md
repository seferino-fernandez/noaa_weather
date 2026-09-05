# Offices

Office IDs are 3 or 4 letter NWS codes. Forecast offices plus regional headquarters (`ARH`, `CRH`, `ERH`, `PRH`, `SRH`, `WRH`) and national headquarters (`NWS`) are accepted; `--help` lists the known forecast offices as a hint without restricting the value. A malformed code is a usage error (exit code 2).

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

Briefing PDFs and weather-story images are not summarized because they are binary downloads written directly to the required output path.

<!-- BEGIN GENERATED SHOWN/OMITTED -->

## Human-summary property coverage

The table is generated from the summary contracts. `Shown` properties appear in keyed human-summary content; `Otherwise accounted for` properties are deliberately handled without a keyed table or fact.

| Response | Property | Treatment | Reason |
| :--- | :--- | :--- | :--- |
| Active briefing metadata | `briefing` | Shown | — |
| Active weather-story metadata | `altText` | Shown | — |
| Active weather-story metadata | `description` | Shown | — |
| Active weather-story metadata | `download` | Shown | — |
| Active weather-story metadata | `endTime` | Otherwise accounted for | publication scheduling metadata is available with JSON output |
| Active weather-story metadata | `officeId` | Otherwise accounted for | the command already identifies the publishing office |
| Active weather-story metadata | `order` | Shown | — |
| Active weather-story metadata | `priority` | Shown | — |
| Active weather-story metadata | `startTime` | Otherwise accounted for | publication scheduling metadata is available with JSON output |
| Active weather-story metadata | `stories` | Shown | — |
| Active weather-story metadata | `title` | Shown | — |
| Active weather-story metadata | `updateTime` | Otherwise accounted for | publication scheduling metadata is available with JSON output |
| Office headline | `@id` | Otherwise accounted for | the server-issued headline identifier is shown |
| Office headline | `content` | Otherwise accounted for | raw HTML is available with JSON output |
| Office headline | `id` | Shown | — |
| Office headline | `important` | Shown | — |
| Office headline | `issuanceTime` | Shown | — |
| Office headline | `link` | Shown | — |
| Office headline | `name` | Shown | — |
| Office headline | `office` | Shown | — |
| Office headline | `summary` | Shown | — |
| Office headline | `title` | Otherwise accounted for | shown as the summary title |
| Office headline list | `@graph` | Shown | — |
| Office headline list | `@id` | Otherwise accounted for | the server-issued headline identifier is shown |
| Office headline list | `content` | Otherwise accounted for | raw HTML is available with JSON output |
| Office headline list | `id` | Shown | — |
| Office headline list | `important` | Shown | — |
| Office headline list | `issuanceTime` | Shown | — |
| Office headline list | `link` | Shown | — |
| Office headline list | `name` | Otherwise accounted for | the title is more descriptive in a headline list |
| Office headline list | `office` | Otherwise accounted for | the command already identifies the publishing office |
| Office headline list | `summary` | Shown | — |
| Office headline list | `title` | Shown | — |
| Office metadata | `@id` | Otherwise accounted for | the office identifier addresses the same resource |
| Office metadata | `@type` | Otherwise accounted for | fixed organization and postal-address types |
| Office metadata | `address` | Shown | — |
| Office metadata | `addressLocality` | Shown | — |
| Office metadata | `addressRegion` | Shown | — |
| Office metadata | `approvedObservationStations` | Shown | — |
| Office metadata | `email` | Shown | — |
| Office metadata | `faxNumber` | Shown | — |
| Office metadata | `id` | Shown | — |
| Office metadata | `name` | Otherwise accounted for | shown as the summary title |
| Office metadata | `nwsRegion` | Shown | — |
| Office metadata | `parentOrganization` | Shown | — |
| Office metadata | `postalCode` | Shown | — |
| Office metadata | `responsibleCounties` | Shown | — |
| Office metadata | `responsibleFireZones` | Shown | — |
| Office metadata | `responsibleForecastZones` | Shown | — |
| Office metadata | `sameAs` | Shown | — |
| Office metadata | `streetAddress` | Shown | — |
| Office metadata | `telephone` | Shown | — |

<!-- END GENERATED SHOWN/OMITTED -->
