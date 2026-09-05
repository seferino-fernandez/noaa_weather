# Aviation

CWSU and ATSU identifiers are validated before any request is made (3 to 4 letters or digits); a malformed value is a usage error (exit code 2).

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

<!-- BEGIN GENERATED SHOWN/OMITTED -->

## Human-summary property coverage

The table is generated from the summary contracts. `Shown` properties appear in keyed human-summary content; `Otherwise accounted for` properties are deliberately handled without a keyed table or fact.

| Response | Property | Treatment | Reason |
| :--- | :--- | :--- | :--- |
| Center Weather Advisory | `cwsu` | Shown | — |
| Center Weather Advisory | `end` | Shown | — |
| Center Weather Advisory | `geometry` | Otherwise accounted for | polygon coordinates are not useful in a text summary |
| Center Weather Advisory | `id` | Shown | — |
| Center Weather Advisory | `issueTime` | Shown | — |
| Center Weather Advisory | `observedProperty` | Shown | — |
| Center Weather Advisory | `properties` | Otherwise accounted for | the advisory; its keys are accounted for one by one |
| Center Weather Advisory | `sequence` | Shown | — |
| Center Weather Advisory | `start` | Shown | — |
| Center Weather Advisory | `text` | Shown | — |
| Center Weather Advisory | `type` | Otherwise accounted for | always Feature |
| Center Weather Advisory list | `cwsu` | Shown | — |
| Center Weather Advisory list | `end` | Shown | — |
| Center Weather Advisory list | `features` | Otherwise accounted for | each advisory is one table row |
| Center Weather Advisory list | `geometry` | Otherwise accounted for | polygon coordinates are not useful in a text summary |
| Center Weather Advisory list | `id` | Shown | — |
| Center Weather Advisory list | `issueTime` | Shown | — |
| Center Weather Advisory list | `observedProperty` | Shown | — |
| Center Weather Advisory list | `pagination` | Otherwise accounted for | surfaced as the more-advisories note |
| Center Weather Advisory list | `sequence` | Shown | — |
| Center Weather Advisory list | `start` | Shown | — |
| Center Weather Advisory list | `text` | Shown | — |
| Center Weather Advisory list | `title` | Otherwise accounted for | shown as the summary title |
| Center Weather Advisory list | `type` | Otherwise accounted for | always FeatureCollection or Feature |
| Center Weather Advisory list | `updated` | Otherwise accounted for | the products carry their own issue times |
| Center Weather Service Unit | `@context` | Otherwise accounted for | fixed JSON-LD vocabulary metadata |
| Center Weather Service Unit | `@id` | Otherwise accounted for | the API URL; the CWSU id identifies the same office |
| Center Weather Service Unit | `@type` | Otherwise accounted for | always GovernmentOrganization |
| Center Weather Service Unit | `address` | Shown | — |
| Center Weather Service Unit | `addressLocality` | Shown | — |
| Center Weather Service Unit | `addressRegion` | Shown | — |
| Center Weather Service Unit | `email` | Shown | — |
| Center Weather Service Unit | `faxNumber` | Shown | — |
| Center Weather Service Unit | `id` | Shown | — |
| Center Weather Service Unit | `name` | Otherwise accounted for | shown as the summary title |
| Center Weather Service Unit | `nwsRegion` | Shown | — |
| Center Weather Service Unit | `postalCode` | Shown | — |
| Center Weather Service Unit | `sameAs` | Shown | — |
| Center Weather Service Unit | `streetAddress` | Shown | — |
| Center Weather Service Unit | `telephone` | Shown | — |
| SIGMET or AIRMET | `atsu` | Shown | — |
| SIGMET or AIRMET | `end` | Shown | — |
| SIGMET or AIRMET | `fir` | Shown | — |
| SIGMET or AIRMET | `geometry` | Otherwise accounted for | polygon coordinates are not useful in a text summary |
| SIGMET or AIRMET | `id` | Shown | — |
| SIGMET or AIRMET | `issueTime` | Shown | — |
| SIGMET or AIRMET | `phenomenon` | Shown | — |
| SIGMET or AIRMET | `properties` | Otherwise accounted for | the product; its keys are accounted for one by one |
| SIGMET or AIRMET | `sequence` | Shown | — |
| SIGMET or AIRMET | `start` | Shown | — |
| SIGMET or AIRMET | `type` | Otherwise accounted for | always Feature |
| SIGMET or AIRMET list | `atsu` | Shown | — |
| SIGMET or AIRMET list | `end` | Shown | — |
| SIGMET or AIRMET list | `features` | Otherwise accounted for | each product is one table row |
| SIGMET or AIRMET list | `fir` | Shown | — |
| SIGMET or AIRMET list | `geometry` | Otherwise accounted for | polygon coordinates are not useful in a text summary |
| SIGMET or AIRMET list | `id` | Shown | — |
| SIGMET or AIRMET list | `issueTime` | Shown | — |
| SIGMET or AIRMET list | `pagination` | Otherwise accounted for | surfaced as the more-products note |
| SIGMET or AIRMET list | `phenomenon` | Shown | — |
| SIGMET or AIRMET list | `sequence` | Shown | — |
| SIGMET or AIRMET list | `start` | Shown | — |
| SIGMET or AIRMET list | `title` | Otherwise accounted for | shown as the summary title |
| SIGMET or AIRMET list | `type` | Otherwise accounted for | always FeatureCollection or Feature |
| SIGMET or AIRMET list | `updated` | Otherwise accounted for | the products carry their own issue times |

<!-- END GENERATED SHOWN/OMITTED -->
