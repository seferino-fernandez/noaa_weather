use anyhow::Result;
use clap::Args;
use noaa_weather_client::Client;

use crate::commands::Run;
use crate::output::Output;

/// Fetches and renders the NWS glossary.
#[derive(Args, Debug, Clone)]
pub struct GlossaryCommand {}

impl Run for GlossaryCommand {
    async fn run(&self, client: &Client, output: &Output) -> Result<()> {
        output.show(client.glossary().terms()).await
    }
}
