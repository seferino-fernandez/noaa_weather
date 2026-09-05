//! Human summaries for NOAA radar infrastructure telemetry.

use noaa_weather_client::models::{
    CommandChannel, CommandChannelMode, RadarMeasurement, RadarPosition, RadarQueuesResponse,
    RadarServerTelemetry, RadarServersResponse, RadarSpgdsResponse, RadarStationAlarmsResponse,
    RadarStationTelemetry, RadarStationsResponse,
};

use crate::units::QuantityKind;
use crate::{Align, Column, Fact, Section, Summarize, Summary, SummaryOptions, Value};

fn measurement(
    value: Option<&RadarMeasurement>,
    kind: QuantityKind,
    options: &SummaryOptions,
) -> Value {
    let Some(value) = value else {
        return Value::Missing;
    };
    value.quantity().map_or_else(
        || Value::number(value.value, 2, None),
        |quantity| Value::quantity(&quantity, kind, options),
    )
}

fn optional_timestamp(value: Option<noaa_weather_client::OffsetDateTime>) -> Value {
    value.map_or(Value::Missing, Value::timestamp)
}

fn command_channel(value: Option<&CommandChannel>) -> Value {
    match value {
        Some(CommandChannel::Channel(channel)) => Value::count(u64::from(*channel)),
        Some(CommandChannel::Mode(CommandChannelMode::Single)) => Value::text(Some("Single")),
        Some(CommandChannel::Other(mode)) => Value::text(Some(mode)),
        Some(_) => Value::Invalid,
        None => Value::Missing,
    }
}

fn ping(up: usize, total: usize) -> Value {
    Value::text(Some(&format!("{up}/{total} up")))
}

impl Summarize for RadarStationTelemetry {
    fn summarize(&self, options: &SummaryOptions) -> Summary {
        let station = &self.station;
        let position = match self.position() {
            RadarPosition::Coordinates {
                longitude,
                latitude,
            } => Value::coordinates(latitude, longitude),
            RadarPosition::Missing => Value::Missing,
            RadarPosition::Invalid => Value::Invalid,
            _ => Value::Invalid,
        };
        let mut summary = Summary::new("Radar station")
            .subtitle(format!("{} — {}", station.id, station.name))
            .push(Section::Facts {
                heading: Some("Station information".to_owned()),
                facts: vec![
                    Fact::new(
                        "Station ID",
                        Some("id"),
                        Value::identifier(station.id.to_string()),
                    ),
                    Fact::new("Name", Some("name"), Value::text(Some(&station.name))),
                    Fact::new(
                        "Station Type",
                        Some("stationType"),
                        Value::text(Some(&station.station_type)),
                    ),
                    Fact::new(
                        "Elevation",
                        Some("elevation"),
                        measurement(Some(&station.elevation), QuantityKind::Height, options),
                    ),
                    Fact::new(
                        "Time Zone",
                        Some("timeZone"),
                        Value::text(station.time_zone.iana_name()),
                    ),
                    Fact::new("Location", Some("geometry"), position),
                ],
            });

        let latency = &station.latency;
        summary = summary.push(Section::Facts {
            heading: Some("Latency".to_owned()),
            facts: vec![
                Fact::new(
                    "Current",
                    Some("current"),
                    measurement(latency.current.as_ref(), QuantityKind::Index, options),
                ),
                Fact::new(
                    "Average",
                    Some("average"),
                    measurement(latency.average.as_ref(), QuantityKind::Index, options),
                ),
                Fact::new(
                    "Maximum",
                    Some("max"),
                    measurement(latency.maximum.as_ref(), QuantityKind::Index, options),
                ),
                Fact::new(
                    "Last Level II Product",
                    Some("levelTwoLastReceivedTime"),
                    optional_timestamp(latency.level_two_last_received_time),
                ),
                Fact::new(
                    "Maximum At",
                    Some("maxLatencyTime"),
                    optional_timestamp(latency.max_latency_time),
                ),
                Fact::new(
                    "Reporting Host",
                    Some("reportingHost"),
                    Value::text(latency.reporting_host.as_deref()),
                ),
                Fact::new(
                    "Data Host",
                    Some("host"),
                    Value::text(latency.host.as_deref()),
                ),
            ],
        });

        if let Some(rda) = &station.rda {
            let properties = &rda.properties;
            summary = summary.push(Section::Facts {
                heading: Some("Radar data acquisition".to_owned()),
                facts: vec![
                    Fact::new(
                        "Observed",
                        Some("timestamp"),
                        Value::timestamp(rda.timestamp),
                    ),
                    Fact::new(
                        "Reporting Host",
                        Some("reportingHost"),
                        Value::text(Some(&rda.reporting_host)),
                    ),
                    Fact::new(
                        "Status",
                        Some("status"),
                        Value::text(Some(&properties.status)),
                    ),
                    Fact::new(
                        "Operability",
                        Some("operabilityStatus"),
                        Value::text(Some(&properties.operability_status)),
                    ),
                    Fact::new("Mode", Some("mode"), Value::text(Some(&properties.mode))),
                    Fact::new(
                        "Control",
                        Some("controlStatus"),
                        Value::text(Some(&properties.control_status)),
                    ),
                    Fact::new(
                        "Alarm Summary",
                        Some("alarmSummary"),
                        Value::text(Some(&properties.alarm_summary)),
                    ),
                    Fact::new(
                        "Coverage Pattern",
                        Some("volumeCoveragePattern"),
                        Value::text(Some(&properties.volume_coverage_pattern)),
                    ),
                    Fact::new(
                        "Build",
                        Some("buildNumber"),
                        Value::number(Some(properties.build_number), 1, None),
                    ),
                    Fact::new(
                        "Resolution Version",
                        Some("resolutionVersion"),
                        properties
                            .resolution_version
                            .map_or(Value::Missing, |value| {
                                Value::count(value.unsigned_abs().into())
                            }),
                    ),
                ],
            });
        }

        if let Some(performance) = &station.performance {
            if let Some(properties) = &performance.properties {
                summary = summary.push(Section::Facts {
                    heading: Some("Performance highlights".to_owned()),
                    facts: vec![
                        Fact::new(
                            "Observed",
                            Some("timestamp"),
                            optional_timestamp(performance.timestamp),
                        ),
                        Fact::new(
                            "Performance Check",
                            Some("performanceCheckTime"),
                            optional_timestamp(properties.performance_check_time),
                        ),
                        Fact::new(
                            "NTP Status",
                            Some("ntp_status"),
                            properties.ntp_status.map_or(Value::Missing, |value| {
                                Value::count(value.unsigned_abs().into())
                            }),
                        ),
                        Fact::new(
                            "Command Channel",
                            Some("commandChannel"),
                            command_channel(properties.command_channel.as_ref()),
                        ),
                        Fact::new(
                            "Power Source",
                            Some("powerSource"),
                            Value::text(properties.power_source.as_deref()),
                        ),
                        Fact::new(
                            "Shelter Temperature",
                            Some("shelterTemperature"),
                            measurement(
                                properties.shelter_temperature.as_ref(),
                                QuantityKind::Temperature,
                                options,
                            ),
                        ),
                        Fact::new(
                            "Radome Temperature",
                            Some("radomeAirTemperature"),
                            measurement(
                                properties.radome_air_temperature.as_ref(),
                                QuantityKind::Temperature,
                                options,
                            ),
                        ),
                        Fact::new(
                            "Fuel Level",
                            Some("fuelLevel"),
                            measurement(
                                properties.fuel_level.as_ref(),
                                QuantityKind::Percent,
                                options,
                            ),
                        ),
                    ],
                });
            } else {
                summary = summary.push(Section::Empty {
                    key: Some("performance"),
                    message: "No detailed performance telemetry is available".to_owned(),
                });
            }
        }

        if let Some(adaptation) = &station.adaptation {
            if let Some(properties) = &adaptation.properties {
                summary = summary.push(Section::Facts {
                    heading: Some("Adaptation highlights".to_owned()),
                    facts: vec![
                        Fact::new(
                            "Observed",
                            Some("timestamp"),
                            optional_timestamp(adaptation.timestamp),
                        ),
                        Fact::new(
                            "Transmitter Frequency",
                            Some("transmitterFrequency"),
                            measurement(
                                properties.transmitter_frequency.as_ref(),
                                QuantityKind::Index,
                                options,
                            ),
                        ),
                        Fact::new(
                            "Spectrum Filter Installed",
                            Some("transmitterSpectrumFilterInstalled"),
                            Value::text(
                                properties.transmitter_spectrum_filter_installed.as_deref(),
                            ),
                        ),
                    ],
                });
            } else {
                summary = summary.push(Section::Empty {
                    key: Some("adaptation"),
                    message: "No detailed adaptation telemetry is available".to_owned(),
                });
            }
        }

        summary
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        ("type", "GeoJSON envelope type"),
        ("properties", "station fields are summarized directly"),
        ("@id", "the station id identifies the same resource"),
        ("@type", "always a radar-station resource"),
        ("latency", "expanded as latency facts"),
        ("rda", "expanded as radar data acquisition facts"),
        (
            "performance",
            "expanded as performance highlights or an empty state",
        ),
        (
            "adaptation",
            "expanded as adaptation highlights or an empty state",
        ),
        ("properties", "nested telemetry is expanded into facts"),
        ("nl2Path", "internal Level II routing path"),
        ("generatorState", "low-level operational diagnostic"),
        ("superResolutionStatus", "low-level operational diagnostic"),
        (
            "averageTransmitterPower",
            "low-level calibration diagnostic",
        ),
        (
            "reflectivityCalibrationCorrection",
            "low-level calibration diagnostic",
        ),
        ("reportingHost", "shown with the RDA and latency sections"),
        ("transitionalPowerSource", "low-level power diagnostic"),
        (
            "horizontalShortPulseNoise",
            "low-level calibration diagnostic",
        ),
        ("elevationEncoderLight", "low-level encoder diagnostic"),
        (
            "horizontalLongPulseNoise",
            "low-level calibration diagnostic",
        ),
        ("azimuthEncoderLight", "low-level encoder diagnostic"),
        (
            "horizontalNoiseTemperature",
            "low-level calibration diagnostic",
        ),
        ("linearity", "low-level calibration diagnostic"),
        ("transmitterPeakPower", "low-level calibration diagnostic"),
        ("horizontalDeltadBZ0", "low-level calibration diagnostic"),
        (
            "transmitterRecycleCount",
            "low-level transmitter diagnostic",
        ),
        ("verticalDeltadBZ0", "low-level calibration diagnostic"),
        ("receiverBias", "low-level calibration diagnostic"),
        (
            "shortPulseHorizontaldBZ0",
            "low-level calibration diagnostic",
        ),
        ("transmitterImbalance", "low-level calibration diagnostic"),
        (
            "longPulseHorizontaldBZ0",
            "low-level calibration diagnostic",
        ),
        (
            "transmitterLeavingAirTemperature",
            "low-level transmitter diagnostic",
        ),
        ("dynamicRange", "low-level calibration diagnostic"),
        ("shortPulseVerticaldBZ0", "low-level calibration diagnostic"),
        ("longPulseVerticaldBZ0", "low-level calibration diagnostic"),
        ("pathLossWG04Circulator", "low-level adaptation calibration"),
        (
            "antennaGainIncludingRadome",
            "low-level adaptation calibration",
        ),
        ("pathLossA6ArcDetector", "low-level adaptation calibration"),
        ("cohoPowerAtA1J4", "low-level adaptation calibration"),
        (
            "ameHorzizontalTestSignalPower",
            "low-level adaptation calibration",
        ),
        (
            "pathLossTransmitterCouplerCoupling",
            "low-level adaptation calibration",
        ),
        ("staloPowerAtA1J2", "low-level adaptation calibration"),
        (
            "ameNoiseSourceHorizontalExcessNoiseRatio",
            "low-level adaptation calibration",
        ),
        (
            "pathLossVerticalIFHeliaxTo4AT16",
            "low-level adaptation calibration",
        ),
        ("pathLossAT4Attenuator", "low-level adaptation calibration"),
        (
            "pathLossHorzontalIFHeliaxTo4AT17",
            "low-level adaptation calibration",
        ),
        (
            "pathLossIFDRIFAntiAliasFilter",
            "low-level adaptation calibration",
        ),
        (
            "pathLossIFDBurstAntiAliasFilter",
            "low-level adaptation calibration",
        ),
        (
            "pathLossWG02HarmonicFilter",
            "low-level adaptation calibration",
        ),
        (
            "transmitterPowerDataWattsFactor",
            "low-level adaptation calibration",
        ),
        (
            "pathLossWaveguideKlystronToSwitch",
            "low-level adaptation calibration",
        ),
        (
            "pulseWidthTransmitterOutputShortPulse",
            "low-level adaptation calibration",
        ),
        (
            "pulseWidthTransmitterOutputLongPulse",
            "low-level adaptation calibration",
        ),
        (
            "pathLossWG06SpectrumFilter",
            "low-level adaptation calibration",
        ),
        (
            "horizontalReceiverNoiseShortPulse",
            "low-level adaptation calibration",
        ),
        (
            "horizontalReceiverNoiseLongPulse",
            "low-level adaptation calibration",
        ),
        ("value", "folded into each displayed measurement"),
        ("unitCode", "folded into each displayed measurement"),
        ("minValue", "folded into each displayed measurement"),
        ("maxValue", "folded into each displayed measurement"),
        ("qualityControl", "raw measurement quality flag"),
    ];
}

impl Summarize for RadarStationsResponse {
    fn summarize(&self, options: &SummaryOptions) -> Summary {
        let mut summary =
            Summary::new("Radar stations").subtitle(format!("{} stations", self.len()));
        if self.is_empty() {
            return summary.push(Section::Empty {
                key: Some("features"),
                message: "No radar stations matched the request".to_owned(),
            });
        }
        summary = summary.push(Section::Table {
            heading: None,
            columns: vec![
                Column::new("Station ID", Some("id")),
                Column::new("Name", Some("name")),
                Column::new("Type", Some("stationType")),
                Column::new("Elevation", Some("elevation")).align(Align::Right),
                Column::new("Time Zone", Some("timeZone")),
            ],
            rows: self
                .iter()
                .map(|feature| {
                    let station = &feature.station;
                    vec![
                        Value::identifier(station.id.to_string()).into(),
                        Value::text(Some(&station.name)).into(),
                        Value::text(Some(&station.station_type)).into(),
                        measurement(Some(&station.elevation), QuantityKind::Height, options).into(),
                        Value::text(station.time_zone.iana_name()).into(),
                    ]
                })
                .collect(),
        });
        summary
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        ("type", "GeoJSON envelope type"),
        ("features", "each station is one table row"),
        ("geometry", "available from the single-station command"),
        ("properties", "station properties supply each row"),
        ("@id", "the station id identifies the same resource"),
        ("@type", "always a radar-station resource"),
        ("latency", "available from the single-station command"),
        ("rda", "available from the single-station command"),
        ("value", "folded into elevation"),
        ("unitCode", "folded into elevation"),
    ];
}

impl Summarize for RadarStationAlarmsResponse {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let mut summary =
            Summary::new("Radar station alarms").subtitle(format!("{} alarms", self.len()));
        if self.is_empty() {
            return summary.push(Section::Empty {
                key: Some("@graph"),
                message: "No active radar-station alarms".to_owned(),
            });
        }
        summary = summary.push(Section::Table {
            heading: None,
            columns: vec![
                Column::new("Station ID", Some("stationId")),
                Column::new("Alarm Time", Some("timestamp")),
                Column::new("Message", Some("message")),
                Column::new("Status", Some("status")),
                Column::new("Active Channel", Some("activeChannel")).align(Align::Right),
            ],
            rows: self
                .iter()
                .map(|alarm| {
                    vec![
                        alarm
                            .station_id
                            .as_ref()
                            .map_or(Value::Missing, |id| Value::identifier(id.to_string()))
                            .into(),
                        optional_timestamp(alarm.timestamp).into(),
                        Value::text(alarm.message.as_deref()).into(),
                        Value::text(alarm.status.as_deref()).into(),
                        alarm
                            .active_channel
                            .map_or(Value::Missing, |value| {
                                Value::count(value.unsigned_abs().into())
                            })
                            .into(),
                    ]
                })
                .collect(),
        });
        summary
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        ("@id", "identifies the queried alarm collection"),
        ("@graph", "each alarm is one table row"),
        ("@type", "always a radar-station alarm"),
    ];
}

impl Summarize for RadarQueuesResponse {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let mut summary =
            Summary::new("Radar data queue").subtitle(format!("{} queued products", self.len()));
        if self.is_empty() {
            return summary.push(Section::Empty {
                key: Some("@graph"),
                message: "No products are waiting in this radar queue".to_owned(),
            });
        }
        summary = summary.push(Section::Table {
            heading: None,
            columns: vec![
                Column::new("Host", Some("host")),
                Column::new("Station", Some("stationId")),
                Column::new("Arrival", Some("arrivalTime")),
                Column::new("Created", Some("creationTime")),
                Column::new("Type", Some("type")),
                Column::new("Feed", Some("feed")),
                Column::new("Resolution", Some("resolutionVersion")).align(Align::Right),
                Column::new("Sequence", Some("sequenceNumber")),
                Column::new("Size", Some("size")).align(Align::Right),
            ],
            rows: self
                .iter()
                .map(|entry| {
                    vec![
                        Value::text(Some(&entry.host)).into(),
                        Value::identifier(entry.station_id.to_string()).into(),
                        Value::timestamp(entry.arrival_time).into(),
                        Value::timestamp(entry.creation_time).into(),
                        Value::text(Some(&entry.data_type)).into(),
                        Value::text(Some(&entry.feed)).into(),
                        Value::count(entry.resolution_version.unsigned_abs().into()).into(),
                        Value::identifier(entry.sequence_number.clone()).into(),
                        Value::bytes(entry.size).into(),
                    ]
                })
                .collect(),
        });
        summary
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        ("@id", "identifies the queried queue"),
        ("@graph", "each queued product is one table row"),
        ("@type", "always a radar-queue item"),
    ];
}

impl Summarize for RadarServerTelemetry {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let targets = &self.ping.targets;
        let command = self.command.as_ref();
        let mut summary =
            Summary::new("Radar server")
                .subtitle(self.id.clone())
                .push(Section::Facts {
                    heading: Some("General".to_owned()),
                    facts: vec![
                        Fact::new("Server ID", Some("id"), Value::identifier(self.id.clone())),
                        Fact::new("Type", Some("type"), Value::text(Some(&self.server_type))),
                        Fact::new("Active", Some("active"), Value::yes_no(self.active)),
                        Fact::new("Primary", Some("primary"), Value::yes_no(self.primary)),
                        Fact::new(
                            "Aggregate",
                            Some("aggregate"),
                            Value::yes_no(self.aggregate),
                        ),
                        Fact::new("Locked", Some("locked"), Value::yes_no(self.locked)),
                        Fact::new(
                            "Radar Network Up",
                            Some("radarNetworkUp"),
                            Value::yes_no(self.radar_network_up),
                        ),
                        Fact::new(
                            "Collected",
                            Some("collectionTime"),
                            Value::timestamp(self.collection_time),
                        ),
                        Fact::new(
                            "Reporting Host",
                            Some("reportingHost"),
                            Value::text(Some(&self.reporting_host)),
                        ),
                        Fact::new(
                            "Ingest Host",
                            Some("ingestHost"),
                            Value::text(Some(&self.ingest_host)),
                        ),
                    ],
                });
        summary = summary.push(Section::Facts {
            heading: Some("Ping status".to_owned()),
            facts: vec![
                Fact::new(
                    "Observed",
                    Some("timestamp"),
                    Value::timestamp(self.ping.timestamp),
                ),
                Fact::new("Client Targets", Some("client"), {
                    let value = targets.client_summary();
                    ping(value.up, value.total)
                }),
                Fact::new("LDM Targets", Some("ldm"), {
                    let value = targets.ldm_summary();
                    ping(value.up, value.total)
                }),
                Fact::new("Radar Targets", Some("radar"), {
                    let value = targets.radar_summary();
                    ping(value.up, value.total)
                }),
                Fact::new("Server Targets", Some("server"), {
                    let value = targets.server_summary();
                    ping(value.up, value.total)
                }),
                Fact::new("Misc Targets", Some("misc"), {
                    let value = targets.misc_summary();
                    ping(value.up, value.total)
                }),
            ],
        });
        if let Some(command) = command {
            summary = summary.push(Section::Facts {
                heading: Some("Command status".to_owned()),
                facts: vec![
                    Fact::new(
                        "Observed",
                        Some("timestamp"),
                        Value::timestamp(command.timestamp),
                    ),
                    Fact::new(
                        "Last Executed",
                        Some("lastExecuted"),
                        Value::text(Some(&command.last_executed)),
                    ),
                    Fact::new(
                        "Last Executed At",
                        Some("lastExecutedTime"),
                        Value::timestamp(command.last_executed_time),
                    ),
                    Fact::new(
                        "Last NEXRAD Data",
                        Some("lastNexradDataTime"),
                        Value::timestamp(command.last_nexrad_data_time),
                    ),
                    Fact::new(
                        "Last Received",
                        Some("lastReceived"),
                        Value::text(Some(&command.last_received)),
                    ),
                    Fact::new(
                        "Last Received At",
                        Some("lastReceivedTime"),
                        Value::timestamp(command.last_received_time),
                    ),
                ],
            });
        }
        let hardware = &self.hardware;
        summary = summary.push(Section::Facts {
            heading: Some("Hardware".to_owned()),
            facts: vec![
                Fact::new(
                    "Observed",
                    Some("timestamp"),
                    Value::timestamp(hardware.timestamp),
                ),
                Fact::new(
                    "CPU Idle",
                    Some("cpuIdle"),
                    Value::percent(Some(hardware.cpu_idle)),
                ),
                Fact::new(
                    "I/O Utilization",
                    Some("ioUtilization"),
                    Value::percent(Some(hardware.io_utilization)),
                ),
                Fact::new(
                    "Disk",
                    Some("disk"),
                    Value::number(Some(f64::from(hardware.disk)), 0, None),
                ),
                Fact::new(
                    "Load (1m / 5m / 15m)",
                    Some("load1"),
                    Value::list(vec![
                        Value::number(Some(hardware.load1), 2, None),
                        Value::number(Some(hardware.load5), 2, None),
                        Value::number(Some(hardware.load15), 2, None),
                    ]),
                )
                .also(&["load5", "load15"]),
                Fact::new(
                    "Memory",
                    Some("memory"),
                    Value::percent(Some(hardware.memory)),
                ),
                Fact::new(
                    "Up Since",
                    Some("uptime"),
                    Value::timestamp(hardware.uptime),
                ),
            ],
        });
        let ldm = &self.ldm;
        summary = summary.push(Section::Facts {
            heading: Some("Local Data Manager".to_owned()),
            facts: vec![
                Fact::new(
                    "Observed",
                    Some("timestamp"),
                    Value::timestamp(ldm.timestamp),
                ),
                Fact::new("Active", Some("active"), Value::yes_no(Some(ldm.active))),
                Fact::new(
                    "Latest Product",
                    Some("latestProduct"),
                    Value::timestamp(ldm.latest_product),
                ),
                Fact::new(
                    "Oldest Product",
                    Some("oldestProduct"),
                    Value::timestamp(ldm.oldest_product),
                ),
                Fact::new(
                    "Storage",
                    Some("storageSize"),
                    Value::bytes(ldm.storage_size),
                ),
                Fact::new("Products", Some("count"), Value::count(ldm.count)),
            ],
        });
        summary.push(Section::Table {
            heading: Some("Network".to_owned()),
            columns: vec![
                Column::new("Interface", Some("interface")),
                Column::new("Active", Some("active")),
                Column::new("Sent", Some("transNoError")).align(Align::Right),
                Column::new("Send Errors", Some("transError")).align(Align::Right),
                Column::new("Received", Some("recvNoError")).align(Align::Right),
                Column::new("Receive Errors", Some("recvError")).align(Align::Right),
            ],
            rows: [&self.network.eth0, &self.network.eth1]
                .into_iter()
                .map(|interface| {
                    vec![
                        Value::identifier(interface.interface.clone()).into(),
                        Value::yes_no(Some(interface.active)).into(),
                        Value::count(interface.trans_no_error).into(),
                        Value::count(interface.trans_error).into(),
                        Value::count(interface.recv_no_error).into(),
                        Value::count(interface.recv_error).into(),
                    ]
                })
                .collect(),
        })
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        ("@id", "the server id identifies the same resource"),
        ("@type", "always a radar-server resource"),
        ("ping", "expanded as ping-status facts"),
        (
            "targets",
            "each target category is summarized as up over total",
        ),
        ("command", "expanded as command-status facts when present"),
        ("hardware", "expanded as hardware facts"),
        ("ldm", "expanded as Local Data Manager facts"),
        ("network", "expanded as a network table"),
        ("timestamp", "shown within each telemetry section"),
        ("eth0", "shown as a network table row"),
        ("eth1", "shown as a network table row"),
        ("transDropped", "low-level interface counter"),
        ("transOverrun", "low-level interface counter"),
        ("recvDropped", "low-level interface counter"),
        ("recvOverrun", "low-level interface counter"),
    ];
}

impl Summarize for RadarServersResponse {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let mut summary = Summary::new("Radar servers").subtitle(format!("{} servers", self.len()));
        if self.is_empty() {
            return summary.push(Section::Empty {
                key: Some("@graph"),
                message: "No radar servers matched the request".to_owned(),
            });
        }
        summary = summary.push(Section::Table {
            heading: None,
            columns: vec![
                Column::new("Server", Some("id")),
                Column::new("Type", Some("type")),
                Column::new("Active", Some("active")),
                Column::new("Primary", Some("primary")),
                Column::new("Network Up", Some("radarNetworkUp")),
                Column::new("LDM Active", Some("ldm")),
                Column::new("LDM Products", Some("count")).align(Align::Right),
                Column::new("Load (1m)", Some("load1")).align(Align::Right),
                Column::new("Collected", Some("collectionTime")),
                Column::new("Reporter", Some("reportingHost")),
            ],
            rows: self
                .iter()
                .map(|server| {
                    vec![
                        Value::identifier(server.id.clone()).into(),
                        Value::text(Some(&server.server_type)).into(),
                        Value::yes_no(server.active).into(),
                        Value::yes_no(server.primary).into(),
                        Value::yes_no(server.radar_network_up).into(),
                        Value::yes_no(Some(server.ldm.active)).into(),
                        Value::count(server.ldm.count).into(),
                        Value::number(Some(server.hardware.load1), 2, None).into(),
                        Value::timestamp(server.collection_time).into(),
                        Value::text(Some(&server.reporting_host)).into(),
                    ]
                })
                .collect(),
        });
        summary
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        ("@graph", "each server is one table row"),
        ("@id", "the server id identifies the same resource"),
        ("@type", "always a radar-server resource"),
        ("aggregate", "available from the single-server command"),
        ("locked", "available from the single-server command"),
        ("ingestHost", "available from the single-server command"),
        ("ping", "available from the single-server command"),
        ("command", "available from the single-server command"),
        ("hardware", "load1 is folded into the row"),
        ("ldm", "active state and count are folded into the row"),
        ("network", "available from the single-server command"),
    ];
}

impl Summarize for RadarSpgdsResponse {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let mut summary =
            Summary::new("Radar SPGDS telemetry").subtitle(format!("{} hosts", self.len()));
        if self.is_empty() {
            return summary.push(Section::Empty {
                key: Some("@graph"),
                message: "No SPGDS telemetry matched the request".to_owned(),
            });
        }
        summary = summary.push(Section::Table {
            heading: None,
            columns: vec![
                Column::new("Host", Some("id")),
                Column::new("Timestamp", Some("timestamp")),
                Column::new("Data Flow", Some("dataflow")),
                Column::new("Connect Queue", Some("connectQ")),
                Column::new("Application", Some("appRunning")),
                Column::new("LDM Connections", Some("conns")).align(Align::Right),
                Column::new("Disk Used", Some("pctUsed")).align(Align::Right),
                Column::new("Inbound", Some("in")).align(Align::Right),
                Column::new("Outbound", Some("out")).align(Align::Right),
                Column::new("Gateways", Some("spg")).align(Align::Right),
            ],
            rows: self
                .iter()
                .map(|entry| {
                    vec![
                        Value::identifier(entry.id.clone()).into(),
                        Value::timestamp(entry.timestamp).into(),
                        Value::text(Some(&entry.dataflow.state)).into(),
                        Value::text(Some(&entry.connect_q.state)).into(),
                        Value::text(Some(&entry.app_running.state)).into(),
                        Value::text(Some(&entry.ldm.conns)).into(),
                        entry
                            .second_hd
                            .percent_used
                            .parse::<f64>()
                            .ok()
                            .map_or(Value::text(Some(&entry.second_hd.percent_used)), |value| {
                                Value::percent(Some(value))
                            })
                            .into(),
                        Value::text(Some(&entry.throughput.inbound)).into(),
                        Value::text(Some(&entry.throughput.outbound)).into(),
                        Value::count(entry.spg.len() as u64).into(),
                    ]
                })
                .collect(),
        });
        summary
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        ("@graph", "each SPGDS host is one table row"),
        ("@type", "always an SPGDS telemetry record"),
        ("state", "shown for the three primary status categories"),
        ("stateSince", "low-level epoch-second diagnostic"),
        ("stateValid", "low-level epoch-second diagnostic"),
        ("ldm", "connection count is folded into the row"),
        ("connsValid", "low-level epoch-second diagnostic"),
        ("secondHD", "disk utilization is folded into the row"),
        ("pctUsedValid", "low-level epoch-second diagnostic"),
        ("spgdsUpSince", "low-level epoch-second diagnostic"),
        ("upSince", "low-level epoch-second diagnostic"),
        ("upSinceValid", "low-level epoch-second diagnostic"),
        (
            "throughput",
            "inbound and outbound values are folded into the row",
        ),
        ("inDateTime", "low-level epoch-second diagnostic"),
        ("inValid", "low-level epoch-second diagnostic"),
        ("outDateTime", "low-level epoch-second diagnostic"),
        ("outValid", "low-level epoch-second diagnostic"),
        ("swimDataState", "per-gateway diagnostic available in JSON"),
        (
            "swimDataStateSince",
            "per-gateway diagnostic available in JSON",
        ),
        (
            "swimDataStateValid",
            "per-gateway diagnostic available in JSON",
        ),
        ("ldmPingState", "per-gateway diagnostic available in JSON"),
        (
            "ldmPingStateSince",
            "per-gateway diagnostic available in JSON",
        ),
        (
            "ldmPingStateValid",
            "per-gateway diagnostic available in JSON",
        ),
    ];
}
