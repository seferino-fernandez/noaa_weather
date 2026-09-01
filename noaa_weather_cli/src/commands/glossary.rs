use anyhow::Result;
use noaa_weather_client::{Client, apis::glossary};

use crate::output::Output;

/// Fetches and renders the NWS glossary.
pub async fn handle_command(output: &Output, client: &Client) -> Result<()> {
    output
        .show("getting NWS glossary", glossary::get_glossary(client))
        .await
}
