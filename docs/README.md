# NOAA Weather CLI Documentation

`noaa-weather` targets NOAA Weather API 3.11.0. These flags are global: every structured command accepts them, before or after the subcommand.

| Flag | Values | Default | Meaning |
| :--- | :--- | :--- | :--- |
| `-f`, `--format` | `table`, `json` | `table` | `table` draws box-drawing tables for a terminal; `json` is pretty JSON. |
| `--json` | — | off | An alias for `--format json`. Passing both fails. |
| `--color` | `auto`, `always`, `never` | `auto` | `auto` colors only when the destination is a terminal and `NO_COLOR` is unset or empty. `always` writes escapes even into a file or a pipe. |
| `--width` | a number of columns | `COLUMNS`, else the terminal, else 100 | Wrapping width, never narrower than 40. `--width 0` never wraps, for piping into `less -S`. |
| `--units` | `us`, `si` | `us` | The system measurements are converted to before they are shown: `us` is Fahrenheit, miles per hour, feet, miles, inches and inches of mercury; `si` is Celsius, km/h, metres, kilometres, millimetres and hectopascals. NOAA's own `units` request parameter is inert — every response is metric whatever it asks — so the conversion happens here. |
| `--time-zone` | `auto`, `source`, an IANA name | `auto` | `auto` is this machine's zone; `source` keeps the UTC offset NOAA sent; a name such as `America/Detroit` shows every timestamp there. |
| `-o`, `--output` | a path, or `-` | standard output | Writes to an atomically replaced file; `-` selects standard output explicitly. |
| `--base-url` | an `http` or `https` URL | `https://api.weather.gov` | The API root every path is joined onto. The program has one real destination, so this is for testing against a fixture server and for pointing at a local proxy. |
| `--user-agent` | a header value | `noaa-weather/<version> (+<repository>)` | The `User-Agent` sent with every request. NOAA asks callers to identify themselves, ideally with contact information. |
| `--timeout` | a duration such as `30s` or `1m30s` | 30 seconds | The time one attempt may take, redirects and reading the body included. Zero and negative durations are rejected: there is no "wait forever". |
| `--retries` | a whole number | 3 | Attempts for a retryable failure, the first request included. `0` and `1` both mean one attempt and no retry; the request is always sent. |

Each of those four reads an environment variable when the flag is absent: `NOAA_WEATHER_BASE_URL`, `NOAA_WEATHER_USER_AGENT`, `NOAA_WEATHER_TIMEOUT`, `NOAA_WEATHER_RETRIES`. A flag always wins over the variable.

`NOAA_WEATHER_API_KEY` is read too and has no flag, because a process's arguments are readable by other users on the machine. When set, its value is sent as the `X-Api-Key` header to the base URL's origin, is dropped if a redirect leaves that origin, and never appears in debug output. NOAA's API is free and does not normally need a key; the mechanism is experimental and worth validating against NOAA before relying on it.

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
