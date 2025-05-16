use anyhow::Result;
use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::{ContentArrangement, Table};
use noaa_weather_client::models::ObservationStationCollectionGeoJson;

/// Formats station data into a comfy table.
pub fn create_stations_table(station_data: &ObservationStationCollectionGeoJson) -> Result<Table> {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec!["Station ID", "Name", "Elevation (m)", "Time Zone"]);

    for feature in &station_data.features {
        if let Some(station) = &feature.properties {
            // Safely access the nested elevation value
            let elevation_str = station.elevation.clone().map_or_else(
                || "N/A".to_string(),
                |quantitative_value| {
                    quantitative_value.value.map_or_else(
                        || "N/A".to_string(),
                        |value| format!("{:?}", value.unwrap_or_default()),
                    )
                },
            );

            let timezone_str = station
                .time_zone
                .clone()
                .unwrap_or_else(|| "N/A".to_string());

            table.add_row(vec![
                station.station_identifier.as_deref().unwrap_or("N/A"),
                station.name.as_deref().unwrap_or("N/A"),
                &elevation_str,
                &timezone_str,
            ]);
        } else {
            // Handle cases where station properties are missing
            table.add_row(vec!["Error", "Missing station data", "N/A", "N/A"]);
        }
    }

    Ok(table)
}
