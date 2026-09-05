# Points

Get point metadata for a latitude and longitude, given as one `LAT,LON` value in decimal degrees:

```sh
noaa-weather points metadata <LAT,LON>
noaa-weather points metadata 39.7456,-97.0892
```

Because the pair is one argument, negative longitudes need no `--` separator. Latitude must be within -90..=90 and longitude within -180..=180; values are rounded to four decimals, and a malformed pair is a usage error (exit code 2).

The deprecated `/points/{latitude},{longitude}/stations` operation is not exposed. Resolve point metadata first, then use its gridpoint with `gridpoints stations` or query stations directly.

<!-- BEGIN GENERATED SHOWN/OMITTED -->

## Human-summary property coverage

The table is generated from the summary contracts. `Shown` properties appear in keyed human-summary content; `Otherwise accounted for` properties are deliberately handled without a keyed table or fact.

| Response | Property | Treatment | Reason |
| :--- | :--- | :--- | :--- |
| Point metadata | `@id` | Otherwise accounted for | the point's own URL, as the envelope id already is |
| Point metadata | `@type` | Otherwise accounted for | always wx:Point |
| Point metadata | `astronomicalData` | Otherwise accounted for | sunrise, sunset and the twilights; deferred to a later slice, not judged unimportant |
| Point metadata | `county` | Shown | — |
| Point metadata | `cwa` | Shown | — |
| Point metadata | `fireWeatherZone` | Shown | — |
| Point metadata | `forecast` | Otherwise accounted for | the URL of `gridpoints forecast` for this grid cell |
| Point metadata | `forecastGridData` | Otherwise accounted for | the URL of `gridpoints gridpoint` for this grid cell |
| Point metadata | `forecastHourly` | Otherwise accounted for | the URL of `gridpoints forecast-hourly` for this grid cell |
| Point metadata | `forecastOffice` | Otherwise accounted for | the URL of the office `cwa` names; `offices office` fetches it |
| Point metadata | `forecastZone` | Shown | — |
| Point metadata | `geometry` | Otherwise accounted for | shown as the subtitle |
| Point metadata | `gridId` | Shown | — |
| Point metadata | `gridX` | Shown | — |
| Point metadata | `gridY` | Shown | — |
| Point metadata | `id` | Otherwise accounted for | the point's own URL; the coordinates address it again |
| Point metadata | `nwr` | Otherwise accounted for | NOAA Weather Radio transmitter and SAME code, for a receiver rather than a reader |
| Point metadata | `observationStations` | Otherwise accounted for | the URL of `gridpoints stations` for this grid cell |
| Point metadata | `properties` | Otherwise accounted for | the point itself; its keys are accounted for one by one |
| Point metadata | `radarStation` | Shown | — |
| Point metadata | `relativeLocation` | Otherwise accounted for | shown as the title |
| Point metadata | `timeZone` | Shown | — |
| Point metadata | `type` | Shown | — |

<!-- END GENERATED SHOWN/OMITTED -->
