# NOAA Weather CLI Documentation

`noaa-weather` targets NOAA Weather API 3.11.0. Structured commands render concise tables by default. Add the global `--json` flag for pretty JSON, use `--output <PATH>` to write output to an atomically replaced file, or use `--output -` to select standard output explicitly. Text and JSON output always ends in exactly one newline.

Office briefing PDFs and weather-story images are binary. Their download commands require `--output <PATH>`, reject `--json` and `--output -`, reject empty responses, and never write binary bytes to standard output.

## Command guides

- [Alerts](cli/alerts.md)
- [Aviation](cli/aviation.md)
- [Gridpoints](cli/gridpoints.md)
- [Offices](cli/offices.md)
- [Points](cli/points.md)
- [Products](cli/products.md)
- [Radar](cli/radar.md)
- [Radio](cli/radio.md)
- [Stations](cli/stations.md)
- [Zones](cli/zones.md)

The NWS glossary is a top-level command:

```sh
noaa-weather glossary
noaa-weather glossary --json
```
