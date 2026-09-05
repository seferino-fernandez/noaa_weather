# Zones

Zone ids are validated against NOAA's shape (state or marine area prefix, `C` or `Z`, three digits) before any request is made; a malformed value is a usage error (exit code 2). `--effective`, `--start`, and `--end` accept an RFC 3339 timestamp or a relative age such as `1d`.

List zones with various filters.

```sh
noaa-weather zones list [--id <ID,...>] [--area <CODE,...>] [--region <CODE,...>] [--type <TYPE,...>] [--point <LAT,LON>] [--include-geometry <true|false>] [--limit <1-500>] [--effective <TIME>]
```

Get metadata for a specific zone.

```sh
noaa-weather zones metadata --type <TYPE> --id <ID> [--effective <TIME>]
```

Get the text forecast for a specific zone.

```sh
noaa-weather zones forecast --type <TYPE> --id <ID>
```

List observation stations within a forecast zone. `--cursor` is accepted because the NOAA API declares it, but NOAA currently publishes a broken `pagination.next` link for this operation (it points at the generic `/stations` listing past the end of the results), so do not feed that cursor back; without `--limit` the first page already lists every station in the zone.

The response uses the canonical [station human-summary property coverage](stations.md#human-summary-property-coverage).

```sh
noaa-weather zones stations --id <ID> [--limit <1-500>] [--cursor <CURSOR>]
```

List observations for a forecast zone.

The response uses the canonical [station human-summary property coverage](stations.md#human-summary-property-coverage).

```sh
noaa-weather zones observations --id <ID> [--start <TIME>] [--end <TIME>] [--limit <1-500>]
```

<!-- BEGIN GENERATED SHOWN/OMITTED -->

## Human-summary property coverage

The table is generated from the summary contracts. `Shown` properties appear in keyed human-summary content; `Otherwise accounted for` properties are deliberately handled without a keyed table or fact.

| Response | Property | Treatment | Reason |
| :--- | :--- | :--- | :--- |
| Zone forecast | `detailedForecast` | Shown | — |
| Zone forecast | `geometry` | Otherwise accounted for | the forecast text identifies the zone |
| Zone forecast | `name` | Shown | — |
| Zone forecast | `number` | Otherwise accounted for | period order is already preserved by the table |
| Zone forecast | `periods` | Shown | — |
| Zone forecast | `properties` | Otherwise accounted for | the forecast; its keys are accounted for one by one |
| Zone forecast | `type` | Otherwise accounted for | the feature envelope is always Feature |
| Zone forecast | `updated` | Shown | — |
| Zone forecast | `zone` | Shown | — |
| Zone list | `@id` | Otherwise accounted for | the typed zone identifier is enough for the next command |
| Zone list | `@type` | Otherwise accounted for | always wx:Zone |
| Zone list | `awipsLocationIdentifier` | Otherwise accounted for | available from the single-zone metadata command |
| Zone list | `cwa` | Otherwise accounted for | deprecated office identifiers duplicate forecastOffice |
| Zone list | `effectiveDate` | Otherwise accounted for | catalog rows omit zone-definition history |
| Zone list | `expirationDate` | Otherwise accounted for | catalog rows omit zone-definition history |
| Zone list | `features` | Otherwise accounted for | each zone is one table row |
| Zone list | `forecastOffice` | Shown | — |
| Zone list | `forecastOffices` | Otherwise accounted for | deprecated office URLs duplicate forecastOffice |
| Zone list | `geometry` | Otherwise accounted for | catalog rows use the zone name and identifier |
| Zone list | `gridIdentifier` | Otherwise accounted for | available from the single-zone metadata command |
| Zone list | `id` | Shown | the feature URL duplicates the typed zone identifier |
| Zone list | `name` | Shown | — |
| Zone list | `observationStations` | Shown | — |
| Zone list | `pagination` | Otherwise accounted for | surfaced as the more-zones note |
| Zone list | `properties` | Otherwise accounted for | each zone's keys are accounted for one by one |
| Zone list | `radarStation` | Otherwise accounted for | available from the single-zone metadata command |
| Zone list | `state` | Shown | — |
| Zone list | `timeZone` | Shown | — |
| Zone list | `title` | Otherwise accounted for | zone collections do not carry a title |
| Zone list | `type` | Shown | always FeatureCollection or Feature; the zone type is shown |
| Zone list | `updated` | Otherwise accounted for | zone collections do not carry an update time |
| Zone metadata | `@id` | Otherwise accounted for | the typed zone identifier is enough for the next command |
| Zone metadata | `@type` | Otherwise accounted for | always wx:Zone |
| Zone metadata | `awipsLocationIdentifier` | Shown | — |
| Zone metadata | `cwa` | Otherwise accounted for | deprecated office identifiers duplicate forecastOffice |
| Zone metadata | `effectiveDate` | Shown | — |
| Zone metadata | `expirationDate` | Shown | — |
| Zone metadata | `forecastOffice` | Shown | — |
| Zone metadata | `forecastOffices` | Otherwise accounted for | deprecated office URLs duplicate forecastOffice |
| Zone metadata | `geometry` | Otherwise accounted for | the zone name and identifier are more useful in text output |
| Zone metadata | `gridIdentifier` | Shown | — |
| Zone metadata | `id` | Shown | the envelope URL duplicates the typed zone identifier |
| Zone metadata | `name` | Shown | — |
| Zone metadata | `observationStations` | Shown | — |
| Zone metadata | `properties` | Otherwise accounted for | the zone; its keys are accounted for one by one |
| Zone metadata | `radarStation` | Shown | — |
| Zone metadata | `state` | Shown | — |
| Zone metadata | `timeZone` | Shown | — |
| Zone metadata | `type` | Shown | the feature envelope is always Feature; the zone type is shown |

<!-- END GENERATED SHOWN/OMITTED -->
