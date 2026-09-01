# Output architecture

The CLI separates response meaning, default presentation and destination writing.

Typed NOAA responses retain their serialization contract. `--json` serializes those responses directly and never invokes default-presentation policy. Default output instead crosses one `DefaultPresenter` seam before any destination transaction begins.

`DefaultPresenter` owns the in-process policy shared by every default presentation:

- the system time zone is resolved once when default output is configured, with an explicit UTC fallback;
- missing generic values and blank text render as `N/A`;
- a present malformed timestamp returns a typed, contextual presentation error;
- a document is completely constructed before writing, so presentation failures emit no partial output;
- finite unitless measurements retain their number without an invented unit;
- non-finite measurements and impossible scalar values remain invalid rather than masquerading as missing;
- semantic fallbacks choose the first usable value, not merely the first populated wrapper;
- unit labels, precision, wind ordering, byte thresholds and resource-identifier extraction remain local to presentation policy.

Response-specific renderers are implementation beneath this seam. Their private operations may preserve domain meaning such as aviation Zulu time, TAF `Unchanged` and `Not reported` states, and radar summaries. Those meanings are not flattened into generic missing state.

The output module owns the `DefaultPresenter` instance. Production constructs it from one `jiff::tz::TimeZone`; tests inject a fixed zone. The presenter performs no process time-zone lookup itself. JSON configuration constructs no presenter at all.

Tests exercise policy through `DefaultPresenter::present` or the complete `Output::show` path. Raw helper-level tests are avoided because callers and tests should cross the same seam.
