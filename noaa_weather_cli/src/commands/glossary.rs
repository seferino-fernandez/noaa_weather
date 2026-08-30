use anyhow::{Context as _, Result};
use noaa_weather_client::apis::{configuration::Configuration, glossary};

use crate::{Cli, tables, utils::format::write_output};

/// Fetches and renders the NWS glossary.
pub async fn handle_command(cli: Cli, config: &Configuration) -> Result<()> {
    let result = glossary::get_glossary(config)
        .await
        .context("getting NWS glossary")?;
    let content = if cli.json {
        serde_json::to_string_pretty(&result)?
    } else {
        tables::glossary::create_glossary_table(&result).to_string()
    };
    write_output(cli.output.as_deref(), &content)
}
