# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/seferino-fernandez/noaa_weather/releases/tag/noaa_weather_client-v0.1.0) - 2025-06-19

### Added

- *(CLI)* Updated more CLI commands with correct flag name
- *(CLI)* Made as much CLI args use enums
- *(gridpoints, auth)* Removed all auth config, refactored gripoints CLI.
- *(Aviation)* Refactored Aviation CLI to have typed args.
- *(Alerts)* Refactored Alerts CLI to have typed args.
- Updated docs
- *(cli)* Added table output for the Zones CLI
- *(stations)* Updated Stations CLI to have table formatting
- *(radar)* Updated Radar CLI to have table formatting
- *(cli)* Created table output for Products CLI
- *(cli)* Updated Offices CLI to output tables
- *(cli)* Updated Aviation CLI to output tables
- *(cli)* Added table output for Alerts CLI
- *(cli)* Added comfy_table crate to provide table formatting for CLI output
- Created text products CLI
- Created Aviation CLI API along with CLI tests
- Added Gridpoints CLI
- Added CLI for Radar API
- Implement CLI command for Zones API
- Initial NOAA Weather Client and CLI commit

### Other

- Added release-plz config and Github actions
- *(client,cli)* Improved variable and function naming
- *(client)* Removed unneeded extern crate stuff
- *(client)* Applied Clippy fixes
- Removed extra quality control structs and defined one
- Removed unused models (Icons)
- Removed unused crates and models (Glossary)
- *(zones)* Refactor the Zones API to have a better interface and documentation
- *(stations)* Updated the Stations API to have better documentation and better interface
- *(radar)* Updated the Radar API to have better documentation and better interface
- *(products)* Updated the Products API to have better documentation and better interface
- Improved Points API with better parameter support and API documentation
- Improved Offices API with better parameter support and API documentation
- Improved Gridpoints API with better parameter support and API documentation
- Improved Aviation API with better parameter support and API documentation
- Improved Alerts API with better parameter support and API documentation
- *(deps)* Updated dependencies - clap, anyhow, asset_cmd
- Removed custom weather API
