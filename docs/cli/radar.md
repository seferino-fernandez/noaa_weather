# Radar

Interval flags (`--published`, `--arrived`, `--created`, `--time`, `--interval`) take an ISO 8601 time interval in any of its four forms: `start/end`, `start/duration`, `duration/end`, or a bare `duration` (for example `2026-08-30T00:00:00Z/PT1H` or `PT1H`). Radar station ids are four or five letters or digits (for example `KFSX` or the profiler `HWPA2`). Malformed values are usage errors (exit code 2).

## SPGDS telemetry

Get radar SPGDS telemetry, optionally restricted by a publication interval:

```sh
noaa-weather radar spgds [--published <INTERVAL>]
```

## Data queues

Get queue data for `rds` or `tds`. The limit must be between 1 and 50,000 and defaults to 10.

```sh
noaa-weather radar data-queue --host <rds|tds> [--limit <LIMIT>]
```

Additional queue filters include `--arrived`, `--created`, `--published` (intervals), `--station` (radar station id), `--type`, `--feed`, and `--resolution`.

## Servers

```sh
noaa-weather radar server --id <ID> [--reporting-host <HOST>]
noaa-weather radar servers [--reporting-host <HOST>]
```

## Stations and alarms

```sh
noaa-weather radar station --station-id <ID> [--reporting-host <HOST>] [--host <rds|tds>]
noaa-weather radar station-alarms --station-id <ID>
noaa-weather radar stations [--station-type <TYPE>...] [--reporting-host <HOST>] [--host <rds|tds>]
```

## Wind profilers

```sh
noaa-weather radar wind-profiler --id <ID> [--time <INTERVAL>] [--interval <INTERVAL>]
```

Typed radar results render as tables by default and support the global `--json` option. Wind-profiler raw JSON is not summarized because it has no stable typed response model; that command always emits pretty JSON, and `--json` is accepted but redundant there.

<!-- BEGIN GENERATED SHOWN/OMITTED -->

## Human-summary property coverage

The table is generated from the summary contracts. `Shown` properties appear in keyed human-summary content; `Otherwise accounted for` properties are deliberately handled without a keyed table or fact.

| Response | Property | Treatment | Reason |
| :--- | :--- | :--- | :--- |
| Radar SPGDS telemetry | `@graph` | Otherwise accounted for | each SPGDS host is one table row |
| Radar SPGDS telemetry | `@type` | Otherwise accounted for | always an SPGDS telemetry record |
| Radar SPGDS telemetry | `appRunning` | Shown | — |
| Radar SPGDS telemetry | `connectQ` | Shown | — |
| Radar SPGDS telemetry | `conns` | Shown | — |
| Radar SPGDS telemetry | `connsValid` | Otherwise accounted for | low-level epoch-second diagnostic |
| Radar SPGDS telemetry | `dataflow` | Shown | — |
| Radar SPGDS telemetry | `id` | Shown | — |
| Radar SPGDS telemetry | `in` | Shown | — |
| Radar SPGDS telemetry | `inDateTime` | Otherwise accounted for | low-level epoch-second diagnostic |
| Radar SPGDS telemetry | `inValid` | Otherwise accounted for | low-level epoch-second diagnostic |
| Radar SPGDS telemetry | `ldm` | Otherwise accounted for | connection count is folded into the row |
| Radar SPGDS telemetry | `ldmPingState` | Otherwise accounted for | per-gateway diagnostic available in JSON |
| Radar SPGDS telemetry | `ldmPingStateSince` | Otherwise accounted for | per-gateway diagnostic available in JSON |
| Radar SPGDS telemetry | `ldmPingStateValid` | Otherwise accounted for | per-gateway diagnostic available in JSON |
| Radar SPGDS telemetry | `out` | Shown | — |
| Radar SPGDS telemetry | `outDateTime` | Otherwise accounted for | low-level epoch-second diagnostic |
| Radar SPGDS telemetry | `outValid` | Otherwise accounted for | low-level epoch-second diagnostic |
| Radar SPGDS telemetry | `pctUsed` | Shown | — |
| Radar SPGDS telemetry | `pctUsedValid` | Otherwise accounted for | low-level epoch-second diagnostic |
| Radar SPGDS telemetry | `secondHD` | Otherwise accounted for | disk utilization is folded into the row |
| Radar SPGDS telemetry | `spg` | Shown | — |
| Radar SPGDS telemetry | `spgdsUpSince` | Otherwise accounted for | low-level epoch-second diagnostic |
| Radar SPGDS telemetry | `state` | Otherwise accounted for | shown for the three primary status categories |
| Radar SPGDS telemetry | `stateSince` | Otherwise accounted for | low-level epoch-second diagnostic |
| Radar SPGDS telemetry | `stateValid` | Otherwise accounted for | low-level epoch-second diagnostic |
| Radar SPGDS telemetry | `swimDataState` | Otherwise accounted for | per-gateway diagnostic available in JSON |
| Radar SPGDS telemetry | `swimDataStateSince` | Otherwise accounted for | per-gateway diagnostic available in JSON |
| Radar SPGDS telemetry | `swimDataStateValid` | Otherwise accounted for | per-gateway diagnostic available in JSON |
| Radar SPGDS telemetry | `throughput` | Otherwise accounted for | inbound and outbound values are folded into the row |
| Radar SPGDS telemetry | `timestamp` | Shown | — |
| Radar SPGDS telemetry | `upSince` | Otherwise accounted for | low-level epoch-second diagnostic |
| Radar SPGDS telemetry | `upSinceValid` | Otherwise accounted for | low-level epoch-second diagnostic |
| Radar data queue | `@graph` | Otherwise accounted for | each queued product is one table row |
| Radar data queue | `@id` | Otherwise accounted for | identifies the queried queue |
| Radar data queue | `@type` | Otherwise accounted for | always a radar-queue item |
| Radar data queue | `arrivalTime` | Shown | — |
| Radar data queue | `creationTime` | Shown | — |
| Radar data queue | `feed` | Shown | — |
| Radar data queue | `host` | Shown | — |
| Radar data queue | `resolutionVersion` | Shown | — |
| Radar data queue | `sequenceNumber` | Shown | — |
| Radar data queue | `size` | Shown | — |
| Radar data queue | `stationId` | Shown | — |
| Radar data queue | `type` | Shown | — |
| Radar server list | `@graph` | Otherwise accounted for | each server is one table row |
| Radar server list | `@id` | Otherwise accounted for | the server id identifies the same resource |
| Radar server list | `@type` | Otherwise accounted for | always a radar-server resource |
| Radar server list | `active` | Shown | — |
| Radar server list | `aggregate` | Otherwise accounted for | available from the single-server command |
| Radar server list | `collectionTime` | Shown | — |
| Radar server list | `command` | Otherwise accounted for | available from the single-server command |
| Radar server list | `count` | Shown | — |
| Radar server list | `hardware` | Otherwise accounted for | load1 is folded into the row |
| Radar server list | `id` | Shown | — |
| Radar server list | `ingestHost` | Otherwise accounted for | available from the single-server command |
| Radar server list | `ldm` | Shown | active state and count are folded into the row |
| Radar server list | `load1` | Shown | — |
| Radar server list | `locked` | Otherwise accounted for | available from the single-server command |
| Radar server list | `network` | Otherwise accounted for | available from the single-server command |
| Radar server list | `ping` | Otherwise accounted for | available from the single-server command |
| Radar server list | `primary` | Shown | — |
| Radar server list | `radarNetworkUp` | Shown | — |
| Radar server list | `reportingHost` | Shown | — |
| Radar server list | `type` | Shown | — |
| Radar server telemetry | `@id` | Otherwise accounted for | the server id identifies the same resource |
| Radar server telemetry | `@type` | Otherwise accounted for | always a radar-server resource |
| Radar server telemetry | `active` | Shown | — |
| Radar server telemetry | `aggregate` | Shown | — |
| Radar server telemetry | `client` | Shown | — |
| Radar server telemetry | `collectionTime` | Shown | — |
| Radar server telemetry | `command` | Otherwise accounted for | expanded as command-status facts when present |
| Radar server telemetry | `count` | Shown | — |
| Radar server telemetry | `cpuIdle` | Shown | — |
| Radar server telemetry | `disk` | Shown | — |
| Radar server telemetry | `eth0` | Otherwise accounted for | shown as a network table row |
| Radar server telemetry | `eth1` | Otherwise accounted for | shown as a network table row |
| Radar server telemetry | `hardware` | Otherwise accounted for | expanded as hardware facts |
| Radar server telemetry | `id` | Shown | — |
| Radar server telemetry | `ingestHost` | Shown | — |
| Radar server telemetry | `interface` | Shown | — |
| Radar server telemetry | `ioUtilization` | Shown | — |
| Radar server telemetry | `lastExecuted` | Shown | — |
| Radar server telemetry | `lastExecutedTime` | Shown | — |
| Radar server telemetry | `lastNexradDataTime` | Shown | — |
| Radar server telemetry | `lastReceived` | Shown | — |
| Radar server telemetry | `lastReceivedTime` | Shown | — |
| Radar server telemetry | `latestProduct` | Shown | — |
| Radar server telemetry | `ldm` | Shown | expanded as Local Data Manager facts |
| Radar server telemetry | `load1` | Shown | — |
| Radar server telemetry | `load15` | Shown | — |
| Radar server telemetry | `load5` | Shown | — |
| Radar server telemetry | `locked` | Shown | — |
| Radar server telemetry | `memory` | Shown | — |
| Radar server telemetry | `misc` | Shown | — |
| Radar server telemetry | `network` | Otherwise accounted for | expanded as a network table |
| Radar server telemetry | `oldestProduct` | Shown | — |
| Radar server telemetry | `ping` | Otherwise accounted for | expanded as ping-status facts |
| Radar server telemetry | `primary` | Shown | — |
| Radar server telemetry | `radar` | Shown | — |
| Radar server telemetry | `radarNetworkUp` | Shown | — |
| Radar server telemetry | `recvDropped` | Otherwise accounted for | low-level interface counter |
| Radar server telemetry | `recvError` | Shown | — |
| Radar server telemetry | `recvNoError` | Shown | — |
| Radar server telemetry | `recvOverrun` | Otherwise accounted for | low-level interface counter |
| Radar server telemetry | `reportingHost` | Shown | — |
| Radar server telemetry | `server` | Shown | — |
| Radar server telemetry | `storageSize` | Shown | — |
| Radar server telemetry | `targets` | Otherwise accounted for | each target category is summarized as up over total |
| Radar server telemetry | `timestamp` | Shown | shown within each telemetry section |
| Radar server telemetry | `transDropped` | Otherwise accounted for | low-level interface counter |
| Radar server telemetry | `transError` | Shown | — |
| Radar server telemetry | `transNoError` | Shown | — |
| Radar server telemetry | `transOverrun` | Otherwise accounted for | low-level interface counter |
| Radar server telemetry | `type` | Shown | — |
| Radar server telemetry | `uptime` | Shown | — |
| Radar station alarms | `@graph` | Shown | each alarm is one table row |
| Radar station alarms | `@id` | Otherwise accounted for | identifies the queried alarm collection |
| Radar station alarms | `@type` | Otherwise accounted for | always a radar-station alarm |
| Radar station alarms | `activeChannel` | Shown | — |
| Radar station alarms | `message` | Shown | — |
| Radar station alarms | `stationId` | Shown | — |
| Radar station alarms | `status` | Shown | — |
| Radar station alarms | `timestamp` | Shown | — |
| Radar station list | `@id` | Otherwise accounted for | the station id identifies the same resource |
| Radar station list | `@type` | Otherwise accounted for | always a radar-station resource |
| Radar station list | `elevation` | Shown | — |
| Radar station list | `features` | Otherwise accounted for | each station is one table row |
| Radar station list | `geometry` | Otherwise accounted for | available from the single-station command |
| Radar station list | `id` | Shown | — |
| Radar station list | `latency` | Otherwise accounted for | available from the single-station command |
| Radar station list | `name` | Shown | — |
| Radar station list | `properties` | Otherwise accounted for | station properties supply each row |
| Radar station list | `rda` | Otherwise accounted for | available from the single-station command |
| Radar station list | `stationType` | Shown | — |
| Radar station list | `timeZone` | Shown | — |
| Radar station list | `type` | Otherwise accounted for | GeoJSON envelope type |
| Radar station list | `unitCode` | Otherwise accounted for | folded into elevation |
| Radar station list | `value` | Otherwise accounted for | folded into elevation |
| Radar station telemetry | `@id` | Otherwise accounted for | the station id identifies the same resource |
| Radar station telemetry | `@type` | Otherwise accounted for | always a radar-station resource |
| Radar station telemetry | `adaptation` | Otherwise accounted for | expanded as adaptation highlights or an empty state |
| Radar station telemetry | `alarmSummary` | Shown | — |
| Radar station telemetry | `ameHorzizontalTestSignalPower` | Otherwise accounted for | low-level adaptation calibration |
| Radar station telemetry | `ameNoiseSourceHorizontalExcessNoiseRatio` | Otherwise accounted for | low-level adaptation calibration |
| Radar station telemetry | `antennaGainIncludingRadome` | Otherwise accounted for | low-level adaptation calibration |
| Radar station telemetry | `average` | Shown | — |
| Radar station telemetry | `averageTransmitterPower` | Otherwise accounted for | low-level calibration diagnostic |
| Radar station telemetry | `azimuthEncoderLight` | Otherwise accounted for | low-level encoder diagnostic |
| Radar station telemetry | `buildNumber` | Shown | — |
| Radar station telemetry | `cohoPowerAtA1J4` | Otherwise accounted for | low-level adaptation calibration |
| Radar station telemetry | `commandChannel` | Shown | — |
| Radar station telemetry | `controlStatus` | Shown | — |
| Radar station telemetry | `current` | Shown | — |
| Radar station telemetry | `dynamicRange` | Otherwise accounted for | low-level calibration diagnostic |
| Radar station telemetry | `elevation` | Shown | — |
| Radar station telemetry | `elevationEncoderLight` | Otherwise accounted for | low-level encoder diagnostic |
| Radar station telemetry | `fuelLevel` | Shown | — |
| Radar station telemetry | `generatorState` | Otherwise accounted for | low-level operational diagnostic |
| Radar station telemetry | `geometry` | Shown | — |
| Radar station telemetry | `horizontalDeltadBZ0` | Otherwise accounted for | low-level calibration diagnostic |
| Radar station telemetry | `horizontalLongPulseNoise` | Otherwise accounted for | low-level calibration diagnostic |
| Radar station telemetry | `horizontalNoiseTemperature` | Otherwise accounted for | low-level calibration diagnostic |
| Radar station telemetry | `horizontalReceiverNoiseLongPulse` | Otherwise accounted for | low-level adaptation calibration |
| Radar station telemetry | `horizontalReceiverNoiseShortPulse` | Otherwise accounted for | low-level adaptation calibration |
| Radar station telemetry | `horizontalShortPulseNoise` | Otherwise accounted for | low-level calibration diagnostic |
| Radar station telemetry | `host` | Shown | — |
| Radar station telemetry | `id` | Shown | — |
| Radar station telemetry | `latency` | Otherwise accounted for | expanded as latency facts |
| Radar station telemetry | `levelTwoLastReceivedTime` | Shown | — |
| Radar station telemetry | `linearity` | Otherwise accounted for | low-level calibration diagnostic |
| Radar station telemetry | `longPulseHorizontaldBZ0` | Otherwise accounted for | low-level calibration diagnostic |
| Radar station telemetry | `longPulseVerticaldBZ0` | Otherwise accounted for | low-level calibration diagnostic |
| Radar station telemetry | `max` | Shown | — |
| Radar station telemetry | `maxLatencyTime` | Shown | — |
| Radar station telemetry | `maxValue` | Otherwise accounted for | folded into each displayed measurement |
| Radar station telemetry | `minValue` | Otherwise accounted for | folded into each displayed measurement |
| Radar station telemetry | `mode` | Shown | — |
| Radar station telemetry | `name` | Shown | — |
| Radar station telemetry | `nl2Path` | Otherwise accounted for | internal Level II routing path |
| Radar station telemetry | `ntp_status` | Shown | — |
| Radar station telemetry | `operabilityStatus` | Shown | — |
| Radar station telemetry | `pathLossA6ArcDetector` | Otherwise accounted for | low-level adaptation calibration |
| Radar station telemetry | `pathLossAT4Attenuator` | Otherwise accounted for | low-level adaptation calibration |
| Radar station telemetry | `pathLossHorzontalIFHeliaxTo4AT17` | Otherwise accounted for | low-level adaptation calibration |
| Radar station telemetry | `pathLossIFDBurstAntiAliasFilter` | Otherwise accounted for | low-level adaptation calibration |
| Radar station telemetry | `pathLossIFDRIFAntiAliasFilter` | Otherwise accounted for | low-level adaptation calibration |
| Radar station telemetry | `pathLossTransmitterCouplerCoupling` | Otherwise accounted for | low-level adaptation calibration |
| Radar station telemetry | `pathLossVerticalIFHeliaxTo4AT16` | Otherwise accounted for | low-level adaptation calibration |
| Radar station telemetry | `pathLossWG02HarmonicFilter` | Otherwise accounted for | low-level adaptation calibration |
| Radar station telemetry | `pathLossWG04Circulator` | Otherwise accounted for | low-level adaptation calibration |
| Radar station telemetry | `pathLossWG06SpectrumFilter` | Otherwise accounted for | low-level adaptation calibration |
| Radar station telemetry | `pathLossWaveguideKlystronToSwitch` | Otherwise accounted for | low-level adaptation calibration |
| Radar station telemetry | `performance` | Otherwise accounted for | expanded as performance highlights or an empty state |
| Radar station telemetry | `performanceCheckTime` | Shown | — |
| Radar station telemetry | `powerSource` | Shown | — |
| Radar station telemetry | `properties` | Otherwise accounted for | nested telemetry is expanded into facts |
| Radar station telemetry | `pulseWidthTransmitterOutputLongPulse` | Otherwise accounted for | low-level adaptation calibration |
| Radar station telemetry | `pulseWidthTransmitterOutputShortPulse` | Otherwise accounted for | low-level adaptation calibration |
| Radar station telemetry | `qualityControl` | Otherwise accounted for | raw measurement quality flag |
| Radar station telemetry | `radomeAirTemperature` | Shown | — |
| Radar station telemetry | `rda` | Otherwise accounted for | expanded as radar data acquisition facts |
| Radar station telemetry | `receiverBias` | Otherwise accounted for | low-level calibration diagnostic |
| Radar station telemetry | `reflectivityCalibrationCorrection` | Otherwise accounted for | low-level calibration diagnostic |
| Radar station telemetry | `reportingHost` | Shown | shown with the RDA and latency sections |
| Radar station telemetry | `resolutionVersion` | Shown | — |
| Radar station telemetry | `shelterTemperature` | Shown | — |
| Radar station telemetry | `shortPulseHorizontaldBZ0` | Otherwise accounted for | low-level calibration diagnostic |
| Radar station telemetry | `shortPulseVerticaldBZ0` | Otherwise accounted for | low-level calibration diagnostic |
| Radar station telemetry | `staloPowerAtA1J2` | Otherwise accounted for | low-level adaptation calibration |
| Radar station telemetry | `stationType` | Shown | — |
| Radar station telemetry | `status` | Shown | — |
| Radar station telemetry | `superResolutionStatus` | Otherwise accounted for | low-level operational diagnostic |
| Radar station telemetry | `timeZone` | Shown | — |
| Radar station telemetry | `timestamp` | Shown | — |
| Radar station telemetry | `transitionalPowerSource` | Otherwise accounted for | low-level power diagnostic |
| Radar station telemetry | `transmitterFrequency` | Shown | — |
| Radar station telemetry | `transmitterImbalance` | Otherwise accounted for | low-level calibration diagnostic |
| Radar station telemetry | `transmitterLeavingAirTemperature` | Otherwise accounted for | low-level transmitter diagnostic |
| Radar station telemetry | `transmitterPeakPower` | Otherwise accounted for | low-level calibration diagnostic |
| Radar station telemetry | `transmitterPowerDataWattsFactor` | Otherwise accounted for | low-level adaptation calibration |
| Radar station telemetry | `transmitterRecycleCount` | Otherwise accounted for | low-level transmitter diagnostic |
| Radar station telemetry | `transmitterSpectrumFilterInstalled` | Shown | — |
| Radar station telemetry | `type` | Otherwise accounted for | GeoJSON envelope type |
| Radar station telemetry | `unitCode` | Otherwise accounted for | folded into each displayed measurement |
| Radar station telemetry | `value` | Otherwise accounted for | folded into each displayed measurement |
| Radar station telemetry | `verticalDeltadBZ0` | Otherwise accounted for | low-level calibration diagnostic |
| Radar station telemetry | `volumeCoveragePattern` | Shown | — |

<!-- END GENERATED SHOWN/OMITTED -->
