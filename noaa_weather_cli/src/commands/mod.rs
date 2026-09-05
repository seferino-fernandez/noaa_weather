pub mod alerts;
pub mod aviation;
pub mod glossary;
pub mod gridpoints;
pub mod offices;
pub mod parse;
pub mod points;
pub mod products;
pub mod radar;
pub mod radio;
pub mod stations;
pub mod zones;

use anyhow::Result;
use clap::Subcommand;
use noaa_weather_client::Client;

use crate::output::Output;

/// Executes one parsed command family through the configured client and output.
pub(crate) trait Run {
    async fn run(&self, client: &Client, output: &Output) -> Result<()>;
}

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
    /// Get the NWS glossary of weather terms.
    Glossary(glossary::GlossaryCommand),
    /// Get NWS office information
    Offices {
        #[command(subcommand)]
        command: Box<offices::OfficeCommands>,
    },
    /// Get metadata for a geographic point
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
    /// Access NOAA Weather Radio broadcast information
    Radio {
        #[command(subcommand)]
        command: Box<radio::RadioCommands>,
    },
}

impl Run for Commands {
    async fn run(&self, client: &Client, output: &Output) -> Result<()> {
        match self {
            Self::Alerts { command } => command.run(client, output).await,
            Self::Gridpoints { command } => command.run(client, output).await,
            Self::Glossary(command) => command.run(client, output).await,
            Self::Offices { command } => command.run(client, output).await,
            Self::Points { command } => command.run(client, output).await,
            Self::Stations { command } => command.run(client, output).await,
            Self::Zones { command } => command.run(client, output).await,
            Self::Radar { command } => command.run(client, output).await,
            Self::Aviation { command } => command.run(client, output).await,
            Self::Products { command } => command.run(client, output).await,
            Self::Radio { command } => command.run(client, output).await,
        }
    }
}
