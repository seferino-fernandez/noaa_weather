# noaa_weather_summary

Format-independent human summaries of NOAA weather.gov API responses.

A [`noaa_weather_client`] response is a faithful copy of what the API sent.
Most readers want much less. This crate decides *what a person should see*,
so the CLI, an MCP server and any other front end show the same summary and
differ only in how they draw it.

## Rule of thumb

Default output answers **what**, **where**, **when** and **how bad**.

- Identifiers appear only when the reader needs one for the next command, such
  as an alert id or a zone code.
- Provenance is always omitted: URLs, `@context`, `@id`, `@type`, geometry and
  anything else that describes the response rather than the weather.
- Every omitted property is listed in `Summarize::OMITTED` with a reason, and
  `coverage_gaps` reports any property that is neither shown nor listed.

## Meaning versus appearance

This crate decides meaning. A `Summarize` impl chooses which properties become
facts, table columns or prose, what they are called, and how urgent they are
(`Emphasis`). The `Value` constructors classify raw fields: blank text is
`Missing`, a non-finite number is `Invalid`, the last path segment of a NOAA
URL is an `Identifier`.

Renderers decide appearance. `render::markdown` and `render::plain` share one
set of formatting rules (`N/A` for missing values, `YYYY-MM-DD HH:MM -04:00`
for timestamps in the offset NOAA sent them, humanized byte sizes) and differ
only in markup. A terminal front end can add its own renderer with colors and
box drawing without touching a single meaning decision.

## Summaries

The `alerts` module implements `Summarize` for the `/alerts` family: a
`FeatureCollection<Alert>` becomes one table with a row per alert, a
`Feature<Alert>` becomes a fact sheet followed by its description and
instruction, `ActiveAlertCounts` becomes three totals plus per-area and
per-region tables, and `AlertEventTypes` becomes a one-column list. Severity
sets the emphasis: `Extreme` and `Severe` are `Danger`, `Moderate` is
`Warning`, `Minor` is `Info`.

```rust,no_run
use noaa_weather_client::FeatureCollection;
use noaa_weather_client::models::Alert;
use noaa_weather_summary::render::{RenderOptions, markdown};
use noaa_weather_summary::{Summarize, SummaryOptions};

let alerts: FeatureCollection<Alert> = serde_json::from_str(&std::fs::read_to_string("alerts.json")?)?;
let summary = alerts.summarize(&SummaryOptions::default());
println!("{}", markdown::render(&summary, &RenderOptions::default()));
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Building a summary by hand

```rust
use noaa_weather_summary::render::{RenderOptions, markdown};
use noaa_weather_summary::{Emphasis, Fact, Section, Summary, Value};

let summary = Summary::new("Tornado Warning")
    .subtitle("Tornado Warning issued for Wayne County until 8:15 PM EDT")
    .emphasis(Emphasis::Danger)
    .push(Section::Facts {
        heading: None,
        facts: vec![
            Fact::new("Area", Some("areaDesc"), Value::text(Some("Wayne County, MI"))),
            Fact::new("Severity", Some("severity"), Value::text(Some("Extreme")))
                .with_emphasis(Emphasis::Danger),
            Fact::new("Ends", Some("ends"), Value::text(None)),
        ],
    })
    .note("More alerts available");

let text = markdown::render(&summary, &RenderOptions::default());
assert!(text.starts_with("# Tornado Warning\n"));
assert!(text.contains("- **Severity:** **Extreme**"));
assert!(text.contains("- **Ends:** N/A"));
```

[`noaa_weather_client`]: https://crates.io/crates/noaa_weather_client
