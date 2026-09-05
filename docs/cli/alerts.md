# Alerts

The deprecated `active` query flag on the general alerts endpoint is not exposed. Use the dedicated active-alert commands below.

Zone ids (`--zone`, `--zone-id`) and points (`--point <LAT,LON>`) are validated before any request is made; a malformed value is a usage error (exit code 2).

## List Active Alerts

```sh
noaa-weather alerts active [--area <CODE,...>] [--point <LAT,LON>] [--zone <ID,...>] [--severity <LEVEL,...>] ...
```

## Get Active Alerts for a Specific Area (State/Territory or Marine Area)

```sh
noaa-weather alerts area --area <AREA>
```

## Get Count of Active Alerts

```sh
noaa-weather alerts count
```

## Get Active Alerts for a Marine Region

```sh
noaa-weather alerts marine-region --marine-region <REGION>
```

## Get Active Alerts for a Zone

```sh
noaa-weather alerts zone --zone-id <ZONE_ID>
```

## List Alerts, Including Past Ones

`--start` and `--end` accept an RFC 3339 timestamp or a relative age such as `6h`, `30m`, or `2d`, resolved when the command starts. `--cursor` accepts the opaque pagination cursor from a previous page's `pagination.next` value in `--json` output.

```sh
noaa-weather alerts list [--start <TIME>] [--end <TIME>] [--limit <1-500>] [--cursor <CURSOR>] ...
noaa-weather alerts list --start 6h --status actual
```

## Get a Single Alert by ID

```sh
noaa-weather alerts alert --id <ID>
```

## List Available Alert Types

```sh
noaa-weather alerts types
```

<!-- BEGIN GENERATED SHOWN/OMITTED -->

## Human-summary property coverage

The table is generated from the summary contracts. `Shown` properties appear in keyed human-summary content; `Otherwise accounted for` properties are deliberately handled without a keyed table or fact.

| Response | Property | Treatment | Reason |
| :--- | :--- | :--- | :--- |
| Active alert count | `areas` | Shown | — |
| Active alert count | `land` | Shown | — |
| Active alert count | `marine` | Shown | — |
| Active alert count | `regions` | Shown | — |
| Active alert count | `total` | Shown | — |
| Active alert count | `zones` | Otherwise accounted for | about 2,900 rows; per-zone counts are in --json |
| Active alert list | `@id` | Otherwise accounted for | the alert's own URL; the id is enough to fetch it again |
| Active alert list | `@type` | Otherwise accounted for | always wx:Alert |
| Active alert list | `affectedZones` | Otherwise accounted for | zone ids; areaDesc names the same places |
| Active alert list | `areaDesc` | Shown | — |
| Active alert list | `category` | Otherwise accounted for | almost always Met; the single-alert view shows it |
| Active alert list | `certainty` | Otherwise accounted for | the single-alert view shows it |
| Active alert list | `code` | Otherwise accounted for | always IPAWSv1.0 |
| Active alert list | `description` | Otherwise accounted for | paragraphs of text; the single-alert view shows it |
| Active alert list | `effective` | Shown | — |
| Active alert list | `ends` | Otherwise accounted for | usually null; expires bounds the row and the single-alert view shows ends |
| Active alert list | `event` | Otherwise accounted for | the headline names the event and when it was issued |
| Active alert list | `eventCode` | Otherwise accounted for | SAME and NWS codes for the event; the event name says the same |
| Active alert list | `expires` | Shown | — |
| Active alert list | `features` | Otherwise accounted for | each feature's properties is one table row |
| Active alert list | `geocode` | Otherwise accounted for | UGC codes duplicate affectedZones; SAME codes are FIPS county codes for broadcast receivers |
| Active alert list | `geometry` | Otherwise accounted for | polygon coordinates are unreadable in text; areaDesc names the area |
| Active alert list | `headline` | Shown | — |
| Active alert list | `id` | Shown | — |
| Active alert list | `instruction` | Shown | — |
| Active alert list | `language` | Otherwise accounted for | always en-US |
| Active alert list | `messageType` | Otherwise accounted for | the single-alert view shows it |
| Active alert list | `note` | Otherwise accounted for | usually null; the single-alert view shows it |
| Active alert list | `onset` | Otherwise accounted for | usually equals effective; the single-alert view shows it |
| Active alert list | `pagination` | Otherwise accounted for | surfaced as the 'More alerts available' note |
| Active alert list | `parameters` | Otherwise accounted for | CAP system parameters (VTEC, AWIPS and WMO ids, threat codes); NWSheadline and the hazard values are candidates for future facts |
| Active alert list | `references` | Otherwise accounted for | ids of the alerts this one updates; the current alert is what matters |
| Active alert list | `replacedAt` | Otherwise accounted for | the single-alert view shows it |
| Active alert list | `replacedBy` | Otherwise accounted for | the single-alert view shows it |
| Active alert list | `response` | Otherwise accounted for | the instruction column says what to do |
| Active alert list | `scope` | Otherwise accounted for | always Public for anything the API serves |
| Active alert list | `sender` | Otherwise accounted for | always the NWS webmaster mailbox |
| Active alert list | `senderName` | Shown | — |
| Active alert list | `sent` | Otherwise accounted for | effective is the time that matters and is usually the same |
| Active alert list | `severity` | Shown | — |
| Active alert list | `status` | Otherwise accounted for | list callers filter by status; the single-alert view shows it |
| Active alert list | `title` | Otherwise accounted for | shown as the summary title |
| Active alert list | `type` | Otherwise accounted for | always FeatureCollection |
| Active alert list | `updated` | Otherwise accounted for | when NOAA built the page; the alerts carry their own times |
| Active alert list | `urgency` | Otherwise accounted for | the single-alert view shows it |
| Active alert list | `web` | Otherwise accounted for | always the generic weather.gov home page |
| Alert | `@id` | Otherwise accounted for | the alert's own URL; the id is enough to fetch it again |
| Alert | `@type` | Otherwise accounted for | always wx:Alert |
| Alert | `affectedZones` | Shown | — |
| Alert | `areaDesc` | Shown | — |
| Alert | `category` | Shown | — |
| Alert | `certainty` | Shown | — |
| Alert | `code` | Otherwise accounted for | always IPAWSv1.0 |
| Alert | `description` | Shown | — |
| Alert | `effective` | Shown | — |
| Alert | `ends` | Shown | — |
| Alert | `event` | Otherwise accounted for | shown as the title |
| Alert | `eventCode` | Otherwise accounted for | SAME and NWS codes for the event; the event name says the same |
| Alert | `expires` | Shown | — |
| Alert | `geocode` | Otherwise accounted for | UGC codes duplicate affectedZones; SAME codes are FIPS county codes for broadcast receivers |
| Alert | `geometry` | Otherwise accounted for | polygon coordinates are unreadable in text; areaDesc names the area |
| Alert | `headline` | Otherwise accounted for | shown as the subtitle |
| Alert | `id` | Shown | — |
| Alert | `instruction` | Shown | — |
| Alert | `language` | Otherwise accounted for | always en-US |
| Alert | `messageType` | Shown | — |
| Alert | `note` | Shown | — |
| Alert | `onset` | Shown | — |
| Alert | `parameters` | Otherwise accounted for | CAP system parameters (VTEC, AWIPS and WMO ids, threat codes); NWSheadline and the hazard values are candidates for future facts |
| Alert | `properties` | Otherwise accounted for | the alert itself; its keys are accounted for one by one |
| Alert | `references` | Otherwise accounted for | ids of the alerts this one updates; the current alert is what matters |
| Alert | `replacedAt` | Shown | — |
| Alert | `replacedBy` | Shown | — |
| Alert | `response` | Shown | — |
| Alert | `scope` | Otherwise accounted for | always Public for anything the API serves |
| Alert | `sender` | Otherwise accounted for | always the NWS webmaster mailbox |
| Alert | `senderName` | Shown | — |
| Alert | `sent` | Shown | — |
| Alert | `severity` | Shown | — |
| Alert | `status` | Shown | — |
| Alert | `type` | Otherwise accounted for | always Feature |
| Alert | `urgency` | Shown | — |
| Alert | `web` | Otherwise accounted for | always the generic weather.gov home page |
| Alert event types | `eventTypes` | Shown | — |

<!-- END GENERATED SHOWN/OMITTED -->
