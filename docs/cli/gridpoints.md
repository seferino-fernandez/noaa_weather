# Gridpoints

[NOAA Gridpoints Documentation](https://weather-gov.github.io/api/gridpoints)

Every gridpoint command takes one positional grid cell as `OFFICE/X,Y` (for example `TOP/31,80`), the same shape NOAA uses in its URLs. Find it with `noaa-weather points metadata <LAT,LON>`. A malformed value is a usage error (exit code 2).

Forecast commands automatically request NOAA's quantitative temperature and wind values; there is no caller-supplied `--feature-flags` option. Under those feature flags NOAA answers in its own pipeline units — Celsius and km/h — whichever `units` the request asks for, so the API's `units` parameter changes nothing but the narrative text this CLI does not print. Choose what you read with the global `--units <us|si>` instead.

## Gridpoint

```sh
noaa-weather gridpoints gridpoint <OFFICE/X,Y>
```

## Forecast

```sh
noaa-weather gridpoints forecast <OFFICE/X,Y>
```

## Hourly Forecast

```sh
noaa-weather gridpoints forecast-hourly <OFFICE/X,Y>
```

## Stations

This command uses the canonical [station human-summary property coverage](stations.md#human-summary-property-coverage).

```sh
noaa-weather gridpoints stations <OFFICE/X,Y> [--limit <1-500>]
```

<!-- BEGIN GENERATED SHOWN/OMITTED -->

## Human-summary property coverage

The table is generated from the summary contracts. `Shown` properties appear in keyed human-summary content; `Otherwise accounted for` properties are deliberately handled without a keyed table or fact.

| Response | Property | Treatment | Reason |
| :--- | :--- | :--- | :--- |
| Forecast and hourly forecast | `detailedForecast` | Otherwise accounted for | a paragraph per period; the short forecast is already a column and twelve paragraphs are not a summary |
| Forecast and hourly forecast | `elevation` | Otherwise accounted for | the grid cell's elevation, which `gridpoints gridpoint` shows and a forecast does not turn on |
| Forecast and hourly forecast | `forecastGenerator` | Otherwise accounted for | it chooses the title and the table shape, which is how it is shown |
| Forecast and hourly forecast | `generatedAt` | Otherwise accounted for | when NOAA rendered the text; updateTime is when the data behind it changed |
| Forecast and hourly forecast | `geometry` | Otherwise accounted for | the polygon of the grid cell's four corners, the same cell the request named |
| Forecast and hourly forecast | `periods` | Shown | — |
| Forecast and hourly forecast | `properties` | Otherwise accounted for | the forecast itself; its keys are accounted for one by one |
| Forecast and hourly forecast | `type` | Otherwise accounted for | always Feature |
| Forecast and hourly forecast | `units` | Otherwise accounted for | the echoed request parameter; the feature flags this crate always sends make it inert, and --units decides what is shown |
| Forecast and hourly forecast | `updateTime` | Shown | — |
| Forecast and hourly forecast | `validTimes` | Shown | — |
| Gridpoint | `@id` | Otherwise accounted for | the grid cell's own URL, as the envelope id already is |
| Gridpoint | `@type` | Otherwise accounted for | always wx:Gridpoint |
| Gridpoint | `apparentTemperature` | Shown | — |
| Gridpoint | `atmosphericDispersionIndex` | Shown | — |
| Gridpoint | `ceilingHeight` | Shown | — |
| Gridpoint | `davisStabilityIndex` | Shown | — |
| Gridpoint | `dewpoint` | Shown | — |
| Gridpoint | `dispersionIndex` | Shown | — |
| Gridpoint | `elevation` | Shown | — |
| Gridpoint | `forecastOffice` | Otherwise accounted for | shown as the subtitle |
| Gridpoint | `geometry` | Otherwise accounted for | the polygon of the cell's four corners; the grid cell names it in six characters |
| Gridpoint | `grasslandFireDangerIndex` | Shown | — |
| Gridpoint | `gridId` | Shown | — |
| Gridpoint | `gridX` | Shown | — |
| Gridpoint | `gridY` | Shown | — |
| Gridpoint | `hainesIndex` | Shown | — |
| Gridpoint | `hazards` | Shown | — |
| Gridpoint | `heatIndex` | Shown | — |
| Gridpoint | `heatRisk` | Shown | — |
| Gridpoint | `iceAccumulation` | Shown | — |
| Gridpoint | `id` | Otherwise accounted for | the grid cell's own URL; the grid cell fact addresses it again |
| Gridpoint | `lightningActivityLevel` | Shown | — |
| Gridpoint | `lowVisibilityOccurrenceRiskIndex` | Shown | — |
| Gridpoint | `maxTemperature` | Shown | — |
| Gridpoint | `minTemperature` | Shown | — |
| Gridpoint | `mixingHeight` | Shown | — |
| Gridpoint | `potentialOf15mphWinds` | Shown | — |
| Gridpoint | `potentialOf20mphWindGusts` | Shown | — |
| Gridpoint | `potentialOf25mphWinds` | Shown | — |
| Gridpoint | `potentialOf30mphWindGusts` | Shown | — |
| Gridpoint | `potentialOf35mphWinds` | Shown | — |
| Gridpoint | `potentialOf40mphWindGusts` | Shown | — |
| Gridpoint | `potentialOf45mphWinds` | Shown | — |
| Gridpoint | `potentialOf50mphWindGusts` | Shown | — |
| Gridpoint | `potentialOf60mphWindGusts` | Shown | — |
| Gridpoint | `pressure` | Shown | — |
| Gridpoint | `primarySwellDirection` | Shown | — |
| Gridpoint | `primarySwellHeight` | Shown | — |
| Gridpoint | `probabilityOfHurricaneWinds` | Shown | — |
| Gridpoint | `probabilityOfPrecipitation` | Shown | — |
| Gridpoint | `probabilityOfThunder` | Shown | — |
| Gridpoint | `probabilityOfTropicalStormWinds` | Shown | — |
| Gridpoint | `properties` | Otherwise accounted for | the gridpoint itself; its keys are accounted for one by one |
| Gridpoint | `quantitativePrecipitation` | Shown | — |
| Gridpoint | `redFlagThreatIndex` | Shown | — |
| Gridpoint | `relativeHumidity` | Shown | — |
| Gridpoint | `secondarySwellDirection` | Shown | — |
| Gridpoint | `secondarySwellHeight` | Shown | — |
| Gridpoint | `skyCover` | Shown | — |
| Gridpoint | `snowLevel` | Shown | — |
| Gridpoint | `snowfallAmount` | Shown | — |
| Gridpoint | `stability` | Shown | — |
| Gridpoint | `temperature` | Shown | — |
| Gridpoint | `transportWindDirection` | Shown | — |
| Gridpoint | `transportWindSpeed` | Shown | — |
| Gridpoint | `twentyFootWindDirection` | Shown | — |
| Gridpoint | `twentyFootWindSpeed` | Shown | — |
| Gridpoint | `type` | Otherwise accounted for | always Feature |
| Gridpoint | `updateTime` | Shown | — |
| Gridpoint | `validTimes` | Shown | — |
| Gridpoint | `visibility` | Shown | — |
| Gridpoint | `waveDirection` | Shown | — |
| Gridpoint | `waveHeight` | Shown | — |
| Gridpoint | `wavePeriod` | Shown | — |
| Gridpoint | `wavePeriod2` | Shown | — |
| Gridpoint | `weather` | Shown | — |
| Gridpoint | `wetBulbGlobeTemperature` | Shown | — |
| Gridpoint | `windChill` | Shown | — |
| Gridpoint | `windDirection` | Shown | — |
| Gridpoint | `windGust` | Shown | — |
| Gridpoint | `windSpeed` | Shown | — |
| Gridpoint | `windWaveHeight` | Shown | — |

<!-- END GENERATED SHOWN/OMITTED -->
