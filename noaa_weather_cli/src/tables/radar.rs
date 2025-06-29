use std::collections::HashMap;

use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::{Attribute, Cell, CellAlignment, ContentArrangement, Table};
use noaa_weather_client::models::radar_server::RadarServerNetworkInterfaceStats;
use noaa_weather_client::models::{
    RadarQueuesResponse, RadarServer, RadarServersResponse, RadarStationAlarmsResponse,
    RadarStationFeature, RadarStationsResponse,
};

use crate::utils::format::{
    format_bytes_to_human_readable, format_datetime_human_readable, format_optional_bool_as_yes_no,
    format_optional_f64_display, format_optional_f64_precise, format_optional_i32,
    format_optional_i64, format_optional_string, format_optional_value_unit,
};

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
fn format_coords(coords: &Option<Vec<f64>>) -> String {
    coords.as_ref().map_or("N/A".to_owned(), |coord| {
        if coord.len() >= 2 {
            format!("Lon: {:.5}, Lat: {:.5}", coord[0], coord[1])
        } else {
            "Invalid Coords".to_owned()
        }
    })
}

/// Creates a summary string for ping targets (e.g., "X / Y up").
fn format_ping_map_summary(map_opt: &Option<HashMap<String, bool>>) -> String {
    map_opt.as_ref().map_or_else(
        || "N/A".to_owned(),
        |map| {
            let total = map.len();
            if total == 0 {
                return "0 targets".to_owned();
            }
            let up_count = map.values().filter(|&&value| value).count();
            format!("{up_count} / {total} up")
        },
    )
}

/// Adds rows for network interface statistics to the table.
fn add_network_interface_stats_rows(
    table: &mut Table,
    if_name: &str,
    stats: &RadarServerNetworkInterfaceStats,
) {
    table.add_row(vec![
        Cell::new(format!("{if_name} Interface")),
        Cell::new(format_optional_string(&stats.interface)),
    ]);
    table.add_row(vec![
        Cell::new(format!("{if_name} Active")),
        Cell::new(format_optional_bool_as_yes_no(&stats.active)),
    ]);
    table.add_row(vec![
        Cell::new(format!("{if_name} Tx Packets (OK/Err/Drop)")),
        Cell::new(format!(
            "{}/{}/{}",
            format_optional_i64(&stats.trans_no_error),
            format_optional_i64(&stats.trans_error),
            format_optional_i64(&stats.trans_dropped)
        )),
    ]);
    table.add_row(vec![
        Cell::new(format!("{if_name} Tx Overruns")),
        Cell::new(format_optional_i64(&stats.trans_overrun)),
    ]);
    table.add_row(vec![
        Cell::new(format!("{if_name} Rx Packets (OK/Err/Drop)")),
        Cell::new(format!(
            "{}/{}/{}",
            format_optional_i64(&stats.recv_no_error),
            format_optional_i64(&stats.recv_error),
            format_optional_i64(&stats.recv_dropped)
        )),
    ]);
    table.add_row(vec![
        Cell::new(format!("{if_name} Rx Overruns")),
        Cell::new(format_optional_i64(&stats.recv_overrun)),
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
pub fn create_radar_station_feature_table(radar_station_feature: &RadarStationFeature) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
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
        Cell::new(format_optional_string(&radar_station_feature.id)),
    ]);

    if let Some(geometry) = &radar_station_feature.geometry {
        table.add_row(vec![
            Cell::new("Coordinates"),
            Cell::new(format_coords(&geometry.coordinates)),
        ]);
    } else {
        table.add_row(vec![Cell::new("Coordinates"), Cell::new("N/A")]);
    }

    if let Some(station) = &radar_station_feature.radar_station {
        table.add_row(vec![
            Cell::new("Station ID (ICAO)"),
            Cell::new(format_optional_string(&station.id)),
        ]);
        table.add_row(vec![
            Cell::new("Name"),
            Cell::new(format_optional_string(&station.name)),
        ]);
        table.add_row(vec![
            Cell::new("Type"),
            Cell::new(format_optional_string(&station.station_type)),
        ]);
        table.add_row(vec![
            Cell::new("Elevation"),
            Cell::new(format_optional_value_unit(&station.elevation)),
        ]);
        table.add_row(vec![
            Cell::new("Time Zone"),
            Cell::new(format_optional_string(&station.time_zone)),
        ]);

        add_section_header(&mut table, "Latency Information");
        if let Some(latency) = &station.latency {
            table.add_row(vec![
                Cell::new("Current Latency"),
                Cell::new(format_optional_value_unit(&latency.current)),
            ]);
            table.add_row(vec![
                Cell::new("Average Latency"),
                Cell::new(format_optional_value_unit(&latency.average)),
            ]);
            table.add_row(vec![
                Cell::new("Max Latency"),
                Cell::new(format_datetime_human_readable(
                    latency.max_latency_time.as_deref(),
                )),
            ]);
            table.add_row(vec![
                Cell::new("Level II Last Received"),
                Cell::new(format_datetime_human_readable(
                    latency.level_two_last_received_time.as_deref(),
                )),
            ]);
            table.add_row(vec![
                Cell::new("Max Latency Time"),
                Cell::new(format_datetime_human_readable(
                    latency.max_latency_time.as_deref(),
                )),
            ]);
            table.add_row(vec![
                Cell::new("Reporting Host"),
                Cell::new(format_optional_string(&latency.reporting_host)),
            ]);
            table.add_row(vec![
                Cell::new("Data Host"),
                Cell::new(format_optional_string(&latency.host)),
            ]);
        } else {
            table.add_row(vec![Cell::new("Latency Data"), Cell::new("N/A")]);
        }

        add_section_header(&mut table, "RDA Information");
        if let Some(rda_info) = &station.rda {
            table.add_row(vec![
                Cell::new("RDA Timestamp"),
                Cell::new(format_datetime_human_readable(
                    rda_info.timestamp.as_deref(),
                )),
            ]);
            table.add_row(vec![
                Cell::new("RDA Reporting Host"),
                Cell::new(format_optional_string(&rda_info.reporting_host)),
            ]);
            if let Some(rda_props) = &rda_info.properties {
                table.add_row(vec![
                    Cell::new("Volume Coverage Pattern (VCP)"),
                    Cell::new(format_optional_string(&rda_props.volume_coverage_pattern)),
                ]);
                table.add_row(vec![
                    Cell::new("Control Status"),
                    Cell::new(format_optional_string(&rda_props.control_status)),
                ]);
                table.add_row(vec![
                    Cell::new("Build Number"),
                    Cell::new(format_optional_f64_display(&rda_props.build_number)),
                ]);
                table.add_row(vec![
                    Cell::new("Alarm Summary"),
                    Cell::new(format_optional_string(&rda_props.alarm_summary)),
                ]);
                table.add_row(vec![
                    Cell::new("Mode"),
                    Cell::new(format_optional_string(&rda_props.mode)),
                ]);
                table.add_row(vec![
                    Cell::new("Generator State"),
                    Cell::new(format_optional_string(&rda_props.generator_state)),
                ]);
                table.add_row(vec![
                    Cell::new("Super Resolution Status"),
                    Cell::new(format_optional_string(&rda_props.super_resolution_status)),
                ]);
                table.add_row(vec![
                    Cell::new("Operability Status"),
                    Cell::new(format_optional_string(&rda_props.operability_status)),
                ]);
                table.add_row(vec![
                    Cell::new("RDA Status"),
                    Cell::new(format_optional_string(&rda_props.status)),
                ]);
                table.add_row(vec![
                    Cell::new("Avg. Transmitter Power"),
                    Cell::new(format_optional_value_unit(
                        &rda_props.average_transmitter_power,
                    )),
                ]);
                table.add_row(vec![
                    Cell::new("Reflectivity Cal. Correction"),
                    Cell::new(format_optional_value_unit(
                        &rda_props.reflectivity_calibration_correction,
                    )),
                ]);
            } else {
                table.add_row(vec![Cell::new("RDA Properties"), Cell::new("N/A")]);
            }
        } else {
            table.add_row(vec![Cell::new("RDA Data"), Cell::new("N/A")]);
        }

        add_section_header(&mut table, "Performance Information");
        if let Some(perf_info) = &station.performance {
            table.add_row(vec![
                Cell::new("Perf. Timestamp"),
                Cell::new(format_datetime_human_readable(
                    perf_info.timestamp.as_deref(),
                )),
            ]);
            table.add_row(vec![
                Cell::new("Perf. Reporting Host"),
                Cell::new(format_optional_string(&perf_info.reporting_host)),
            ]);
            if let Some(perf_props) = &perf_info.properties {
                table.add_row(vec![
                    Cell::new("NTP Status"),
                    Cell::new(format_optional_i32(&perf_props.ntp_status)),
                ]);
                table.add_row(vec![
                    Cell::new("Linearity"),
                    Cell::new(format_optional_f64_precise(&perf_props.linearity)),
                ]);
                table.add_row(vec![
                    Cell::new("Power Source"),
                    Cell::new(format_optional_string(&perf_props.power_source)),
                ]);
                table.add_row(vec![
                    Cell::new("Fuel Level"),
                    Cell::new(format_optional_value_unit(&perf_props.fuel_level)),
                ]);
                table.add_row(vec![
                    Cell::new("Shelter Temp."),
                    Cell::new(format_optional_value_unit(&perf_props.shelter_temperature)),
                ]);
                table.add_row(vec![
                    Cell::new("Radome Air Temp."),
                    Cell::new(format_optional_value_unit(
                        &perf_props.radome_air_temperature,
                    )),
                ]);
                table.add_row(vec![
                    Cell::new("Transmitter Peak Power"),
                    Cell::new(format_optional_value_unit(
                        &perf_props.transmitter_peak_power,
                    )),
                ]);
                table.add_row(vec![
                    Cell::new("Performance Check Time"),
                    Cell::new(format_datetime_human_readable(
                        perf_props.performance_check_time.as_deref(),
                    )),
                ]);
            } else {
                table.add_row(vec![Cell::new("Performance Properties"), Cell::new("N/A")]);
            }
        } else {
            table.add_row(vec![Cell::new("Performance Data"), Cell::new("N/A")]);
        }

        add_section_header(&mut table, "Adaptation Highlights");
        if let Some(adapt_info) = &station.adaptation {
            table.add_row(vec![
                Cell::new("Adapt. Timestamp"),
                Cell::new(format_datetime_human_readable(
                    adapt_info.timestamp.as_deref(),
                )),
            ]);
            if let Some(adapt_props) = &adapt_info.properties {
                table.add_row(vec![
                    Cell::new("Transmitter Freq."),
                    Cell::new(format_optional_value_unit(
                        &adapt_props.transmitter_frequency,
                    )),
                ]);
                table.add_row(vec![
                    Cell::new("Antenna Gain (incl. Radome)"),
                    Cell::new(format_optional_value_unit(
                        &adapt_props.antenna_gain_including_radome,
                    )),
                ]);
                table.add_row(vec![
                    Cell::new("Tx Spectrum Filter Installed"),
                    Cell::new(format_optional_string(
                        &adapt_props.transmitter_spectrum_filter_installed,
                    )),
                ]);
            } else {
                table.add_row(vec![Cell::new("Adaptation Properties"), Cell::new("N/A")]);
            }
        } else {
            table.add_row(vec![Cell::new("Adaptation Data"), Cell::new("N/A")]);
        }
    } else {
        table.add_row(vec![
            Cell::new("Station Specifics"),
            Cell::new("N/A - Detailed radar station data missing"),
        ]);
    }

    table
}

pub fn create_radar_stations_table(radar_stations: &RadarStationsResponse) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
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
                Cell::new(format_optional_string(&station.id)),
                Cell::new(format_optional_string(&station.name)),
                Cell::new(format_optional_string(&station.station_type)),
                Cell::new(format_optional_value_unit(&station.elevation)),
                Cell::new(format_optional_string(&station.time_zone)),
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
pub fn create_radar_station_alarms_table(
    radar_station_alarms: &RadarStationAlarmsResponse,
) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
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

    for alarm in radar_station_alarms.radar_station_alarms.iter().flatten() {
        table.add_row(vec![
            Cell::new(format_optional_string(&alarm.station_id)),
            Cell::new(format_datetime_human_readable(alarm.timestamp.as_deref())),
            Cell::new(format_optional_string(&alarm.message)),
            Cell::new(format_optional_string(&alarm.status).to_uppercase()),
            Cell::new(format_optional_i32(&alarm.active_channel)),
        ]);
    }
    table
}

pub fn create_radar_data_queue_table(radar_data_queue: &RadarQueuesResponse) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
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

    for entry in radar_data_queue.radar_queues.iter().flatten() {
        table.add_row(vec![
            Cell::new(format_optional_string(&entry.host)),
            Cell::new(format_datetime_human_readable(
                entry.arrival_time.as_deref(),
            )),
            Cell::new(format_datetime_human_readable(
                entry.creation_time.as_deref(),
            )),
            Cell::new(format_optional_string(&entry.r#type)),
            Cell::new(format_optional_string(&entry.feed)),
            Cell::new(format_optional_i32(&entry.resolution_version)),
            Cell::new(format_optional_string(&entry.sequence_number)),
            Cell::new(format_optional_i32(&entry.size)),
        ]);
    }
    table
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
pub fn create_radar_server_table(radar_server: &RadarServer) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);

    let server_id_str = radar_server.id.as_deref().unwrap_or("Unknown Server");
    table.set_header(vec![
        Cell::new(format!("Radar Server Status: {server_id_str}")).add_attribute(Attribute::Bold),
        Cell::new("").add_attribute(Attribute::Bold),
    ]);

    // --- General Server Information ---
    add_section_header(&mut table, "General");
    table.add_row(vec![
        Cell::new("Server ID"),
        Cell::new(format_optional_string(&radar_server.id)),
    ]);
    table.add_row(vec![
        Cell::new("Server Type"),
        Cell::new(format_optional_string(&radar_server.r#type)),
    ]);
    table.add_row(vec![
        Cell::new("Active"),
        Cell::new(format_optional_bool_as_yes_no(&radar_server.active)),
    ]);
    table.add_row(vec![
        Cell::new("Primary"),
        Cell::new(format_optional_bool_as_yes_no(&radar_server.primary)),
    ]);
    table.add_row(vec![
        Cell::new("Aggregate"),
        Cell::new(format_optional_bool_as_yes_no(&radar_server.aggregate)),
    ]);
    table.add_row(vec![
        Cell::new("Locked"),
        Cell::new(format_optional_bool_as_yes_no(&radar_server.locked)),
    ]);
    table.add_row(vec![
        Cell::new("Radar Network Up"),
        Cell::new(format_optional_bool_as_yes_no(
            &radar_server.radar_network_up,
        )),
    ]);
    table.add_row(vec![
        Cell::new("Collection Time"),
        Cell::new(format_datetime_human_readable(
            radar_server.collection_time.as_deref(),
        )),
    ]);
    table.add_row(vec![
        Cell::new("Reporting Host"),
        Cell::new(format_optional_string(&radar_server.reporting_host)),
    ]);

    // --- Ping Status ---
    add_section_header(&mut table, "Ping Status");
    if let Some(ping_status) = &radar_server.ping {
        table.add_row(vec![
            Cell::new("Ping Status Timestamp"),
            Cell::new(format_datetime_human_readable(
                ping_status.timestamp.as_deref(),
            )),
        ]);
        if let Some(targets) = &ping_status.targets {
            table.add_row(vec![
                Cell::new("Client Targets"),
                Cell::new(format_ping_map_summary(&targets.client)),
            ]);
            table.add_row(vec![
                Cell::new("LDM Targets"),
                Cell::new(format_ping_map_summary(&targets.ldm)),
            ]);
            table.add_row(vec![
                Cell::new("Radar Targets"),
                Cell::new(format_ping_map_summary(&targets.radar)),
            ]);
            table.add_row(vec![
                Cell::new("Server Targets"),
                Cell::new(format_ping_map_summary(&targets.server)),
            ]);
            table.add_row(vec![
                Cell::new("Misc Targets"),
                Cell::new(format_ping_map_summary(&targets.misc)),
            ]);
        }
    } else {
        table.add_row(vec![Cell::new("Ping Data"), Cell::new("N/A")]);
    }

    // --- Command Status ---
    add_section_header(&mut table, "Command Status");
    if let Some(command) = &radar_server.command {
        table.add_row(vec![
            Cell::new("Command Status Timestamp"),
            Cell::new(format_datetime_human_readable(command.timestamp.as_deref())),
        ]);
        table.add_row(vec![
            Cell::new("Last Executed"),
            Cell::new(format_optional_string(&command.last_executed)),
        ]);
        table.add_row(vec![
            Cell::new("Last Executed Time"),
            Cell::new(format_datetime_human_readable(
                command.last_executed_time.as_deref(),
            )),
        ]);
        table.add_row(vec![
            Cell::new("Last NEXRAD Data Time"),
            Cell::new(format_datetime_human_readable(
                command.last_nexrad_data_time.as_deref(),
            )),
        ]);
        table.add_row(vec![
            Cell::new("Last Received"),
            Cell::new(format_optional_string(&command.last_received)),
        ]);
        table.add_row(vec![
            Cell::new("Last Received Time"),
            Cell::new(format_datetime_human_readable(
                command.last_received_time.as_deref(),
            )),
        ]);
    } else {
        table.add_row(vec![Cell::new("Command Data"), Cell::new("N/A")]);
    }

    // --- Hardware Status ---
    add_section_header(&mut table, "Hardware Status");
    if let Some(hardware) = &radar_server.hardware {
        table.add_row(vec![
            Cell::new("Hardware Status Timestamp"),
            Cell::new(format_datetime_human_readable(
                hardware.timestamp.as_deref(),
            )),
        ]);
        table.add_row(vec![
            Cell::new("CPU Idle"),
            Cell::new(format!(
                "{} %",
                format_optional_f64_display(&hardware.cpu_idle)
            )),
        ]);
        table.add_row(vec![
            Cell::new("I/O Utilization"),
            Cell::new(format!(
                "{} %",
                format_optional_f64_display(&hardware.io_utilization)
            )),
        ]);
        table.add_row(vec![
            Cell::new("Disk Status/Value"),
            Cell::new(format_optional_i32(&hardware.disk)),
        ]);
        table.add_row(vec![
            Cell::new("Load Avg (1m/5m/15m)"),
            Cell::new(format!(
                "{}/{}/{}",
                format_optional_f64_display(&hardware.load1),
                format_optional_f64_display(&hardware.load5),
                format_optional_f64_display(&hardware.load15)
            )),
        ]);
        table.add_row(vec![
            Cell::new("Memory Usage"),
            Cell::new(format!(
                "{} %",
                format_optional_f64_display(&hardware.memory)
            )),
        ]);
        table.add_row(vec![
            Cell::new("System Uptime Since"),
            Cell::new(format_datetime_human_readable(hardware.uptime.as_deref())),
        ]);
    } else {
        table.add_row(vec![Cell::new("Hardware Data"), Cell::new("N/A")]);
    }

    // --- LDM Status ---
    add_section_header(&mut table, "LDM Status");
    if let Some(ldm) = &radar_server.ldm {
        table.add_row(vec![
            Cell::new("LDM Status Timestamp"),
            Cell::new(format_datetime_human_readable(ldm.timestamp.as_deref())),
        ]);
        table.add_row(vec![
            Cell::new("LDM Active"),
            Cell::new(format_optional_bool_as_yes_no(&ldm.active)),
        ]);
        table.add_row(vec![
            Cell::new("Latest Product Time"),
            Cell::new(format_datetime_human_readable(
                ldm.latest_product.as_deref(),
            )),
        ]);
        table.add_row(vec![
            Cell::new("Oldest Product Time"),
            Cell::new(format_datetime_human_readable(
                ldm.oldest_product.as_deref(),
            )),
        ]);
        table.add_row(vec![
            Cell::new("Storage Size"),
            Cell::new(format_bytes_to_human_readable(ldm.storage_size)),
        ]);
        table.add_row(vec![
            Cell::new("Product Count"),
            Cell::new(format_optional_i32(&ldm.count)),
        ]);
    } else {
        table.add_row(vec![Cell::new("LDM Data"), Cell::new("N/A")]);
    }

    // --- Network Status ---
    add_section_header(&mut table, "Network Status");
    if let Some(network) = &radar_server.network {
        table.add_row(vec![
            Cell::new("Network Status Timestamp"),
            Cell::new(format_datetime_human_readable(network.timestamp.as_deref())),
        ]);
        if let Some(if_stats) = &network.eth0 {
            add_network_interface_stats_rows(&mut table, "eth0", if_stats);
        } else {
            table.add_row(vec![Cell::new("eth0 Interface Data"), Cell::new("N/A")]);
        }
        if let Some(if_stats) = &network.eth1 {
            add_network_interface_stats_rows(&mut table, "eth1", if_stats);
        } else {
            table.add_row(vec![Cell::new("eth1 Interface Data"), Cell::new("N/A")]);
        }
    } else {
        table.add_row(vec![Cell::new("Network Data"), Cell::new("N/A")]);
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
pub fn create_radar_servers_table(radar_servers_response: &RadarServersResponse) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
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
            let ldm_active = server.ldm.as_ref().and_then(|ldm| ldm.active).map_or_else(
                || "N/A".to_owned(),
                |bool_opt| format_optional_bool_as_yes_no(&Some(bool_opt)),
            );
            let ldm_count = server
                .ldm
                .as_ref()
                .and_then(|ldm| ldm.count)
                .map_or_else(|| "N/A".to_owned(), |count| count.to_string());
            let load1 = server
                .hardware
                .as_ref()
                .and_then(|hw| hw.load1)
                .map_or_else(|| "N/A".to_owned(), |load1| format!("{load1:.2}"));

            table.add_row(vec![
                Cell::new(format_optional_string(&server.id)),
                Cell::new(format_optional_string(&server.r#type)),
                Cell::new(format_optional_bool_as_yes_no(&server.active)),
                Cell::new(format_optional_bool_as_yes_no(&server.primary)),
                Cell::new(format_optional_bool_as_yes_no(&server.radar_network_up)),
                Cell::new(ldm_active),
                Cell::new(ldm_count).set_alignment(CellAlignment::Right),
                Cell::new(load1).set_alignment(CellAlignment::Right),
                Cell::new(format_datetime_human_readable(
                    server.collection_time.as_deref(),
                )),
                Cell::new(format_optional_string(&server.reporting_host)),
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

    table
}
