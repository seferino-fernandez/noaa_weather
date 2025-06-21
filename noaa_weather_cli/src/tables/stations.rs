use comfy_table::presets::{UTF8_FULL, UTF8_FULL_CONDENSED};
use comfy_table::{Attribute, Cell, CellAlignment, ContentArrangement, Table};
use jiff::Timestamp;
use noaa_weather_client::models::terminal_aerodrome_forecast::{
    BaseForecastMeteorologicalAerodromeForecastCloudAerodromeCloudForecastLayer,
    BaseForecastMeteorologicalAerodromeForecastPrevailingVisibility,
    BaseForecastMeteorologicalAerodromeForecastSurfaceWindAerodromeSurfaceWindForecast,
    ChangeForecastMeteorologicalAerodromeForecastCloudAerodromeCloudForecastLayer,
    ChangeForecastMeteorologicalAerodromeForecastPrevailingVisibility,
    ChangeForecastMeteorologicalAerodromeForecastSurfaceWindAerodromeSurfaceWindForecast, Weather,
};
use noaa_weather_client::models::{
    GeoJsonGeometry, ObservationCollectionGeoJson, ObservationGeoJson,
    ObservationStationCollectionGeoJson, ObservationStationGeoJson, TerminalAerodromeForecast,
    TerminalAerodromeForecastsResponse,
};

use crate::utils::format::{
    format_datetime_human_readable, format_optional_value_unit, get_zone_from_url,
};

/// Creates a table listing all observation stations with key summary information.
///
/// This function processes a `ObservationStationCollectionGeoJson`, which contains a list of observation stations,
/// and formats them into a table. Each row represents a station, displaying its ID, name,
/// elevation, and time zone.
///
pub fn create_stations_table(station_data: &ObservationStationCollectionGeoJson) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("Station ID")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Name")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Elevation (m)")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Time Zone")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Point (Coords)")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Zones")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);

    for feature in &station_data.features {
        table.add_row(create_station_row(feature));
    }

    table
}

/// Creates a table listing a single observation station with key summary information.
///
/// This function processes a `ObservationStationGeoJson`, which contains a single observation station,
/// and formats it into a table. Each row represents a station, displaying its ID, name,
/// elevation, and time zone.
///
pub fn create_observation_station_table(observation_station: &ObservationStationGeoJson) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("Station ID")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Name")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Elevation (m)")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Time Zone")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Point (Coords)")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Zones")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);

    table.add_row(create_station_row(observation_station));

    table
}

/// Creates a table listing the latest observation for a single observation station.
///
/// This function processes an `ObservationGeoJson`, which contains a single observation,
/// and formats it into a table. Each row represents a station, displaying its ID, name,
/// elevation, and time zone.
///
pub fn create_stations_observation_table(observation: &ObservationGeoJson) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);

    let props = &observation.properties;

    let station_id_str =
        get_zone_from_url(props.station.as_ref()).unwrap_or_else(|| "N/A".to_owned());

    let title = format!("Station: {station_id_str} - Observation");
    table.set_header(vec![
        Cell::new(title)
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);

    table.add_row(vec![
        Cell::new("Timestamp").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(format_datetime_human_readable(props.timestamp.as_deref())),
    ]);

    table.add_row(vec![
        Cell::new("Text Description").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(props.text_description.as_deref().unwrap_or("N/A")),
    ]);

    table.add_row(vec![
        Cell::new("Temperature").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(format_optional_value_unit(&props.temperature)),
    ]);

    table.add_row(vec![
        Cell::new("Dewpoint").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(format_optional_value_unit(&props.dewpoint)),
    ]);

    table.add_row(vec![
        Cell::new("Wind Direction").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(format_optional_value_unit(&props.wind_direction)),
    ]);

    table.add_row(vec![
        Cell::new("Wind Speed").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(format_optional_value_unit(&props.wind_speed)),
    ]);

    table.add_row(vec![
        Cell::new("Wind Gust").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(format_optional_value_unit(&props.wind_gust)),
    ]);

    table.add_row(vec![
        Cell::new("Barometric Pressure").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(format_optional_value_unit(&props.barometric_pressure)),
    ]);

    table.add_row(vec![
        Cell::new("Sea Level Pressure").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(format_optional_value_unit(&props.sea_level_pressure)),
    ]);

    table.add_row(vec![
        Cell::new("Visibility").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(format_optional_value_unit(&props.visibility)),
    ]);

    table.add_row(vec![
        Cell::new("Relative Humidity").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(format_optional_value_unit(&props.relative_humidity)),
    ]);

    table.add_row(vec![
        Cell::new("Wind Chill").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(format_optional_value_unit(&props.wind_chill)),
    ]);

    table.add_row(vec![
        Cell::new("Heat Index").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(format_optional_value_unit(&props.heat_index)),
    ]);

    table
}

/// Creates a table listing the latest observation for a single observation station.
///
/// This function processes an `ObservationCollectionGeoJson`, which contains a single observation,
/// and formats it into a table. Each row represents a station, displaying its ID, name,
/// elevation, and time zone.
///
pub fn create_stations_observations_table(observations: &ObservationCollectionGeoJson) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        Cell::new("Timestamp")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Temperature")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Dewpoint")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Wind Direction")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Wind Speed")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Wind Gust")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Barometric Pressure")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Sea Level Pressure")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Visibility")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Relative Humidity")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Wind Chill")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Heat Index")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);
    let observations_features = &observations.features;

    for observation in observations_features {
        let timestamp_str =
            format_datetime_human_readable(observation.properties.timestamp.as_deref());
        let temperature_str = format_optional_value_unit(&observation.properties.temperature);
        let dewpoint_str = format_optional_value_unit(&observation.properties.dewpoint);
        let wind_direction_str = format_optional_value_unit(&observation.properties.wind_direction);
        let wind_speed_str = format_optional_value_unit(&observation.properties.wind_speed);
        let wind_gust_str = format_optional_value_unit(&observation.properties.wind_gust);
        let barometric_pressure_str =
            format_optional_value_unit(&observation.properties.barometric_pressure);
        let sea_level_pressure_str =
            format_optional_value_unit(&observation.properties.sea_level_pressure);
        let visibility_str = format_optional_value_unit(&observation.properties.visibility);
        let relative_humidity_str =
            format_optional_value_unit(&observation.properties.relative_humidity);
        let wind_chill_str = format_optional_value_unit(&observation.properties.wind_chill);
        let heat_index_str = format_optional_value_unit(&observation.properties.heat_index);

        table.add_row(vec![
            Cell::new(timestamp_str),
            Cell::new(temperature_str),
            Cell::new(dewpoint_str),
            Cell::new(wind_direction_str),
            Cell::new(wind_speed_str),
            Cell::new(wind_gust_str),
            Cell::new(barometric_pressure_str),
            Cell::new(sea_level_pressure_str),
            Cell::new(visibility_str),
            Cell::new(relative_humidity_str),
            Cell::new(wind_chill_str),
            Cell::new(heat_index_str),
        ]);
    }

    table
}

/// Creates a table listing the metadata for Terminal Aerodrome Forecasts (TAFs) for a single airport station.
///
/// This function processes a `TerminalAerodromeForecastsResponse`, which contains metadata for TAFs,
/// and formats it into a table. Each row represents a TAF, displaying its ID, issue time, location,
/// start time, and end time.
///
pub fn create_stations_tafs_metadata_table(tafs: &TerminalAerodromeForecastsResponse) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        Cell::new("ID")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Issue Time")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Location")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Start")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("End")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Geometry")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);

    for taf in tafs.graph.as_ref().unwrap_or(&vec![]) {
        table.add_row(vec![
            Cell::new(taf.id.clone()),
            Cell::new(format_datetime_human_readable(taf.issue_time.as_deref())),
            Cell::new(taf.location.as_deref().unwrap_or("N/A")),
            Cell::new(format_datetime_human_readable(taf.start.as_deref())),
            Cell::new(format_datetime_human_readable(taf.end.as_deref())),
            Cell::new(taf.geometry.as_deref().unwrap_or("N/A")),
        ]);
    }

    table
}

// --- Time Formatting Helpers (Simplified for UTC Display) ---

/// Formats a TAF timestamp string into a "DD Mon HH:MM UTC" string.
/// Example: "01 Jun 05:27 UTC"
fn format_taf_datetime_as_utc(datetime_str: &str) -> String {
    datetime_str.parse::<Timestamp>().map_or_else(
        |_| "Invalid time".to_owned(),
        |ts| {
            let day = ts.strftime("%d").to_string();
            format!(
                "{} {} UTC",
                day.trim_start_matches('0'), // e.g., "1" instead of "01"
                ts.strftime("%b %H:%M")      // e.g., "Jun 05:27"
            )
        },
    )
}

/// Formats a TAF validity period (begin and end timestamps) as UTC.
/// Example: "01 Jun 06:00 to 02 Jun 12:00 UTC"
fn format_taf_validity_period_as_utc(begin_str: &str, end_str: &str) -> String {
    let begin_ts_res = begin_str.parse::<Timestamp>();
    let end_ts_res: Result<Timestamp, jiff::Error> = end_str.parse::<Timestamp>();

    match (begin_ts_res, end_ts_res) {
        (Ok(b_ts), Ok(e_ts)) => {
            let b_day = b_ts.strftime("%d").to_string();
            let e_day = e_ts.strftime("%d").to_string();
            format!(
                "{} {} to {} {} UTC",
                b_day.trim_start_matches('0'),
                b_ts.strftime("%b %H:%M"),
                e_day.trim_start_matches('0'),
                e_ts.strftime("%b %H:%M")
            )
        }
        _ => "Invalid period".to_owned(),
    }
}

// --- Weather Element Formatting Helpers (Unchanged from previous version, as they don't involve time conversion) ---

// VisibilityDataProvider trait and impls
trait VisibilityDataProvider {
    fn text_val(&self) -> &Option<String>;
    fn uom_val(&self) -> &String;
}
impl VisibilityDataProvider for BaseForecastMeteorologicalAerodromeForecastPrevailingVisibility {
    fn text_val(&self) -> &Option<String> {
        &self.text
    }
    fn uom_val(&self) -> &String {
        &self.uom
    }
}
impl VisibilityDataProvider for ChangeForecastMeteorologicalAerodromeForecastPrevailingVisibility {
    fn text_val(&self) -> &Option<String> {
        &self.text
    }
    fn uom_val(&self) -> &String {
        &self.uom
    }
}

fn format_visibility_generic<T: VisibilityDataProvider>(
    vis_data_opt: Option<&T>,
    operator_opt: Option<&String>,
) -> String {
    match vis_data_opt {
        Some(vis_data) => {
            let value_str = vis_data.text_val().as_deref().unwrap_or("");
            let uom = vis_data.uom_val();

            if (value_str == "10000" || value_str == "9999") && uom == "m" {
                return "10+ km (6+ mi)".to_owned();
            }

            let mut display_str = String::new();
            if let Some(op_str) = operator_opt {
                if op_str == "ABOVE" {
                    display_str.push('\u{2265}');
                } else if op_str == "BELOW" {
                    display_str.push('\u{2264}');
                }
            }

            display_str.push_str(value_str);
            if !uom.is_empty() {
                display_str.push(' ');
                if uom == "m" {
                    display_str.push_str("meters");
                } else {
                    display_str.push_str(uom);
                }
            }
            if display_str.trim().is_empty() || value_str.is_empty() {
                "N/A".to_owned()
            } else {
                display_str
            }
        }
        None => "N/A".to_owned(),
    }
}

// WindDataProvider trait and impls
trait WindDataProvider {
    fn mean_wind_direction_val(&self) -> Option<&str>;
    fn mean_wind_speed_val(&self) -> Option<&str>;
    fn mean_wind_speed_uom(&self) -> Option<&str>;
    fn wind_gust_speed_val(&self) -> Option<&str>;
    fn wind_gust_speed_uom(&self) -> Option<&str>;
    fn is_variable_wind_direction(&self) -> bool;
}
impl WindDataProvider
    for BaseForecastMeteorologicalAerodromeForecastSurfaceWindAerodromeSurfaceWindForecast
{
    fn mean_wind_direction_val(&self) -> Option<&str> {
        let direction = self.mean_wind_direction.as_ref()?;
        direction.text.as_deref()
    }
    fn mean_wind_speed_val(&self) -> Option<&str> {
        let speed = self.mean_wind_speed.as_ref()?;
        speed.text.as_deref()
    }
    fn mean_wind_speed_uom(&self) -> Option<&str> {
        self.mean_wind_speed
            .as_ref()
            .map(|speed| speed.uom.as_str())
    }
    fn wind_gust_speed_val(&self) -> Option<&str> {
        let gust = self.wind_gust_speed.as_ref()?;
        gust.text.as_deref()
    }
    fn wind_gust_speed_uom(&self) -> Option<&str> {
        self.wind_gust_speed.as_ref().map(|g| g.uom.as_str())
    }
    fn is_variable_wind_direction(&self) -> bool {
        self.variable_wind_direction.parse().unwrap_or(false)
    }
}
impl WindDataProvider
    for ChangeForecastMeteorologicalAerodromeForecastSurfaceWindAerodromeSurfaceWindForecast
{
    fn mean_wind_direction_val(&self) -> Option<&str> {
        let direction = self.mean_wind_direction.as_ref()?;
        direction.text.as_deref()
    }
    fn mean_wind_speed_val(&self) -> Option<&str> {
        let speed = self.mean_wind_speed.as_ref()?;
        speed.text.as_deref()
    }
    fn mean_wind_speed_uom(&self) -> Option<&str> {
        self.mean_wind_speed
            .as_ref()
            .map(|speed| speed.uom.as_str())
    }
    fn wind_gust_speed_val(&self) -> Option<&str> {
        let gust = self.wind_gust_speed.as_ref()?;
        gust.text.as_deref()
    }
    fn wind_gust_speed_uom(&self) -> Option<&str> {
        self.wind_gust_speed.as_ref().map(|gust| gust.uom.as_str())
    }
    fn is_variable_wind_direction(&self) -> bool {
        self.variable_wind_direction.parse().unwrap_or(false)
    }
}

fn format_wind_generic<T: WindDataProvider>(wind_data_opt: Option<&T>) -> String {
    wind_data_opt.map_or_else(
        || "N/A".to_owned(),
        |wind_data| {
            let mut parts = Vec::new();
            if wind_data.is_variable_wind_direction() {
                parts.push("Variable (VRB)".to_owned());
            } else if let Some(dir_val) = wind_data.mean_wind_direction_val() {
                parts.push(format!("{dir_val}\u{b0}"));
            }

            if let Some(speed_val) = wind_data.mean_wind_speed_val() {
                let speed_uom = wind_data
                    .mean_wind_speed_uom()
                    .unwrap_or("")
                    .replace("[kn_i]", "kts");
                if !parts.is_empty() && parts.last().is_some_and(|part| part != "at") {
                    parts.push("at".to_owned());
                } else if parts.is_empty() {
                    parts.push("Wind".to_owned());
                }
                parts.push(format!("{speed_val} {speed_uom}"));
            }

            if let Some(gust_val) = wind_data.wind_gust_speed_val() {
                let gust_uom = wind_data
                    .wind_gust_speed_uom()
                    .unwrap_or("")
                    .replace("[kn_i]", "kts");
                parts.push("gusting".to_owned());
                parts.push(format!("{gust_val} {gust_uom}"));
            }

            if parts
                .iter()
                .all(|part| part == "N/A" || part.contains("N/A") || part.is_empty())
                || parts.is_empty()
            {
                "N/A".to_owned()
            } else {
                parts.join(" ")
            }
        },
    )
}

// Cloud amount description, CloudLayerDataProvider trait and impls, format_clouds_generic
fn get_cloud_amount_description(xlink_href: &str) -> String {
    if xlink_href.contains("/FEW") {
        "Few clouds".to_owned()
    } else if xlink_href.contains("/SCT") {
        "Scattered clouds".to_owned()
    } else if xlink_href.contains("/BKN") {
        "Broken clouds".to_owned()
    } else if xlink_href.contains("/OVC") {
        "Overcast".to_owned()
    } else if xlink_href.contains("/NSC") {
        "No significant cloud".to_owned()
    } else if xlink_href.contains("/SKC") || xlink_href.contains("/CLR") {
        "Sky clear".to_owned()
    } else {
        xlink_href.rsplit('/').next().unwrap_or("N/A").to_owned()
    }
}
trait CloudLayerDataProvider {
    fn amount_href(&self) -> &String;
    fn base_value(&self) -> Option<String>;
    fn base_uom(&self) -> &String;
}
impl CloudLayerDataProvider
    for BaseForecastMeteorologicalAerodromeForecastCloudAerodromeCloudForecastLayer
{
    fn amount_href(&self) -> &String {
        &self.cloud_layer.amount.xlink_href
    }
    fn base_value(&self) -> Option<String> {
        self.cloud_layer.base.text.clone()
    }
    fn base_uom(&self) -> &String {
        &self.cloud_layer.base.uom
    }
}
impl CloudLayerDataProvider
    for ChangeForecastMeteorologicalAerodromeForecastCloudAerodromeCloudForecastLayer
{
    fn amount_href(&self) -> &String {
        &self.cloud_layer.amount.xlink_href
    }
    fn base_value(&self) -> Option<String> {
        self.cloud_layer.base.text.clone()
    }
    fn base_uom(&self) -> &String {
        &self.cloud_layer.base.uom
    }
}
fn format_clouds_generic<CLD: CloudLayerDataProvider>(
    layers_data_opt: Option<&Vec<CLD>>,
) -> String {
    match layers_data_opt {
        Some(layers) if !layers.is_empty() => layers
            .iter()
            .map(|layer_data| {
                let amount = get_cloud_amount_description(layer_data.amount_href());
                let base_val = layer_data.base_value().unwrap_or_else(|| "N/A".to_owned());
                let base_uom = layer_data.base_uom().replace("[ft_i]", "ft AGL");
                format!("{amount} at {base_val} {base_uom}")
            })
            .collect::<Vec<String>>()
            .join("\n"),
        _ => "No significant clouds or data N/A".to_owned(),
    }
}

// Weather description
fn get_weather_description(weather_opt: Option<&Vec<Weather>>) -> String {
    match weather_opt {
        Some(weather_vec) if !weather_vec.is_empty() => weather_vec
            .iter()
            .map(|weather| {
                let href = &weather.xlink_href;
                if href.contains("VCSH") {
                    "Showers in Vicinity".to_owned()
                } else if href.contains("-SHRA") {
                    "Light Rain Showers".to_owned()
                } else if href.contains("SHRA") {
                    "Rain Showers".to_owned()
                } else if href.contains("+SHRA") {
                    "Heavy Rain Showers".to_owned()
                } else if href.contains("TSRA") {
                    "Thunderstorm with Rain".to_owned()
                } else if href.contains("TS") {
                    "Thunderstorm".to_owned()
                } else {
                    href.rsplit('/').next().unwrap_or("Unknown").to_owned()
                }
            })
            .collect::<Vec<String>>()
            .join(", "),
        _ => "No significant weather".to_owned(),
    }
}

/// Helper function to add a forecast period's details to the table.
fn add_forecast_period_to_table<SWD, PVD, CLD>(
    table: &mut Table,
    period_title: &str,
    surface_wind_data: Option<&SWD>,
    prevailing_visibility_data: Option<&PVD>,
    visibility_operator: Option<&String>,
    weather_data: Option<&Vec<Weather>>,
    cloud_layers_data: Option<&Vec<CLD>>,
) where
    SWD: WindDataProvider + Sized,
    PVD: VisibilityDataProvider + Sized,
    CLD: CloudLayerDataProvider + Sized,
{
    table.add_row(vec![
        Cell::new("---").set_alignment(CellAlignment::Center),
        Cell::new("---").set_alignment(CellAlignment::Center),
    ]);
    table.add_row(vec![
        Cell::new(period_title)
            .add_attribute(Attribute::Bold)
            .add_attribute(Attribute::Underlined),
        Cell::new(""),
    ]);
    table.add_row(vec![
        Cell::new("Wind:").add_attribute(Attribute::Bold),
        Cell::new(format_wind_generic(surface_wind_data)),
    ]);
    table.add_row(vec![
        Cell::new("Visibility:").add_attribute(Attribute::Bold),
        Cell::new(format_visibility_generic(
            prevailing_visibility_data,
            visibility_operator,
        )),
    ]);
    table.add_row(vec![
        Cell::new("Weather:").add_attribute(Attribute::Bold),
        Cell::new(get_weather_description(weather_data)),
    ]);
    table.add_row(vec![
        Cell::new("Clouds:").add_attribute(Attribute::Bold),
        Cell::new(format_clouds_generic(cloud_layers_data)),
    ]);
}

/// Creates a table displaying a Terminal Aerodrome Forecast (TAF) in a user-friendly format.
/// All times are displayed in UTC.
///
/// This function processes a `TerminalAerodromeForecast` struct, extracting key meteorological
/// information for different time periods within the forecast. It formats this information
/// into a `comfy_table::Table` for clear presentation in a Command Line Interface (CLI).
///
/// The table includes:
/// - General information: Airport, issue time, and full validity period (all in UTC).
/// - Base forecast: The initial set of weather conditions.
/// - Change forecasts: Subsequent periods detailing expected changes.
///
/// For each forecast period, it displays:
/// - Wind: Direction, speed, and gusts.
/// - Visibility: Prevailing visibility.
/// - Weather: Significant weather phenomena.
/// - Clouds: Cloud layers with amount and base height.
///
/// # Arguments
/// * `taf_bulletin`: A reference to the `TerminalAerodromeForecast` struct containing the TAF data.
///
pub fn create_stations_taf_table(taf_bulletin: &TerminalAerodromeForecast) -> Table {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Category")
                .add_attribute(Attribute::Bold)
                .set_alignment(CellAlignment::Left),
            Cell::new("Details")
                .add_attribute(Attribute::Bold)
                .set_alignment(CellAlignment::Left),
        ]);

    let taf = &taf_bulletin.ns0_meteorological_information.taf;

    // --- General Information Section ---
    let airport_icao = &taf
        .aerodrome
        .aixm_airport_heliport
        .aixm_time_slice
        .aixm_airport_heliport_time_slice
        .aixm_location_indicator_icao;
    table.add_row(vec![
        Cell::new("Airport:").add_attribute(Attribute::Bold),
        Cell::new(airport_icao),
    ]);

    let issue_time_str =
        format_taf_datetime_as_utc(&taf.issue_time.ns1_time_instant.ns1_time_position);
    table.add_row(vec![
        Cell::new("Issued:").add_attribute(Attribute::Bold),
        Cell::new(issue_time_str),
    ]);

    let valid_period_str = format_taf_validity_period_as_utc(
        &taf.valid_period.ns1_time_period.ns1_begin_position,
        &taf.valid_period.ns1_time_period.ns1_end_position,
    );
    table.add_row(vec![
        Cell::new("Valid Period (UTC):").add_attribute(Attribute::Bold),
        Cell::new(valid_period_str),
    ]);

    // --- Base Forecast ---
    let bf_props = &taf.base_forecast.meteorological_aerodrome_forecast;
    if let Some(bf_phenom_time) = &bf_props.phenomenon_time {
        let period_title = format!(
            "INITIAL FORECAST\nValid (UTC): {}",
            format_taf_validity_period_as_utc(
                &bf_phenom_time.ns1_time_period.ns1_begin_position,
                &bf_phenom_time.ns1_time_period.ns1_end_position,
            )
        );
        add_forecast_period_to_table(
            &mut table,
            &period_title,
            bf_props
                .surface_wind
                .as_ref()
                .map(|sw| &sw.aerodrome_surface_wind_forecast),
            bf_props.prevailing_visibility.as_ref(),
            bf_props.prevailing_visibility_operator.as_ref(),
            None,
            bf_props
                .cloud
                .as_ref()
                .map(|cloud| &cloud.aerodrome_cloud_forecast.layer),
        );
    }

    // --- Change Forecasts ---
    for change_fcst_item in &taf.change_forecast {
        let cf_props = &change_fcst_item.meteorological_aerodrome_forecast;
        let indicator = &cf_props.change_indicator;

        let period_title = format!(
            "CHANGE GROUP: {}\nValid (UTC): {}",
            indicator,
            format_taf_validity_period_as_utc(
                &cf_props.phenomenon_time.ns1_time_period.ns1_begin_position,
                &cf_props.phenomenon_time.ns1_time_period.ns1_end_position,
            )
        );

        add_forecast_period_to_table(
            &mut table,
            &period_title,
            cf_props
                .surface_wind
                .as_ref()
                .map(|sw| &sw.aerodrome_surface_wind_forecast),
            cf_props.prevailing_visibility.as_ref(),
            cf_props.prevailing_visibility_operator.as_ref(),
            cf_props.weather.as_ref(),
            Some(&cf_props.cloud.aerodrome_cloud_forecast.layer),
        );
    }

    table
}

/// Creates a row for a single observation station.
///
/// This function processes an `ObservationStationGeoJson`, which contains a single observation station,
/// and formats it into a row. Each row represents a station, displaying its ID, name,
/// elevation, and time zone.
///
fn create_station_row(observation_station: &ObservationStationGeoJson) -> Vec<String> {
    let station = &observation_station.properties;

    let elevation_str = format_optional_value_unit(&station.elevation);

    let point_str = observation_station.geometry.as_ref().map_or_else(
        || "N/A".to_owned(),
        |geo_json_geometry| match geo_json_geometry.as_ref() {
            GeoJsonGeometry::GeoJsonPoint(point) => {
                format!("{:?}", point.coordinates)
            }
            GeoJsonGeometry::GeoJsonLineString(_)
            | GeoJsonGeometry::GeoJsonPolygon(_)
            | GeoJsonGeometry::GeoJsonMultiPoint(_)
            | GeoJsonGeometry::GeoJsonMultiLineString(_)
            | GeoJsonGeometry::GeoJsonMultiPolygon(_) => "N/A".to_owned(),
        },
    );

    let timezone_str = station
        .time_zone
        .clone()
        .unwrap_or_else(|| "N/A".to_owned());

    let forecast_zone =
        get_zone_from_url(station.forecast.clone()).unwrap_or_else(|| "N/A".to_owned());

    let county = get_zone_from_url(station.county.clone()).unwrap_or_else(|| "N/A".to_owned());

    let fire_weather_zone =
        get_zone_from_url(station.fire_weather_zone.clone()).unwrap_or_else(|| "N/A".to_owned());

    let zones = format!(
        "Forecast Zone: {forecast_zone}\nCounty: {county}\nFire Weather Zone: {fire_weather_zone}"
    );

    vec![
        station
            .station_identifier
            .as_deref()
            .unwrap_or("N/A")
            .to_owned(),
        station.name.as_deref().unwrap_or("N/A").to_owned(),
        elevation_str,
        timezone_str,
        point_str,
        zones,
    ]
}
