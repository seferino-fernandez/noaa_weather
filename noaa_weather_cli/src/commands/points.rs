use crate::utils::format::{get_zone_from_url, write_output};

use anyhow::Result;
use clap::{Args, Subcommand};
use comfy_table::Table;
use comfy_table::presets::UTF8_FULL_CONDENSED;
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::apis::points as points_api;
use noaa_weather_client::models::PointGeoJson;

use crate::{Cli, tables};

/// Arguments requiring a specific geographical point.
#[derive(Args, Debug, Clone)]
pub struct PointArgs {
    /// Geographical point specified as "latitude,longitude" (e.g., "39.7456,-97.0892").
    point: String,
}

/// Access metadata and nearby stations for a specific geographical point.
#[derive(Subcommand, Debug, Clone)]
pub enum PointCommands {
    /// Get metadata for a specific latitude,longitude point.
    ///
    /// Returns information like the responsible forecast office, grid coordinates,
    /// forecast zone, and links to relevant forecast endpoints.
    /// Example: `noaa-weather points metadata "39.7456,-97.0892"`
    Metadata(PointArgs),
    /// Get a list of observation stations near a specific latitude,longitude point.
    ///
    /// Example: `noaa-weather points stations "39.7456,-97.0892"`
    Stations(PointArgs),
}

/// Handles the execution of point-related subcommands.
///
/// Dispatches the command to the appropriate API function based on the
/// provided `PointCommands` variant and arguments.
///
/// # Arguments
///
/// * `command` - The specific point subcommand and its arguments to execute.
/// * `cli` - The CLI arguments, including the `--json` flag.
/// * `config` - The application configuration containing API details.
///
pub async fn handle_command(
    command: &PointCommands,
    cli: Cli,
    config: &Configuration,
) -> Result<()> {
    match command {
        PointCommands::Metadata(args) => {
            let result = points_api::get_point(config, &args.point)
                .await
                .map_err(|e| anyhow::anyhow!("Error getting point metadata: {}", e))?;

            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = format_point_metadata_table(&result)?;
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
        PointCommands::Stations(args) => {
            let result = points_api::get_point_stations(config, &args.point)
                .await
                .map_err(|e| anyhow::anyhow!("Error getting point stations: {}", e))?;

            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::stations::create_stations_table(&result)?;
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
    }
}

/// Formats point metadata into a comfy table.
fn format_point_metadata_table(point_data: &PointGeoJson) -> Result<Table> {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_header(vec!["Property", "Value"]);

    let properties = &point_data.properties;

    // Helper macro to add rows for Option<T> properties
    macro_rules! add_row_if_some {
        ($table:ident, $label:expr, $value:expr) => {
            if let Some(ref val) = $value {
                $table.add_row(vec![$label, &format!("{val}")]);
            }
        };
        ($table:ident, $label:expr, $value:expr, $formatter:expr) => {
            if let Some(ref val) = $value {
                $table.add_row(vec![$label, &$formatter(val)]);
            }
        };
    }

    add_row_if_some!(
        table,
        "Forecast Office",
        get_zone_from_url(properties.forecast_office.clone())
    );
    add_row_if_some!(table, "Grid ID", properties.grid_id);
    add_row_if_some!(table, "Grid X", properties.grid_x);
    add_row_if_some!(table, "Grid Y", properties.grid_y);
    add_row_if_some!(
        table,
        "Forecast Zone",
        get_zone_from_url(properties.forecast_zone.clone())
    );
    add_row_if_some!(
        table,
        "County Zone",
        get_zone_from_url(properties.county.clone())
    );
    add_row_if_some!(
        table,
        "Fire Weather Zone",
        get_zone_from_url(properties.fire_weather_zone.clone())
    );
    add_row_if_some!(table, "Time Zone", properties.time_zone);
    add_row_if_some!(
        table,
        "Radar Station",
        get_zone_from_url(properties.radar_station.clone())
    );

    Ok(table)
}
