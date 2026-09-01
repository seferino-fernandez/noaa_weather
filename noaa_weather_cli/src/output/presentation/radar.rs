use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::{Attribute, Cell, CellAlignment, ContentArrangement, Table};
use noaa_weather_client::models::radar::{
    RadarNetworkInterfaceTelemetry, RadarPingSummary, RadarPosition, RadarServerTelemetry,
    RadarStationTelemetry,
};
use noaa_weather_client::models::{
    RadarQueuesResponse, RadarServer, RadarServersResponse, RadarSpgdsResponse,
    RadarStationAlarmsResponse, RadarStationFeature, RadarStationsResponse,
};

use crate::output::PresentationDocument;
use crate::output::presentation::{DefaultPresentation, DefaultPresenter, PresentationError};

/// Creates a concise summary of SPGDS host telemetry.
fn create_radar_spgds_table(
    response: &RadarSpgdsResponse,
    presenter: &DefaultPresenter,
) -> Result<Table, PresentationError> {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header([
        "ID",
        "Timestamp",
        "Data Flow",
        "Connections",
        "Disk",
        "Gateways",
    ]);

    for (index, entry) in response.spgds.iter().enumerate() {
        table.add_row(vec![
            Cell::new(presenter.text(entry.id.as_deref())),
            Cell::new(presenter.timestamp(
                format!("radar.spgds[{index}].timestamp"),
                entry.timestamp.as_deref(),
            )?),
            Cell::new(
                presenter.text(
                    entry
                        .dataflow
                        .as_ref()
                        .and_then(|status| status.state.as_deref()),
                ),
            ),
            Cell::new(
                presenter.text(
                    entry
                        .ldm
                        .as_ref()
                        .and_then(|status| status.conns.as_deref()),
                ),
            ),
            Cell::new(
                presenter.text(
                    entry
                        .second_hd
                        .as_ref()
                        .and_then(|status| status.state.as_deref()),
                ),
            ),
            Cell::new(entry.spg.len()),
        ]);
    }

    Ok(table)
}

// --- Helper Functions ---
// These are kept private to this module as they are specific to formatting radar station data.

/// Adds a styled section header row to the table.
fn add_section_header(table: &mut Table, title: &str) {
    table.add_row(vec![
        Cell::new(title)
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("").add_attribute(Attribute::Bold), // Second cell to maintain column structure
    ]);
}

/// Formats geographic coordinates `Option<Vec<f64>>` (longitude, latitude) for display.
/// Uses "N/A" if None or invalid.
fn format_position(position: RadarPosition, presenter: &DefaultPresenter) -> String {
    match position {
        RadarPosition::Missing => presenter.missing(),
        RadarPosition::Invalid => "Invalid Coords".to_owned(),
        RadarPosition::Coordinates {
            longitude,
            latitude,
        } => format!("Lon: {longitude:.5}, Lat: {latitude:.5}"),
        _ => presenter.missing(),
    }
}

/// Creates a summary string for ping targets (e.g., "X / Y up").
fn format_ping_summary(summary: Option<RadarPingSummary>, presenter: &DefaultPresenter) -> String {
    summary.map_or_else(
        || presenter.missing(),
        |summary| {
            if summary.total() == 0 {
                return "0 targets".to_owned();
            }
            format!("{} / {} up", summary.up(), summary.total())
        },
    )
}

/// Adds rows for network interface statistics to the table.
fn add_network_interface_stats_rows(
    table: &mut Table,
    if_name: &str,
    stats: &RadarNetworkInterfaceTelemetry,
    presenter: &DefaultPresenter,
) {
    table.add_row(vec![
        Cell::new(format!("{if_name} Interface")),
        Cell::new(presenter.text(stats.interface())),
    ]);
    table.add_row(vec![
        Cell::new(format!("{if_name} Active")),
        Cell::new(presenter.yes_no(stats.active())),
    ]);
    table.add_row(vec![
        Cell::new(format!("{if_name} Tx Packets (OK/Err/Drop)")),
        Cell::new(format!(
            "{}/{}/{}",
            presenter.integer(stats.transmitted_ok()),
            presenter.integer(stats.transmitted_errors()),
            presenter.integer(stats.transmitted_dropped())
        )),
    ]);
    table.add_row(vec![
        Cell::new(format!("{if_name} Tx Overruns")),
        Cell::new(presenter.integer(stats.transmitted_overruns())),
    ]);
    table.add_row(vec![
        Cell::new(format!("{if_name} Rx Packets (OK/Err/Drop)")),
        Cell::new(format!(
            "{}/{}/{}",
            presenter.integer(stats.received_ok()),
            presenter.integer(stats.received_errors()),
            presenter.integer(stats.received_dropped())
        )),
    ]);
    table.add_row(vec![
        Cell::new(format!("{if_name} Rx Overruns")),
        Cell::new(presenter.integer(stats.received_overruns())),
    ]);
}

/// Creates a table displaying detailed information for a single NOAA radar station.
///
/// The table is structured with sections for general information, latency,
/// RDA (Radar Data Acquisition), performance, and adaptation highlights.
/// Optional fields are displayed as "N/A" if not present in the data.
///
/// # Arguments
///
/// * `radar_station_feature`: A reference to the `RadarStationFeature` containing the data.
///
/// # Returns
///
/// A `Result<Table>` which is the `comfy_table::Table` ready for printing,
/// or an error if table creation fails (though current implementation always returns Ok).
fn create_radar_station_telemetry_table(
    telemetry: &RadarStationTelemetry,
    presenter: &DefaultPresenter,
) -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("Station Information")
            .set_alignment(CellAlignment::Left)
            .add_attribute(Attribute::Bold),
        Cell::new("")
            .set_alignment(CellAlignment::Left)
            .add_attribute(Attribute::Bold),
    ]);

    // --- General Information ---
    add_section_header(&mut table, "General Information");
    table.add_row(vec![
        Cell::new("Feature ID (URL)"),
        Cell::new(presenter.text(telemetry.feature_id())),
    ]);
    table.add_row(vec![
        Cell::new("Coordinates"),
        Cell::new(format_position(telemetry.position(), presenter)),
    ]);

    if let Some(station) = telemetry.station() {
        table.add_row(vec![
            Cell::new("Station ID (ICAO)"),
            Cell::new(presenter.text(station.id())),
        ]);
        table.add_row(vec![
            Cell::new("Name"),
            Cell::new(presenter.text(station.name())),
        ]);
        table.add_row(vec![
            Cell::new("Type"),
            Cell::new(presenter.text(station.station_type())),
        ]);
        table.add_row(vec![
            Cell::new("Elevation"),
            Cell::new(presenter.radar_measurement(station.elevation())),
        ]);
        table.add_row(vec![
            Cell::new("Time Zone"),
            Cell::new(presenter.text(station.time_zone())),
        ]);

        add_section_header(&mut table, "Latency Information");
        if let Some(latency) = station.latency() {
            table.add_row(vec![
                Cell::new("Current Latency"),
                Cell::new(presenter.radar_measurement(latency.current())),
            ]);
            table.add_row(vec![
                Cell::new("Average Latency"),
                Cell::new(presenter.radar_measurement(latency.average())),
            ]);
            table.add_row(vec![
                Cell::new("Max Latency"),
                Cell::new(presenter.radar_measurement(latency.maximum())),
            ]);
            table.add_row(vec![
                Cell::new("Level II Last Received"),
                Cell::new(presenter.parsed_timestamp(latency.level_two_last_received())),
            ]);
            table.add_row(vec![
                Cell::new("Max Latency Time"),
                Cell::new(presenter.parsed_timestamp(latency.maximum_at())),
            ]);
            table.add_row(vec![
                Cell::new("Reporting Host"),
                Cell::new(presenter.text(latency.reporting_host())),
            ]);
            table.add_row(vec![
                Cell::new("Data Host"),
                Cell::new(presenter.text(latency.data_host())),
            ]);
        } else {
            table.add_row(vec![
                Cell::new("Latency Data"),
                Cell::new(presenter.missing()),
            ]);
        }

        add_section_header(&mut table, "RDA Information");
        if let Some(rda_info) = station.rda() {
            table.add_row(vec![
                Cell::new("RDA Timestamp"),
                Cell::new(presenter.parsed_timestamp(rda_info.timestamp())),
            ]);
            table.add_row(vec![
                Cell::new("RDA Reporting Host"),
                Cell::new(presenter.text(rda_info.reporting_host())),
            ]);
            if let Some(rda_props) = rda_info.properties() {
                table.add_row(vec![
                    Cell::new("Volume Coverage Pattern (VCP)"),
                    Cell::new(presenter.text(rda_props.volume_coverage_pattern())),
                ]);
                table.add_row(vec![
                    Cell::new("Control Status"),
                    Cell::new(presenter.text(rda_props.control_status())),
                ]);
                table.add_row(vec![
                    Cell::new("Build Number"),
                    Cell::new(presenter.decimal(rda_props.build_number())),
                ]);
                table.add_row(vec![
                    Cell::new("Alarm Summary"),
                    Cell::new(presenter.text(rda_props.alarm_summary())),
                ]);
                table.add_row(vec![
                    Cell::new("Mode"),
                    Cell::new(presenter.text(rda_props.mode())),
                ]);
                table.add_row(vec![
                    Cell::new("Generator State"),
                    Cell::new(presenter.text(rda_props.generator_state())),
                ]);
                table.add_row(vec![
                    Cell::new("Super Resolution Status"),
                    Cell::new(presenter.text(rda_props.super_resolution_status())),
                ]);
                table.add_row(vec![
                    Cell::new("Operability Status"),
                    Cell::new(presenter.text(rda_props.operability_status())),
                ]);
                table.add_row(vec![
                    Cell::new("RDA Status"),
                    Cell::new(presenter.text(rda_props.status())),
                ]);
                table.add_row(vec![
                    Cell::new("Avg. Transmitter Power"),
                    Cell::new(presenter.radar_measurement(rda_props.average_transmitter_power())),
                ]);
                table.add_row(vec![
                    Cell::new("Reflectivity Cal. Correction"),
                    Cell::new(
                        presenter
                            .radar_measurement(rda_props.reflectivity_calibration_correction()),
                    ),
                ]);
            } else {
                table.add_row(vec![
                    Cell::new("RDA Properties"),
                    Cell::new(presenter.missing()),
                ]);
            }
        } else {
            table.add_row(vec![Cell::new("RDA Data"), Cell::new(presenter.missing())]);
        }

        add_section_header(&mut table, "Performance Information");
        if let Some(perf_info) = station.performance() {
            table.add_row(vec![
                Cell::new("Perf. Timestamp"),
                Cell::new(presenter.parsed_timestamp(perf_info.timestamp())),
            ]);
            table.add_row(vec![
                Cell::new("Perf. Reporting Host"),
                Cell::new(presenter.text(perf_info.reporting_host())),
            ]);
            if let Some(perf_props) = perf_info.properties() {
                table.add_row(vec![
                    Cell::new("NTP Status"),
                    Cell::new(presenter.integer(perf_props.ntp_status())),
                ]);
                table.add_row(vec![
                    Cell::new("Linearity"),
                    Cell::new(presenter.precise_decimal(perf_props.linearity())),
                ]);
                table.add_row(vec![
                    Cell::new("Power Source"),
                    Cell::new(presenter.text(perf_props.power_source())),
                ]);
                table.add_row(vec![
                    Cell::new("Fuel Level"),
                    Cell::new(presenter.radar_measurement(perf_props.fuel_level())),
                ]);
                table.add_row(vec![
                    Cell::new("Shelter Temp."),
                    Cell::new(presenter.radar_measurement(perf_props.shelter_temperature())),
                ]);
                table.add_row(vec![
                    Cell::new("Radome Air Temp."),
                    Cell::new(presenter.radar_measurement(perf_props.radome_air_temperature())),
                ]);
                table.add_row(vec![
                    Cell::new("Transmitter Peak Power"),
                    Cell::new(presenter.radar_measurement(perf_props.transmitter_peak_power())),
                ]);
                table.add_row(vec![
                    Cell::new("Performance Check Time"),
                    Cell::new(presenter.parsed_timestamp(perf_props.performance_check_time())),
                ]);
            } else {
                table.add_row(vec![
                    Cell::new("Performance Properties"),
                    Cell::new(presenter.missing()),
                ]);
            }
        } else {
            table.add_row(vec![
                Cell::new("Performance Data"),
                Cell::new(presenter.missing()),
            ]);
        }

        add_section_header(&mut table, "Adaptation Highlights");
        if let Some(adapt_info) = station.adaptation() {
            table.add_row(vec![
                Cell::new("Adapt. Timestamp"),
                Cell::new(presenter.parsed_timestamp(adapt_info.timestamp())),
            ]);
            if let Some(adapt_props) = adapt_info.properties() {
                table.add_row(vec![
                    Cell::new("Transmitter Freq."),
                    Cell::new(presenter.radar_measurement(adapt_props.transmitter_frequency())),
                ]);
                table.add_row(vec![
                    Cell::new("Antenna Gain (incl. Radome)"),
                    Cell::new(
                        presenter.radar_measurement(adapt_props.antenna_gain_including_radome()),
                    ),
                ]);
                table.add_row(vec![
                    Cell::new("Tx Spectrum Filter Installed"),
                    Cell::new(presenter.text(adapt_props.transmitter_spectrum_filter_installed())),
                ]);
            } else {
                table.add_row(vec![
                    Cell::new("Adaptation Properties"),
                    Cell::new(presenter.missing()),
                ]);
            }
        } else {
            table.add_row(vec![
                Cell::new("Adaptation Data"),
                Cell::new(presenter.missing()),
            ]);
        }
    } else {
        table.add_row(vec![
            Cell::new("Station Specifics"),
            Cell::new("N/A - Detailed radar station data missing"),
        ]);
    }

    table
}

fn create_radar_stations_table(
    radar_stations: &RadarStationsResponse,
    presenter: &DefaultPresenter,
) -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("Station ID (ICAO)")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Name")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Type")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Elevation")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Time Zone")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);

    for radar_station_feature in radar_stations.features.iter().flatten() {
        if let Some(station) = &radar_station_feature.radar_station {
            table.add_row(vec![
                Cell::new(presenter.text(station.id.as_deref())),
                Cell::new(presenter.text(station.name.as_deref())),
                Cell::new(presenter.text(station.station_type.as_deref())),
                Cell::new(presenter.value_unit(station.elevation.as_ref())),
                Cell::new(presenter.text(station.time_zone.as_deref())),
            ]);
        }
    }
    table
}

/// Creates a table displaying detailed information for a single NOAA radar station alarm.
///
/// The table is structured with sections for general information, latency,
/// RDA (Radar Data Acquisition), performance, and adaptation highlights.
/// Optional fields are displayed as "N/A" if not present in the data.
fn create_radar_station_alarms_table(
    radar_station_alarms: &RadarStationAlarmsResponse,
    presenter: &DefaultPresenter,
) -> Result<Table, PresentationError> {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("Station ID")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Alarm Time")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Message")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Status")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Active Channel")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);

    for (index, alarm) in radar_station_alarms
        .radar_station_alarms
        .iter()
        .flatten()
        .enumerate()
    {
        table.add_row(vec![
            Cell::new(presenter.text(alarm.station_id.as_deref())),
            Cell::new(presenter.timestamp(
                format!("radar_station_alarms[{index}].timestamp"),
                alarm.timestamp.as_deref(),
            )?),
            Cell::new(presenter.text(alarm.message.as_deref())),
            Cell::new(presenter.text(alarm.status.as_deref()).to_uppercase()),
            Cell::new(presenter.integer(alarm.active_channel)),
        ]);
    }
    Ok(table)
}

fn create_radar_data_queue_table(
    radar_data_queue: &RadarQueuesResponse,
    presenter: &DefaultPresenter,
) -> Result<Table, PresentationError> {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("Host")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Arrival Time")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Creation Time")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Type")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Feed")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Resolution Version")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Sequence Number")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Size")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);

    for (index, entry) in radar_data_queue.radar_queues.iter().flatten().enumerate() {
        table.add_row(vec![
            Cell::new(presenter.text(entry.host.as_deref())),
            Cell::new(presenter.timestamp(
                format!("radar_queues[{index}].arrival_time"),
                entry.arrival_time.as_deref(),
            )?),
            Cell::new(presenter.timestamp(
                format!("radar_queues[{index}].creation_time"),
                entry.creation_time.as_deref(),
            )?),
            Cell::new(presenter.text(entry.r#type.as_deref())),
            Cell::new(presenter.text(entry.feed.as_deref())),
            Cell::new(presenter.integer(entry.resolution_version)),
            Cell::new(presenter.text(entry.sequence_number.as_deref())),
            Cell::new(presenter.integer(entry.size)),
        ]);
    }
    Ok(table)
}

/// Creates a table displaying status information for a NOAA Radar Server.
///
/// The table provides a detailed overview of the server, including its general status,
/// ping statistics, command execution status, hardware metrics, LDM (Local Data Manager)
/// status, and network interface statistics. Optional fields are shown as "N/A".
///
/// # Arguments
///
/// * `radar_server`: A reference to the `RadarServer` struct containing the server data.
///
/// # Returns
///
/// A `Result<Table>` which is the `comfy_table::Table` ready for printing.
fn create_radar_server_telemetry_table(
    telemetry: &RadarServerTelemetry,
    presenter: &DefaultPresenter,
) -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);

    let server_id_str = telemetry.id().unwrap_or("Unknown Server");
    table.set_header(vec![
        Cell::new(format!("Radar Server Status: {server_id_str}")).add_attribute(Attribute::Bold),
        Cell::new("").add_attribute(Attribute::Bold),
    ]);

    // --- General Server Information ---
    add_section_header(&mut table, "General");
    table.add_row(vec![
        Cell::new("Server ID"),
        Cell::new(presenter.text(telemetry.id())),
    ]);
    table.add_row(vec![
        Cell::new("Server Type"),
        Cell::new(presenter.text(telemetry.server_type())),
    ]);
    table.add_row(vec![
        Cell::new("Active"),
        Cell::new(presenter.yes_no(telemetry.active())),
    ]);
    table.add_row(vec![
        Cell::new("Primary"),
        Cell::new(presenter.yes_no(telemetry.primary())),
    ]);
    table.add_row(vec![
        Cell::new("Aggregate"),
        Cell::new(presenter.yes_no(telemetry.aggregate())),
    ]);
    table.add_row(vec![
        Cell::new("Locked"),
        Cell::new(presenter.yes_no(telemetry.locked())),
    ]);
    table.add_row(vec![
        Cell::new("Radar Network Up"),
        Cell::new(presenter.yes_no(telemetry.radar_network_up())),
    ]);
    table.add_row(vec![
        Cell::new("Collection Time"),
        Cell::new(presenter.parsed_timestamp(telemetry.collection_time())),
    ]);
    table.add_row(vec![
        Cell::new("Reporting Host"),
        Cell::new(presenter.text(telemetry.reporting_host())),
    ]);

    // --- Ping Status ---
    add_section_header(&mut table, "Ping Status");
    if let Some(ping_status) = telemetry.ping() {
        table.add_row(vec![
            Cell::new("Ping Status Timestamp"),
            Cell::new(presenter.parsed_timestamp(ping_status.timestamp())),
        ]);
        if let Some(targets) = ping_status.targets() {
            table.add_row(vec![
                Cell::new("Client Targets"),
                Cell::new(format_ping_summary(targets.client(), presenter)),
            ]);
            table.add_row(vec![
                Cell::new("LDM Targets"),
                Cell::new(format_ping_summary(targets.ldm(), presenter)),
            ]);
            table.add_row(vec![
                Cell::new("Radar Targets"),
                Cell::new(format_ping_summary(targets.radar(), presenter)),
            ]);
            table.add_row(vec![
                Cell::new("Server Targets"),
                Cell::new(format_ping_summary(targets.server(), presenter)),
            ]);
            table.add_row(vec![
                Cell::new("Misc Targets"),
                Cell::new(format_ping_summary(targets.misc(), presenter)),
            ]);
        }
    } else {
        table.add_row(vec![Cell::new("Ping Data"), Cell::new(presenter.missing())]);
    }

    // --- Command Status ---
    add_section_header(&mut table, "Command Status");
    if let Some(command) = telemetry.command() {
        table.add_row(vec![
            Cell::new("Command Status Timestamp"),
            Cell::new(presenter.parsed_timestamp(command.timestamp())),
        ]);
        table.add_row(vec![
            Cell::new("Last Executed"),
            Cell::new(presenter.text(command.last_executed())),
        ]);
        table.add_row(vec![
            Cell::new("Last Executed Time"),
            Cell::new(presenter.parsed_timestamp(command.last_executed_time())),
        ]);
        table.add_row(vec![
            Cell::new("Last NEXRAD Data Time"),
            Cell::new(presenter.parsed_timestamp(command.last_nexrad_data_time())),
        ]);
        table.add_row(vec![
            Cell::new("Last Received"),
            Cell::new(presenter.text(command.last_received())),
        ]);
        table.add_row(vec![
            Cell::new("Last Received Time"),
            Cell::new(presenter.parsed_timestamp(command.last_received_time())),
        ]);
    } else {
        table.add_row(vec![
            Cell::new("Command Data"),
            Cell::new(presenter.missing()),
        ]);
    }

    // --- Hardware Status ---
    add_section_header(&mut table, "Hardware Status");
    if let Some(hardware) = telemetry.hardware() {
        table.add_row(vec![
            Cell::new("Hardware Status Timestamp"),
            Cell::new(presenter.parsed_timestamp(hardware.timestamp())),
        ]);
        table.add_row(vec![
            Cell::new("CPU Idle"),
            Cell::new(format!("{} %", presenter.decimal(hardware.cpu_idle()))),
        ]);
        table.add_row(vec![
            Cell::new("I/O Utilization"),
            Cell::new(format!(
                "{} %",
                presenter.decimal(hardware.io_utilization())
            )),
        ]);
        table.add_row(vec![
            Cell::new("Disk Status/Value"),
            Cell::new(presenter.integer(hardware.disk())),
        ]);
        table.add_row(vec![
            Cell::new("Load Avg (1m/5m/15m)"),
            Cell::new(format!(
                "{}/{}/{}",
                presenter.decimal(hardware.load1()),
                presenter.decimal(hardware.load5()),
                presenter.decimal(hardware.load15())
            )),
        ]);
        table.add_row(vec![
            Cell::new("Memory Usage"),
            Cell::new(format!("{} %", presenter.decimal(hardware.memory()))),
        ]);
        table.add_row(vec![
            Cell::new("System Uptime Since"),
            Cell::new(presenter.parsed_timestamp(hardware.uptime())),
        ]);
    } else {
        table.add_row(vec![
            Cell::new("Hardware Data"),
            Cell::new(presenter.missing()),
        ]);
    }

    // --- LDM Status ---
    add_section_header(&mut table, "LDM Status");
    if let Some(ldm) = telemetry.ldm() {
        table.add_row(vec![
            Cell::new("LDM Status Timestamp"),
            Cell::new(presenter.parsed_timestamp(ldm.timestamp())),
        ]);
        table.add_row(vec![
            Cell::new("LDM Active"),
            Cell::new(presenter.yes_no(ldm.active())),
        ]);
        table.add_row(vec![
            Cell::new("Latest Product Time"),
            Cell::new(presenter.parsed_timestamp(ldm.latest_product())),
        ]);
        table.add_row(vec![
            Cell::new("Oldest Product Time"),
            Cell::new(presenter.parsed_timestamp(ldm.oldest_product())),
        ]);
        table.add_row(vec![
            Cell::new("Storage Size"),
            Cell::new(presenter.bytes(ldm.storage_size())),
        ]);
        table.add_row(vec![
            Cell::new("Product Count"),
            Cell::new(presenter.integer(ldm.count())),
        ]);
    } else {
        table.add_row(vec![Cell::new("LDM Data"), Cell::new(presenter.missing())]);
    }

    // --- Network Status ---
    add_section_header(&mut table, "Network Status");
    if let Some(network) = telemetry.network() {
        table.add_row(vec![
            Cell::new("Network Status Timestamp"),
            Cell::new(presenter.parsed_timestamp(network.timestamp())),
        ]);
        if let Some(if_stats) = network.eth0() {
            add_network_interface_stats_rows(&mut table, "eth0", if_stats, presenter);
        } else {
            table.add_row(vec![
                Cell::new("eth0 Interface Data"),
                Cell::new(presenter.missing()),
            ]);
        }
        if let Some(if_stats) = network.eth1() {
            add_network_interface_stats_rows(&mut table, "eth1", if_stats, presenter);
        } else {
            table.add_row(vec![
                Cell::new("eth1 Interface Data"),
                Cell::new(presenter.missing()),
            ]);
        }
    } else {
        table.add_row(vec![
            Cell::new("Network Data"),
            Cell::new(presenter.missing()),
        ]);
    }

    table
}

/// Creates a table listing multiple NOAA Radar Servers with key summary information.
///
/// This function processes a `RadarServersResponse`, which contains a list of radar servers,
/// and formats them into a table. Each row represents a server, displaying its ID, type,
/// operational status (active, primary, network up), LDM status (if applicable),
/// current load, and the last data collection time.
///
/// # Arguments
///
/// * `radar_servers_response`: A `RadarServersResponse` struct containing the list of radar servers.
///
/// # Returns
///
/// A `Result<Table>` which is the `comfy_table::Table` ready for printing.
fn create_radar_servers_table(
    radar_servers_response: &RadarServersResponse,
    presenter: &DefaultPresenter,
) -> Result<Table, PresentationError> {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        Cell::new("Server ID")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Type")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Active")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Primary")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Net Up")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("LDM Active")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("LDM Count")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Load (1m)")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Collected")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Reporter")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);

    if let Some(servers) = &radar_servers_response.radar_servers {
        for server in servers {
            let telemetry =
                RadarServerTelemetry::try_from(server).map_err(PresentationError::source_data)?;
            let ldm_active = presenter.yes_no(telemetry.ldm().and_then(|ldm| ldm.active()));
            let ldm_count = presenter.integer(telemetry.ldm().and_then(|ldm| ldm.count()));
            let load1 =
                presenter.decimal(telemetry.hardware().and_then(|hardware| hardware.load1()));

            table.add_row(vec![
                Cell::new(presenter.text(telemetry.id())),
                Cell::new(presenter.text(telemetry.server_type())),
                Cell::new(presenter.yes_no(telemetry.active())),
                Cell::new(presenter.yes_no(telemetry.primary())),
                Cell::new(presenter.yes_no(telemetry.radar_network_up())),
                Cell::new(ldm_active),
                Cell::new(ldm_count).set_alignment(CellAlignment::Right),
                Cell::new(load1).set_alignment(CellAlignment::Right),
                Cell::new(presenter.parsed_timestamp(telemetry.collection_time())),
                Cell::new(presenter.text(telemetry.reporting_host())),
            ]);
        }
    } else {
        // Optionally, add a row indicating no servers were found if the list is empty or None
        table.add_row(vec![
            Cell::new("No radar server data available")
                .set_alignment(CellAlignment::Center)
                .add_attribute(Attribute::Italic);
            10
        ]);
    }

    Ok(table)
}

impl DefaultPresentation for RadarSpgdsResponse {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_radar_spgds_table(
            self, presenter,
        )?))
    }
}

impl DefaultPresentation for RadarStationFeature {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        let telemetry =
            RadarStationTelemetry::try_from(self).map_err(PresentationError::source_data)?;
        Ok(PresentationDocument::table(
            create_radar_station_telemetry_table(&telemetry, presenter),
        ))
    }
}

impl DefaultPresentation for RadarStationsResponse {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_radar_stations_table(
            self, presenter,
        )))
    }
}

impl DefaultPresentation for RadarStationAlarmsResponse {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(
            create_radar_station_alarms_table(self, presenter)?,
        ))
    }
}

impl DefaultPresentation for RadarQueuesResponse {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_radar_data_queue_table(
            self, presenter,
        )?))
    }
}

impl DefaultPresentation for RadarServer {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        let telemetry =
            RadarServerTelemetry::try_from(self).map_err(PresentationError::source_data)?;
        Ok(PresentationDocument::table(
            create_radar_server_telemetry_table(&telemetry, presenter),
        ))
    }
}

impl DefaultPresentation for RadarServersResponse {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_radar_servers_table(
            self, presenter,
        )?))
    }
}
