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

## Exit codes

A script can tell these apart and act on them differently: 4 is worth retrying in a minute, 3 means NOAA answered and said no, and 2 means the command line was wrong and running it again will not help.

| Code | Meaning | Examples |
| :--- | :--- | :--- |
| `0` | The command produced its output. | |
| `1` | Something failed that none of the codes below describe. | A response body that did not decode; a `Content-Type` the endpoint does not allow; a redirect that could not be followed; an empty binary payload. |
| `2` | A value on the command line, or in an environment variable standing in for one, was rejected — including a request argv alone makes impossible. | A malformed `LAT,LON`, zone id, or interval; an unknown flag; `--timeout 0s`; a `NOAA_WEATHER_BASE_URL` that is not an absolute `http(s)` URL; a binary download without `--output <PATH>`, or with `--output -`; `--json` on a command whose response is a PDF. |
| `3` | NOAA answered with a non-success HTTP status. | `404` for an alert that has expired; `429` when rate limited; `503` when the API is down. |
| `4` | The request never got a complete answer. | Connection refused, DNS failure, TLS failure, or `--timeout` elapsing. |
| `5` | The output destination could not take the bytes on this machine. | An `--output` path whose parent does not exist, is not a directory, or cannot be written; a target that is not a regular file; a failed atomic replace. |

A body that arrived and failed to decode is `1`, not `3`. `3` is meant to be readable as "NOAA refused this request, and the status says why"; a truncated or renamed response is not something a different argument would fix, and folding it into `3` would make `[ $? -eq 3 ]` quietly wrong.

The line between `2` and `5` is whether the machine could ever have satisfied the command. `--json` on a briefing download and a binary download aimed at a terminal fail identically everywhere, so they are `2`; only a filesystem that said no is `5`. Neither implies a request was made — the destination is validated before NOAA is asked, so `5` means "look at the disk", not "the fetch was wasted".

Writing to a pipe that closes early — `noaa-weather alerts list | head` — is `0`, not `5`.

## The error line under `--json`

With `--format json` or `--json`, a failure writes exactly one newline-terminated JSON object to standard error, and nothing else. Standard error parses as a whole, so `2>` a file and read it, or `2>&1 >/dev/null | jq`.

```json
{"error":{"kind":"noaa","message":"fetching active alert count: HTTP 503 Service Unavailable response from https://api.weather.gov/alerts/active/count: ...","status":503,"url":"https://api.weather.gov/alerts/active/count","retry_after":30,"correlation_id":"4e283b2","request_id":"01a0543a-814e-7050-8cf6-9ad000424790","problem":{"type":"...","title":"Service Unavailable","status":503,"detail":"...","instance":"...","correlationId":"4e283b2"}}}
```

`kind` and `message` are always present. Everything else is omitted when the failure does not carry it, rather than written as `null`: a connection refused has no `status`, no `url`, and no `problem`.

| Member | Meaning |
| :--- | :--- |
| `kind` | `noaa` (exit 3), `network` (exit 4), `output` (exit 5), or `internal` (exit 1). There are exactly these four, one per exit code, so `$?` and `kind` cannot disagree. |
| `message` | The same text the human-readable line carries, causes included: the operation the command was performing, then what went wrong. |
| `status` | The HTTP status NOAA answered with. |
| `url` | The URL that failed, after redirects. |
| `retry_after` | NOAA's `Retry-After`, in whole seconds. |
| `correlation_id`, `request_id` | NOAA's `X-Correlation-Id` and `X-Request-Id` headers, which the NWS asks for when reporting a problem. |
| `problem` | NOAA's RFC 7807 body, passed through byte for byte: every member it sent, with the types it sent them as. Nested rather than flattened, because its `status` and `title` are the server's account of the failure and the members beside it are this program's; merging them would silently drop one of the two `status` values. On a `400` it carries `parameterErrors`, naming the parameter and the patterns it failed — the only machine-readable account of what was wrong with the value. |

**Exit 2 never writes a JSON line.** Most usage errors are reported by clap, which prints its own message and exits before `--format` has any effect. The rest this program detects itself, after `--format` has been parsed — a base URL that is not a URL, a value the client rejects, `--json` on a binary download — and those *could* carry a line. They deliberately do not: "exit 2 never writes JSON" is worth more to a consumer as an absolute than as a rule with exceptions to memorise. They write the human-readable `noaa-weather: ...` line whatever `--format` says, so a program reading this contract should treat exit 2 as unparseable by design rather than as a missing line.

Note the direction of that rule. A failure is classified by what went wrong, and only then asked whether its code carries a `kind`. It does not work the other way round: "there is no `usage` kind, so this must be an `output` failure" is how `--json` on a PDF was once mis-filed as exit 5.

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
