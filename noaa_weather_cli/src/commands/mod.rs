pub mod alerts;
pub mod offices;
pub mod points;
pub mod stations;
pub mod weather;
pub mod zones;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Get weather alerts
    Alerts {
        #[command(subcommand)]
        command: Box<alerts::AlertCommands>,
    },
    /// Get NWS office information
    Offices {
        #[command(subcommand)]
        command: Box<offices::OfficeCommands>,
    },
    /// Get point metadata or stations
    Points {
        #[command(subcommand)]
        command: Box<points::PointCommands>,
    },
    /// Get observation station information and data
    Stations {
        #[command(subcommand)]
        command: Box<stations::StationCommands>,
    },
    /// Get weather information
    Weather {
        #[command(subcommand)]
        command: Box<weather::WeatherCommands>,
    },
    /// Get NWS zone information, forecasts, and observations
    Zones {
        #[command(subcommand)]
        command: Box<zones::ZoneCommands>,
    },
}
