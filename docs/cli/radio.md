# Radio

Radio commands are always available. Call signs and county zone ids are validated before any request is made; a malformed value is a usage error (exit code 2).

List transmitter metadata, optionally continuing from the opaque pagination cursor in a previous page's `pagination.next` value in `--json` output:

```sh
noaa-weather radio transmitters [--cursor <CURSOR>]
```

Get one transmitter by call sign:

```sh
noaa-weather radio transmitter <CALL_SIGN>
```

Get transmitters serving a county zone:

```sh
noaa-weather radio zone <ZONE_ID>
```

Get the NOAA Weather Radio broadcast for a transmitter station by call sign:

```sh
noaa-weather radio station <CALL_SIGN>
```

Get the NOAA Weather Radio broadcast for a geographic point, given as one `LAT,LON` value:

```sh
noaa-weather radio point <LAT,LON>
noaa-weather radio point 33.4484,-112.0740
```

Structured transmitter commands use tables by default and support the global `--json` option. Broadcast commands retain their readable transcript output and also support JSON.

<!-- BEGIN GENERATED SHOWN/OMITTED -->

## Human-summary property coverage

The table is generated from the summary contracts. `Shown` properties appear in keyed human-summary content; `Otherwise accounted for` properties are deliberately handled without a keyed table or fact.

| Response | Property | Treatment | Reason |
| :--- | :--- | :--- | :--- |
| Radio broadcast | `@version` | Shown | — |
| Radio broadcast | `@xml:lang` | Shown | — |
| Radio broadcast | `p` | Shown | — |
| Radio transmitter | `@id` | Otherwise accounted for | the call sign identifies the same transmitter |
| Radio transmitter | `@type` | Otherwise accounted for | always wx:Transmitter |
| Radio transmitter | `callSign` | Shown | — |
| Radio transmitter | `counties` | Shown | — |
| Radio transmitter | `sameCodes` | Shown | — |
| Radio transmitter | `setId` | Otherwise accounted for | internal transmitter-dataset revision |
| Radio transmitter | `siteCity` | Shown | — |
| Radio transmitter | `siteName` | Shown | — |
| Radio transmitter | `siteState` | Shown | — |
| Radio transmitter | `transmitterFrequency` | Shown | — |
| Radio transmitter list | `@graph` | Otherwise accounted for | each transmitter is one table row |
| Radio transmitter list | `@id` | Otherwise accounted for | the call sign identifies the same transmitter |
| Radio transmitter list | `@type` | Otherwise accounted for | always wx:Transmitter |
| Radio transmitter list | `callSign` | Shown | — |
| Radio transmitter list | `counties` | Shown | — |
| Radio transmitter list | `pagination` | Otherwise accounted for | surfaced as the more-transmitters note |
| Radio transmitter list | `sameCodes` | Shown | — |
| Radio transmitter list | `setId` | Otherwise accounted for | internal transmitter-dataset revision |
| Radio transmitter list | `siteCity` | Shown | — |
| Radio transmitter list | `siteName` | Shown | — |
| Radio transmitter list | `siteState` | Shown | — |
| Radio transmitter list | `transmitterFrequency` | Shown | — |

<!-- END GENERATED SHOWN/OMITTED -->
