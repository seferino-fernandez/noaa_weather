# Stations

Station ids are 3 to 16 letters or digits and are upper-cased before the request. Time flags (`--start`, `--end`, `--time`, `--issued`) accept an RFC 3339 timestamp or a relative age such as `6h`, `30m`, or `2d`, resolved when the command starts. Malformed values are usage errors (exit code 2).

Get station metadata:

```sh
noaa-weather stations metadata --id <ID>
```

List stations, optionally filtering by ID or state/marine area. `--cursor` accepts the opaque pagination cursor from a previous page's `pagination.next` value in `--json` output.

```sh
noaa-weather stations list [--id <ID1,ID2...>] [--state <ST1,ST2...>] [--limit <1-500>] [--cursor <CURSOR>]
```

Get the latest observation for a station.

```sh
noaa-weather stations latest-observation --station-id <ID> [--require-quality-controlled]
```

List historical observations for a station. `--cursor` accepts the opaque pagination cursor from a previous page's `pagination.next` value in `--json` output.

```sh
noaa-weather stations observations --station-id <ID> [--start <TIME>] [--end <TIME>] [--limit <1-500>] [--cursor <CURSOR>]
noaa-weather stations observations --station-id KPHX --start 6h --end 1h
```

Get a specific observation by time.

```sh
noaa-weather stations observation --station-id <ID> --time <TIME>
```

Get all available Terminal Aerodrome Forecasts (TAFs) for a station.

```sh
noaa-weather stations terminal-aerodrome-forecasts --station-id <ID>
```

Get a specific Terminal Aerodrome Forecast (TAF) for a station by its issue time.

```sh
noaa-weather stations terminal-aerodrome-forecast --station-id <ID> --issued <TIME>
noaa-weather stations terminal-aerodrome-forecast --station-id KPHX --issued 2026-08-30T22:54:00Z
```

NOAA addresses a TAF by its UTC issue date and `HHMM` minute, which are the final two path segments of an `id` returned by `terminal-aerodrome-forecasts`; `--issued` takes the same instant as one timestamp and seconds are dropped. The human table presents normalized forecast meaning: report state and validity, base/change semantics, CAVOK, wind, visibility, exact weather code plus description, cloud layers and convective types, vertical visibility, and temperature extrema. Omitted values in a change group display as unchanged; IWXXM nil reasons display as unavailable or no significant conditions instead of being collapsed into `N/A`.

Use `--json` with either command for pretty JSON. A specific TAF serializes to the same semantic model used by the table, without IWXXM namespaces or XML wrapper fields.

<!-- BEGIN GENERATED SHOWN/OMITTED -->

## Human-summary property coverage

The table is generated from the summary contracts. `Shown` properties appear in keyed human-summary content; `Otherwise accounted for` properties are deliberately handled without a keyed table or fact.

| Response | Property | Treatment | Reason |
| :--- | :--- | :--- | :--- |
| Decoded Terminal Aerodrome Forecast | `aerodrome` | Shown | — |
| Decoded Terminal Aerodrome Forecast | `bulletinIdentifier` | Shown | — |
| Decoded Terminal Aerodrome Forecast | `issuedAt` | Shown | — |
| Decoded Terminal Aerodrome Forecast | `report` | Shown | — |
| Decoded Terminal Aerodrome Forecast | `reportMetadata` | Shown | — |
| Latest or specific observation | `@id` | Otherwise accounted for | the timestamp and station identify the observation |
| Latest or specific observation | `@type` | Otherwise accounted for | always wx:ObservationStation |
| Latest or specific observation | `barometricPressure` | Shown | — |
| Latest or specific observation | `cloudLayers` | Shown | — |
| Latest or specific observation | `dewpoint` | Shown | — |
| Latest or specific observation | `elevation` | Shown | — |
| Latest or specific observation | `geometry` | Otherwise accounted for | station metadata provides the fixed station coordinates |
| Latest or specific observation | `heatIndex` | Shown | — |
| Latest or specific observation | `icon` | Otherwise accounted for | the text description carries the same condition meaning |
| Latest or specific observation | `id` | Otherwise accounted for | the observation URL duplicates the property URL |
| Latest or specific observation | `maxTemperatureLast24Hours` | Shown | — |
| Latest or specific observation | `minTemperatureLast24Hours` | Shown | — |
| Latest or specific observation | `precipitationLast3Hours` | Shown | — |
| Latest or specific observation | `precipitationLast6Hours` | Shown | — |
| Latest or specific observation | `precipitationLastHour` | Shown | — |
| Latest or specific observation | `presentWeather` | Shown | — |
| Latest or specific observation | `properties` | Otherwise accounted for | the observation; its keys are accounted for one by one |
| Latest or specific observation | `rawMessage` | Otherwise accounted for | decoded measurements and weather are shown instead |
| Latest or specific observation | `relativeHumidity` | Shown | — |
| Latest or specific observation | `seaLevelPressure` | Shown | — |
| Latest or specific observation | `station` | Otherwise accounted for | the station id is shown in the title |
| Latest or specific observation | `stationId` | Otherwise accounted for | shown in the title |
| Latest or specific observation | `stationName` | Otherwise accounted for | shown as the subtitle |
| Latest or specific observation | `temperature` | Shown | — |
| Latest or specific observation | `textDescription` | Shown | — |
| Latest or specific observation | `timestamp` | Shown | — |
| Latest or specific observation | `type` | Otherwise accounted for | always Feature |
| Latest or specific observation | `visibility` | Shown | — |
| Latest or specific observation | `windChill` | Shown | — |
| Latest or specific observation | `windDirection` | Shown | — |
| Latest or specific observation | `windGust` | Shown | — |
| Latest or specific observation | `windSpeed` | Shown | — |
| Observation station | `@id` | Otherwise accounted for | the station identifier is enough for the next command |
| Observation station | `@type` | Otherwise accounted for | always wx:ObservationStation |
| Observation station | `bearing` | Shown | — |
| Observation station | `county` | Shown | — |
| Observation station | `distance` | Shown | — |
| Observation station | `elevation` | Shown | — |
| Observation station | `fireWeatherZone` | Shown | — |
| Observation station | `forecast` | Shown | — |
| Observation station | `geometry` | Shown | — |
| Observation station | `id` | Otherwise accounted for | the envelope URL duplicates the station property URL |
| Observation station | `name` | Shown | — |
| Observation station | `properties` | Otherwise accounted for | the station; its keys are accounted for one by one |
| Observation station | `provider` | Shown | — |
| Observation station | `stationIdentifier` | Shown | — |
| Observation station | `subProvider` | Shown | — |
| Observation station | `timeZone` | Shown | — |
| Observation station | `type` | Otherwise accounted for | always Feature |
| Observation station list | `@id` | Otherwise accounted for | the station identifier is enough for the next command |
| Observation station list | `@type` | Otherwise accounted for | always wx:ObservationStation |
| Observation station list | `bearing` | Shown | — |
| Observation station list | `county` | Shown | — |
| Observation station list | `distance` | Shown | — |
| Observation station list | `elevation` | Shown | — |
| Observation station list | `features` | Otherwise accounted for | each station is one table row |
| Observation station list | `fireWeatherZone` | Shown | — |
| Observation station list | `forecast` | Shown | — |
| Observation station list | `geometry` | Shown | — |
| Observation station list | `id` | Otherwise accounted for | the envelope URL duplicates the station property URL |
| Observation station list | `name` | Shown | — |
| Observation station list | `pagination` | Otherwise accounted for | surfaced as the more-stations note |
| Observation station list | `properties` | Otherwise accounted for | each station's keys are accounted for one by one |
| Observation station list | `provider` | Shown | — |
| Observation station list | `stationIdentifier` | Shown | — |
| Observation station list | `subProvider` | Shown | — |
| Observation station list | `timeZone` | Shown | — |
| Observation station list | `title` | Otherwise accounted for | station collections do not carry a title |
| Observation station list | `type` | Otherwise accounted for | always FeatureCollection or Feature |
| Observation station list | `updated` | Otherwise accounted for | station collections do not carry an update time |
| Station observation history | `@id` | Otherwise accounted for | the timestamp and station identify each observation |
| Station observation history | `@type` | Otherwise accounted for | always wx:ObservationStation |
| Station observation history | `barometricPressure` | Shown | — |
| Station observation history | `cloudLayers` | Otherwise accounted for | omitted to keep the history table scannable |
| Station observation history | `dewpoint` | Shown | — |
| Station observation history | `elevation` | Otherwise accounted for | station metadata provides elevation |
| Station observation history | `features` | Otherwise accounted for | each observation is one table row |
| Station observation history | `geometry` | Otherwise accounted for | station metadata provides the fixed coordinates |
| Station observation history | `heatIndex` | Shown | — |
| Station observation history | `icon` | Otherwise accounted for | not useful in a text history table |
| Station observation history | `id` | Otherwise accounted for | the timestamp and station identify each observation |
| Station observation history | `maxTemperatureLast24Hours` | Otherwise accounted for | duplicated across adjacent history rows |
| Station observation history | `minTemperatureLast24Hours` | Otherwise accounted for | duplicated across adjacent history rows |
| Station observation history | `pagination` | Otherwise accounted for | surfaced as the more-observations note |
| Station observation history | `precipitationLast3Hours` | Otherwise accounted for | omitted to keep the history table scannable |
| Station observation history | `precipitationLast6Hours` | Otherwise accounted for | omitted to keep the history table scannable |
| Station observation history | `precipitationLastHour` | Otherwise accounted for | omitted to keep the history table scannable |
| Station observation history | `presentWeather` | Otherwise accounted for | omitted to keep the history table scannable |
| Station observation history | `properties` | Otherwise accounted for | each observation's keys are accounted for one by one |
| Station observation history | `rawMessage` | Otherwise accounted for | decoded measurements are shown instead |
| Station observation history | `relativeHumidity` | Shown | — |
| Station observation history | `seaLevelPressure` | Shown | — |
| Station observation history | `station` | Otherwise accounted for | the command already names the station |
| Station observation history | `stationId` | Otherwise accounted for | the command already names the station |
| Station observation history | `stationName` | Otherwise accounted for | the command already names the station |
| Station observation history | `temperature` | Shown | — |
| Station observation history | `textDescription` | Otherwise accounted for | omitted to keep the history table scannable |
| Station observation history | `timestamp` | Shown | — |
| Station observation history | `title` | Otherwise accounted for | observation collections do not carry a title |
| Station observation history | `type` | Otherwise accounted for | always FeatureCollection or Feature |
| Station observation history | `updated` | Otherwise accounted for | each observation carries its own timestamp |
| Station observation history | `visibility` | Shown | — |
| Station observation history | `windChill` | Shown | — |
| Station observation history | `windDirection` | Shown | — |
| Station observation history | `windGust` | Shown | — |
| Station observation history | `windSpeed` | Shown | — |
| Terminal Aerodrome Forecast list | `@graph` | Otherwise accounted for | each forecast is one table row |
| Terminal Aerodrome Forecast list | `end` | Shown | — |
| Terminal Aerodrome Forecast list | `geometry` | Shown | — |
| Terminal Aerodrome Forecast list | `id` | Shown | — |
| Terminal Aerodrome Forecast list | `issueTime` | Shown | — |
| Terminal Aerodrome Forecast list | `location` | Shown | — |
| Terminal Aerodrome Forecast list | `start` | Shown | — |
| Zone observation list | `@id` | Otherwise accounted for | the timestamp and station identify each observation |
| Zone observation list | `@type` | Otherwise accounted for | always wx:ObservationStation |
| Zone observation list | `barometricPressure` | Shown | — |
| Zone observation list | `cloudLayers` | Shown | — |
| Zone observation list | `dewpoint` | Shown | — |
| Zone observation list | `elevation` | Otherwise accounted for | station metadata provides elevation |
| Zone observation list | `features` | Otherwise accounted for | each observation is one table row |
| Zone observation list | `geometry` | Otherwise accounted for | station metadata provides the fixed coordinates |
| Zone observation list | `heatIndex` | Otherwise accounted for | not part of this compact zone overview |
| Zone observation list | `icon` | Otherwise accounted for | not useful in a text table |
| Zone observation list | `id` | Otherwise accounted for | the timestamp and station identify each observation |
| Zone observation list | `maxTemperatureLast24Hours` | Otherwise accounted for | not part of current conditions |
| Zone observation list | `minTemperatureLast24Hours` | Otherwise accounted for | not part of current conditions |
| Zone observation list | `pagination` | Otherwise accounted for | this endpoint does not paginate |
| Zone observation list | `precipitationLast3Hours` | Otherwise accounted for | not part of this compact zone overview |
| Zone observation list | `precipitationLast6Hours` | Otherwise accounted for | not part of this compact zone overview |
| Zone observation list | `precipitationLastHour` | Otherwise accounted for | not part of this compact zone overview |
| Zone observation list | `presentWeather` | Shown | — |
| Zone observation list | `properties` | Otherwise accounted for | each observation's keys are accounted for one by one |
| Zone observation list | `rawMessage` | Otherwise accounted for | decoded measurements are shown instead |
| Zone observation list | `relativeHumidity` | Otherwise accounted for | not part of this compact zone overview |
| Zone observation list | `seaLevelPressure` | Shown | — |
| Zone observation list | `station` | Otherwise accounted for | the station id is shown |
| Zone observation list | `stationId` | Shown | — |
| Zone observation list | `stationName` | Shown | — |
| Zone observation list | `temperature` | Shown | — |
| Zone observation list | `textDescription` | Shown | — |
| Zone observation list | `timestamp` | Shown | — |
| Zone observation list | `title` | Otherwise accounted for | zone observation collections do not carry a title |
| Zone observation list | `type` | Otherwise accounted for | always FeatureCollection or Feature |
| Zone observation list | `updated` | Otherwise accounted for | each observation carries its own timestamp |
| Zone observation list | `visibility` | Shown | — |
| Zone observation list | `windChill` | Otherwise accounted for | not part of this compact zone overview |
| Zone observation list | `windDirection` | Shown | — |
| Zone observation list | `windGust` | Shown | — |
| Zone observation list | `windSpeed` | Shown | — |

<!-- END GENERATED SHOWN/OMITTED -->
