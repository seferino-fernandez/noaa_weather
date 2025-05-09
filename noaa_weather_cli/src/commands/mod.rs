pub mod alerts;
pub mod aviation;
pub mod gridpoints;
pub mod offices;
pub mod points;
pub mod products;
pub mod radar;
pub mod stations;
pub mod zones;

use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Get weather alerts
    Alerts {
        #[command(subcommand)]
        command: Box<alerts::AlertCommands>,
    },
    /// Get gridpoint forecast data
    Gridpoints {
        #[command(subcommand)]
        command: Box<gridpoints::GridpointCommands>,
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
    /// Get NWS zone information, forecasts, and observations
    Zones {
        #[command(subcommand)]
        command: Box<zones::ZoneCommands>,
    },
    /// Access radar station and server information
    Radar {
        #[command(subcommand)]
        command: Box<radar::RadarCommand>,
    },
    /// Access aviation weather products (CWAs, SIGMETs)
    Aviation {
        #[command(subcommand)]
        command: Box<aviation::AviationCommands>,
    },
    /// Access NWS text product information
    Products {
        #[command(subcommand)]
        command: Box<products::ProductCommands>,
    },
}
