# NOAA Weather CLI Documentation

`noaa-weather` targets NOAA Weather API 3.11.0. These flags are global: every structured command accepts them, before or after the subcommand.

| Flag | Values | Default | Meaning |
| :--- | :--- | :--- | :--- |
| `-f`, `--format` | `table`, `json` | `table` | `table` draws box-drawing tables for a terminal; `json` is pretty JSON. |
| `--json` | — | off | An alias for `--format json`. Passing both fails. |
| `--color` | `auto`, `always`, `never` | `auto` | `auto` colors only when the destination is a terminal and `NO_COLOR` is unset or empty. `always` writes escapes even into a file or a pipe. |
| `--width` | a number of columns | `COLUMNS`, else the terminal, else 100 | Wrapping width, never narrower than 40. `--width 0` never wraps, for piping into `less -S`. |
| `--time-zone` | `auto`, `source`, an IANA name | `auto` | `auto` is this machine's zone; `source` keeps the UTC offset NOAA sent; a name such as `America/Detroit` shows every timestamp there. |
| `-o`, `--output` | a path, or `-` | standard output | Writes to an atomically replaced file; `-` selects standard output explicitly. |

Table output is wrapped whether or not anyone is watching: written to a file or a pipe with no `COLUMNS` set, it wraps to 100 columns, because there is no terminal to measure. Pass `--width 0` when a machine is reading, which turns wrapping off entirely and lets every line run as long as its content; `--json` is the better answer when the reader is a program.

Text and JSON output always ends in exactly one newline.

Office briefing PDFs and weather-story images are binary. Their download commands require `--output <PATH>`, reject any format but `table`, reject `--output -`, reject empty responses, and never write binary bytes to standard output.

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
