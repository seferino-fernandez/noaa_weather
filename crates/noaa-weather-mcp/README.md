# NOAA Weather MCP

`noaa-weather-mcp` exposes typed NOAA weather.gov operations as structured JSON tools and native MCP binary content over the Model Context Protocol's stdio transport.

## Installation

Build and install from crates.io:

```bash
cargo install noaa_weather_mcp
```

Or download a matching binary from GitHub Releases:

```bash
cargo binstall noaa_weather_mcp
```

Run `noaa-weather-mcp` without a subcommand to start the server. It communicates exclusively over standard input and standard output, so an MCP client should launch it as a subprocess.

The `--max-response-bytes` option limits one structured JSON result or one raw binary payload and defaults to 1 MiB. `NOAA_WEATHER_MCP_MAX_RESPONSE_BYTES` supplies the same setting through the environment. Office briefing PDFs are returned as embedded blob resources; weather-story graphics are returned as image content. Both use base64 on the MCP wire.

Generate shell completions or man pages with:

```bash
noaa-weather-mcp completions bash
noaa-weather-mcp man ./man-pages
```
