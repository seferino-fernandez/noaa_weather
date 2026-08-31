use anyhow::Result;
use noaa_weather_client::apis::{configuration::Configuration, glossary};

use crate::output::Output;

/// Fetches and renders the NWS glossary.
pub async fn handle_command(output: &Output, config: &Configuration) -> Result<()> {
    output
        .show("getting NWS glossary", glossary::get_glossary(config))
        .await
}
