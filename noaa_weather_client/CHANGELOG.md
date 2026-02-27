# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.1.0](https://github.com/seferino-fernandez/noaa_weather/compare/v0.1.8...v1.1.0)

_26 February 2026_

### Added

- Add radio broadcast API module behind 'radio' feature flag
- Add get_latest_product_by_type_and_location endpoint
- Add Feature-Flags header to zone stations endpoint
- Add Feature-Flags header to station endpoints and cursor to observations
- [**breaking**] Update gridpoints stations to use feature_flags instead of cursor
- [**breaking**] Change points API to use separate latitude/longitude parameters
- [**breaking**] Split GridpointForecast into Gridpoint12hForecast and GridpointHourlyForecast
- Add PointType, AstronomicalData, NoaaWeatherRadio models and Point fields
- Add pagination field to ObservationCollection models
- Add provider, subProvider, distance, bearing fields to ObservationStation
- Add scope, code, language, web, eventCode fields to Alert model
- Add PQE and PQW forecast office ID variants
- Add API key authentication support to Configuration

### Documented

- Updated documentation with the latest v3.6.0 changes

### Fixed

- Removed broken radio quick-xml tests
- Update basic_usage example for new points API latitude/longitude parameters

### Other

- Format, lint fixes, and extract X-Api-Key constant

## [0.1.8](https://github.com/seferino-fernandez/noaa_weather/compare/v0.1.7...v0.1.8)

_24 January 2026_

### Fixed

- Fixed assert_cmd deprecated function errors from Clippy
- Fixed Cargo Clippy formatting and derive errors

### Updated dependencies

- Update deps for both client and cli

## [0.1.7](https://github.com/seferino-fernandez/noaa_weather/compare/v0.1.6...v0.1.7)

_03 September 2025_

### Updated dependencies

- Update dependencies and patch slab to fix CVE-2025-55159

## [0.1.6](https://github.com/seferino-fernandez/noaa_weather/compare/noaa_weather_client-v0.1.5...noaa_weather_client-v0.1.6)

_04 August 2025_

### Fixed

- Update documentation to use new structs and removed limit param from ActiveAlerts API since its not supported

## [0.1.5](https://github.com/seferino-fernandez/noaa_weather/compare/noaa_weather_client-v0.1.4...noaa_weather_client-v0.1.5)

_21 June 2025_

### Fixed

- _(cli, client)_ Clippy lints - removed single chars, and redundant clones, and more

## [0.1.4](https://github.com/seferino-fernandez/noaa_weather/compare/noaa_weather_client-v0.1.3...noaa_weather_client-v0.1.4) - 2025-06-21

### Fixed

- _(client)_ Cleared up clippy lint suggestions

## [0.1.3](https://github.com/seferino-fernandez/noaa_weather/compare/noaa_weather_client-v0.1.2...noaa_weather_client-v0.1.3) - 2025-06-21

### Added

- Use rustls for linux and added Homebrew docs

### Other

- Revert versions to 0.1.2 to re-enable release-plz

## [0.1.0](https://github.com/seferino-fernandez/noaa_weather/releases/tag/noaa_weather_client-v0.1.0) - 2025-06-19

### Added

- _(CLI)_ Updated more CLI commands with correct flag name
- _(CLI)_ Made as much CLI args use enums
- _(gridpoints, auth)_ Removed all auth config, refactored gripoints CLI.
- _(Aviation)_ Refactored Aviation CLI to have typed args.
- _(Alerts)_ Refactored Alerts CLI to have typed args.
- Updated docs
- _(cli)_ Added table output for the Zones CLI
- _(stations)_ Updated Stations CLI to have table formatting
- _(radar)_ Updated Radar CLI to have table formatting
- _(cli)_ Created table output for Products CLI
- _(cli)_ Updated Offices CLI to output tables
- _(cli)_ Updated Aviation CLI to output tables
- _(cli)_ Added table output for Alerts CLI
- _(cli)_ Added comfy_table crate to provide table formatting for CLI output
- Created text products CLI
- Created Aviation CLI API along with CLI tests
- Added Gridpoints CLI
- Added CLI for Radar API
- Implement CLI command for Zones API
- Initial NOAA Weather Client and CLI commit

### Other

- Updated documentation by breaking down README to their own workspaces
- Added release-plz config and Github actions
- _(client,cli)_ Improved variable and function naming
- _(client)_ Removed unneeded extern crate stuff
- _(client)_ Applied Clippy fixes
- Removed extra quality control structs and defined one
- Removed unused models (Icons)
- Removed unused crates and models (Glossary)
- _(zones)_ Refactor the Zones API to have a better interface and documentation
- _(stations)_ Updated the Stations API to have better documentation and better interface
- _(radar)_ Updated the Radar API to have better documentation and better interface
- _(products)_ Updated the Products API to have better documentation and better interface
- Improved Points API with better parameter support and API documentation
- Improved Offices API with better parameter support and API documentation
- Improved Gridpoints API with better parameter support and API documentation
- Improved Aviation API with better parameter support and API documentation
- Improved Alerts API with better parameter support and API documentation
- _(deps)_ Updated dependencies - clap, anyhow, asset_cmd
- Removed custom weather API
