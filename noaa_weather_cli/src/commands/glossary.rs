use anyhow::Result;
use noaa_weather_client::Client;

use crate::output::Output;

/// Fetches and renders the NWS glossary.
pub async fn handle_command(output: &Output, client: &Client) -> Result<()> {
    output
        .show("getting NWS glossary", client.glossary().terms())
        .await
}
