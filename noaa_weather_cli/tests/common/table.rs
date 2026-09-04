//! The command invocations both integration suites drive.
//!
//! One table, two runners: the hermetic suite serves [`Invocation::body`] from
//! a mock server and checks the request it received against
//! [`Invocation::path`] and [`Invocation::query`], and the live runner sends
//! the same argument list at NOAA and checks the payload that comes back. A
//! command added to only one of them is a command the other silently stopped
//! covering, so both read from here.
//!
//! `tests/completeness.rs` walks clap's command tree and fails when a leaf
//! subcommand has no entry below.

use super::fixtures::{
    ALERT_COUNT, ALERT_ID, ALERT_LIST, ALERT_SINGLE, ALERT_TYPES, BRIEFING_PDF, CWA, CWAS, CWSU,
    GEO_JSON, GLOSSARY, GRIDPOINT, GRIDPOINT_FORECAST, GRIDPOINT_HOURLY, GRIDPOINT_STATIONS, IWXXM,
    JSON_LD, LATEST_OBSERVATION, OBSERVATIONS, OFFICE, OFFICE_BRIEFING, OFFICE_HEADLINE,
    OFFICE_HEADLINES, OFFICE_WEATHER_STORIES, PDF, PNG, POINT, PRODUCT, PRODUCT_LATEST,
    PRODUCT_LIST, PRODUCT_LOCATION_TYPES, PRODUCT_LOCATIONS, PRODUCT_TYPE, PRODUCT_TYPE_LOCATION,
    PRODUCT_TYPE_LOCATIONS, PRODUCT_TYPES, RADAR_ALARMS, RADAR_QUEUE, RADAR_SERVER, RADAR_SERVERS,
    RADAR_SPGDS, RADAR_STATION, RADAR_STATIONS, RADIO_BROADCAST, RADIO_COUNTY, RADIO_POINT,
    RADIO_TRANSMITTER, RADIO_TRANSMITTERS, SIGMET, SIGMETS, SSML, STATION, STATION_LIST,
    STORY_IMAGE, TAF, TAFS, ZONE, ZONE_FORECAST, ZONE_LIST, ZONE_OBSERVATIONS, ZONE_STATIONS,
};

/// What the request's query string has to be.
#[derive(Clone, Copy)]
pub enum Query {
    /// Exactly this text; `""` means the request carries no query at all.
    Exact(&'static str),
    /// A query carrying at least one value resolved against the clock, which
    /// the table cannot spell as a literal.
    ///
    /// It still spells everything else. `matching` in `runner.rs` checks the
    /// parameter names and their order, and `check_ages` checks the
    /// timestamps against the window the run happened in — every invocation
    /// with this query, not just the one family that used to carry a
    /// bespoke test. Accepting any non-empty query, which is what this
    /// variant used to mean, would let `--start 6h` send `start=banana` and
    /// pass.
    Clock(Ages),
}

/// The shape of a query whose timestamps are resolved when the command runs.
#[derive(Clone, Copy)]
pub struct Ages {
    /// Every parameter, in the order the client sends them.
    ///
    /// Order included, because a reordered query string is how a rewritten
    /// serializer announces itself.
    pub parameters: &'static [&'static str],
    /// The parameters resolved from a relative age, and that age in hours.
    ///
    /// Each has to land in the window between the two clock readings that
    /// bracket the run — which pins the arithmetic, not just the shape.
    pub relative: &'static [(&'static str, i64)],
}

/// What the live runner requires of an invocation's `--json` payload.
///
/// The keys named here are the ones the curated models declare `Option` or
/// default: a required field that NOAA renames or drops already fails to
/// decode, so exit 0 covers it. An optional one silently arrives as `null`,
/// and nothing but an assertion like this notices.
///
/// Nothing here names `type`. The CLI writes that string itself on serialize
/// (`geo/feature_collection.rs`, `geo/feature.rs`) rather than reading it off
/// the response, so an assertion on it would hold whatever NOAA sent.
pub struct Expectation {
    /// A JSON pointer to the collection the payload's items live in, or to
    /// the value itself when the response is a single object.
    pub payload: &'static str,
    /// Keys NOAA must populate on the first item, when there is one.
    pub keys: &'static [&'static str],
    /// Whether an empty collection fails the test.
    ///
    /// Opt-in, because most of these filters legitimately match nothing: a
    /// quiet day has no severe alerts in Arizona, no CWAs over Albuquerque,
    /// and no alarms on a healthy radar. A suite that goes red on quiet
    /// weather is a suite somebody turns off.
    pub non_empty: bool,
    /// Document-absolute JSON pointers that must resolve to exactly this
    /// text.
    ///
    /// Everything above asks whether a value is *there*. This asks whether
    /// it is the *right* value, which is the only thing that notices NOAA
    /// answering the URL we asked for with a document about something else.
    /// Pointers are absolute rather than relative to [`Expectation::payload`]
    /// because that field already means three things depending on what it
    /// points at, and a fourth would be worse.
    pub equals: &'static [(&'static str, &'static str)],
}

/// What the live runner does with an invocation.
pub enum Live {
    /// Send it at NOAA and hold the reply to this.
    Check(Expectation),
    /// Hermetic only, for the stated reason.
    Skip(&'static str),
}

/// The optional keys NOAA populated on all 500 alerts in a live sample, so a
/// null in one of them is drift rather than a quiet field.
///
/// `headline` is deliberately absent: it was null on 53 of those 500.
const ALERT_KEYS: &[&str] = &[
    "description",
    "affectedZones",
    "geocode",
    "parameters",
    "web",
];

/// A collection of alerts, which any one filter may legitimately find empty.
const ALERTS_FOUND: Live = Live::Check(Expectation {
    payload: "/features",
    keys: ALERT_KEYS,
    non_empty: false,
    equals: &[],
});

/// The unfiltered listing, which reaches back over past alerts and so always
/// has something in it.
const ALERTS_LISTED: Live = Live::Check(Expectation {
    payload: "/features",
    keys: ALERT_KEYS,
    non_empty: true,
    equals: &[],
});

/// A GeoJSON collection that always has members, with nothing to say about
/// their optional fields.
const fn features(non_empty: bool) -> Live {
    Live::Check(Expectation {
        payload: "/features",
        keys: &[],
        non_empty,
        equals: &[],
    })
}

/// A JSON-LD collection under `@graph`.
const fn graph(non_empty: bool) -> Live {
    Live::Check(Expectation {
        payload: "/@graph",
        keys: &[],
        non_empty,
        equals: &[],
    })
}

/// One command line, and the NOAA request running it should produce.
pub struct Invocation {
    /// The leaf subcommand path, spelled as clap spells it.
    pub command: &'static [&'static str],
    /// Arguments appended after the subcommand path.
    pub arguments: &'static [&'static str],
    /// The request path the client should ask for, query string excluded.
    pub path: &'static str,
    /// The query string the client should send.
    pub query: Query,
    /// The fixture the hermetic runner replies with.
    pub body: &'static str,
    /// The `Content-Type` that fixture must carry; the client checks it.
    pub media: &'static str,
    /// Whether the response is binary and so needs `--output <PATH>`.
    ///
    /// The runners append a temporary path, because the table cannot spell
    /// one and these commands refuse to write bytes to a terminal.
    pub binary: bool,
    /// Substrings the default presentation must write to standard output.
    ///
    /// `hermetic()` checks these, not `live()`, because the fixture is fixed:
    /// the assertion can name an exact value instead of hoping NOAA sends
    /// something recognizable, and it runs offline on every `cargo test`.
    ///
    /// Two kinds belong here and both are worth one entry. A column header
    /// says the presenter drew the table it was supposed to draw. A value
    /// out of the fixture says data reached a cell — which a table with
    /// every header right and every row empty would fail, and which nothing
    /// else in the workspace checks for eight of these families.
    ///
    /// Empty only for the binary downloads, which write no bytes to stdout.
    pub renders: &'static [&'static str],
    /// What the live runner does with it.
    pub live: Live,
}

impl Invocation {
    /// The whole argument list, subcommand path first.
    pub fn argv(&self) -> Vec<&'static str> {
        self.command.iter().chain(self.arguments).copied().collect()
    }

    /// The argument list as one space-separated string, for failure messages.
    pub fn display(&self) -> String {
        self.argv().join(" ")
    }
}

/// A top-level command name and every leaf invocation covering it.
pub struct Family {
    /// The top-level subcommand, spelled as clap spells it.
    pub name: &'static str,
    /// Every invocation the suites drive for that family.
    pub invocations: &'static [Invocation],
}

/// Every family the suites cover.
pub const FAMILIES: &[Family] = &[
    Family {
        name: "alerts",
        invocations: ALERTS,
    },
    Family {
        name: "aviation",
        invocations: AVIATION,
    },
    Family {
        name: "glossary",
        invocations: GLOSSARY_FAMILY,
    },
    Family {
        name: "gridpoints",
        invocations: GRIDPOINTS,
    },
    Family {
        name: "offices",
        invocations: OFFICES,
    },
    Family {
        name: "points",
        invocations: POINTS,
    },
    Family {
        name: "products",
        invocations: PRODUCTS,
    },
    Family {
        name: "radar",
        invocations: RADAR,
    },
    Family {
        name: "radio",
        invocations: RADIO,
    },
    Family {
        name: "stations",
        invocations: STATIONS,
    },
    Family {
        name: "zones",
        invocations: ZONES,
    },
];

/// Families the suites do not drive yet.
///
/// `tests/completeness.rs` exempts the leaves of every name listed here.
/// Emptying this list is what "the suites cover the whole CLI" means; adding
/// to it is going backwards.
pub const FAMILIES_AWAITING_COVERAGE: &[&str] = &[];

/// The `alerts` family.
pub const ALERTS: &[Invocation] = &[
    Invocation {
        command: &["alerts", "active"],
        arguments: &[],
        path: "/alerts/active",
        query: Query::Exact(""),
        body: ALERT_LIST,
        media: GEO_JSON,
        binary: false,
        renders: &["Watches, warnings, and advisories"],
        live: ALERTS_FOUND,
    },
    Invocation {
        command: &["alerts", "active"],
        arguments: &["--area", "AZ", "--severity", "Severe"],
        path: "/alerts/active",
        query: Query::Exact("area=AZ&severity=Severe"),
        body: ALERT_LIST,
        media: GEO_JSON,
        binary: false,
        renders: &["Watches, warnings, and advisories"],
        live: ALERTS_FOUND,
    },
    Invocation {
        command: &["alerts", "active"],
        arguments: &["--point", "39.7456,-97.0892"],
        path: "/alerts/active",
        query: Query::Exact("point=39.7456%2C-97.0892"),
        body: ALERT_LIST,
        media: GEO_JSON,
        binary: false,
        renders: &["Watches, warnings, and advisories"],
        live: ALERTS_FOUND,
    },
    Invocation {
        // The lower-case half is the point: the parser upper-cases zone ids
        // and joins the list into one comma-separated parameter.
        command: &["alerts", "active"],
        arguments: &["--zone", "AZC013,azz540"],
        path: "/alerts/active",
        query: Query::Exact("zone=AZC013%2CAZZ540"),
        body: ALERT_LIST,
        media: GEO_JSON,
        binary: false,
        renders: &["Watches, warnings, and advisories"],
        live: ALERTS_FOUND,
    },
    Invocation {
        command: &["alerts", "area"],
        arguments: &["--area", "AZ"],
        path: "/alerts/active/area/AZ",
        query: Query::Exact(""),
        body: ALERT_LIST,
        media: GEO_JSON,
        binary: false,
        renders: &["Watches, warnings, and advisories"],
        live: ALERTS_FOUND,
    },
    Invocation {
        command: &["alerts", "count"],
        arguments: &[],
        path: "/alerts/active/count",
        query: Query::Exact(""),
        body: ALERT_COUNT,
        media: JSON_LD,
        binary: false,
        renders: &["Active alerts", "Total"],
        live: Live::Check(Expectation {
            // `total`, `land` and `marine` are plain numbers the models
            // require; the three breakdowns default to empty, so they are
            // where a rename would go unnoticed. `count.json` carries 48
            // areas against 5 regions, enough that an empty one is drift
            // rather than a quiet day, and `zones` is broader still at 2935
            // but too large to be a useful probe.
            payload: "/areas",
            keys: &[],
            non_empty: true,
            equals: &[],
        }),
    },
    Invocation {
        command: &["alerts", "marine-region"],
        arguments: &["--marine-region", "PI"],
        path: "/alerts/active/region/PI",
        query: Query::Exact(""),
        body: ALERT_LIST,
        media: GEO_JSON,
        binary: false,
        renders: &["Watches, warnings, and advisories"],
        live: ALERTS_FOUND,
    },
    Invocation {
        command: &["alerts", "zone"],
        arguments: &["--zone-id", "AZC013"],
        path: "/alerts/active/zone/AZC013",
        query: Query::Exact(""),
        body: ALERT_LIST,
        media: GEO_JSON,
        binary: false,
        renders: &["Watches, warnings, and advisories"],
        live: ALERTS_FOUND,
    },
    Invocation {
        command: &["alerts", "list"],
        arguments: &[],
        path: "/alerts",
        query: Query::Exact(""),
        body: ALERT_LIST,
        media: GEO_JSON,
        binary: false,
        renders: &["Watches, warnings, and advisories"],
        live: ALERTS_LISTED,
    },
    Invocation {
        command: &["alerts", "list"],
        arguments: &["--status", "actual", "--limit", "5"],
        path: "/alerts",
        query: Query::Exact("status=actual&limit=5"),
        body: ALERT_LIST,
        media: GEO_JSON,
        binary: false,
        renders: &["Watches, warnings, and advisories"],
        live: ALERTS_LISTED,
    },
    Invocation {
        // Relative ages become absolute timestamps, which is the conversion
        // in this family most likely to drift; NOAA answers a malformed one
        // with a 400. `relative_ages_become_absolute_timestamps` in
        // `tests/alerts.rs` checks what the two parameters actually hold.
        command: &["alerts", "list"],
        arguments: &["--start", "6h", "--end", "1h", "--limit", "5"],
        path: "/alerts",
        query: Query::Clock(Ages {
            parameters: &["start", "end", "limit"],
            relative: &[("start", 6), ("end", 1)],
        }),
        body: ALERT_LIST,
        media: GEO_JSON,
        binary: false,
        renders: &["Watches, warnings, and advisories"],
        live: ALERTS_LISTED,
    },
    Invocation {
        command: &["alerts", "alert"],
        arguments: &["--id", ALERT_ID],
        path: "/alerts/urn:oid:2.49.0.1.840.0.af7e0442df8bfed7953cb5cae6e4661304cc1f49.001.1",
        query: Query::Exact(""),
        body: ALERT_SINGLE,
        media: GEO_JSON,
        binary: false,
        renders: &["Special Weather Statement", "NWS Grand Rapids MI"],
        live: Live::Skip(
            "a hardcoded id rots. NOAA answers the id this test carried on \
             `main`, urn:oid:2.49.0.1.840.0.dcc6cd9527d1f8732519ea87f13d3810e9ef672c.001.1, \
             with `404 Alert Does Not Exist`. `test_alerts_command_get_success` \
             covers the live path with an id it resolves at run time instead.",
        ),
    },
    Invocation {
        command: &["alerts", "types"],
        arguments: &[],
        path: "/alerts/types",
        query: Query::Exact(""),
        body: ALERT_TYPES,
        media: JSON_LD,
        binary: false,
        renders: &["Alert event types", "Event type"],
        live: Live::Check(Expectation {
            // A vocabulary list that defaults to empty when it fails to
            // decode, so an empty one is drift rather than quiet weather.
            payload: "/eventTypes",
            keys: &[],
            non_empty: true,
            equals: &[],
        }),
    },
];

/// The `aviation` family.
pub const AVIATION: &[Invocation] = &[
    Invocation {
        // The date and sequence are the ones in `aviation/cwa.json`. NOAA
        // stops serving a CWA within days, so the live path is
        // `a_cwa_is_fetched_by_a_sequence_resolved_at_run_time`.
        command: &["aviation", "cwa"],
        arguments: &[
            "--cwsu-id",
            "ZAB",
            "--date",
            "2026-09-02",
            "--sequence",
            "106",
        ],
        path: "/aviation/cwsus/ZAB/cwas/2026-09-02/106",
        query: Query::Exact(""),
        body: CWA,
        media: GEO_JSON,
        binary: false,
        renders: &["ZAB", "Issue"],
        live: Live::Skip(
            "a CWA is addressed by a date and a sequence NOAA stops serving \
             within days; `a_cwa_is_fetched_by_a_sequence_resolved_at_run_time` \
             resolves both from the listing instead",
        ),
    },
    Invocation {
        command: &["aviation", "cwas"],
        arguments: &["--cwsu-id", "ZAB"],
        path: "/aviation/cwsus/ZAB/cwas",
        query: Query::Exact(""),
        body: CWAS,
        media: GEO_JSON,
        // A CWSU with no current advisories is an ordinary quiet day.
        binary: false,
        renders: &["ZAB", "Issue"],
        live: features(false),
    },
    Invocation {
        command: &["aviation", "cwsu"],
        arguments: &["--cwsu-id", "ZAB"],
        path: "/aviation/cwsus/ZAB",
        query: Query::Exact(""),
        body: CWSU,
        media: JSON_LD,
        binary: false,
        renders: &["ZAB", "Address"],
        live: Live::Check(Expectation {
            // An office's own record, which is never empty and never
            // changes; `nwsRegion` and `email` are the optional halves.
            payload: "/id",
            keys: &[],
            non_empty: true,
            equals: &[("/id", "ZAB")],
        }),
    },
    Invocation {
        command: &["aviation", "sigmet"],
        arguments: &["--atsu", "KKCI", "--issued", "2026-08-31T00:30:00Z"],
        path: "/aviation/sigmets/KKCI/2026-08-31/0030",
        query: Query::Exact(""),
        body: SIGMET,
        media: GEO_JSON,
        binary: false,
        renders: &["KKCI"],
        live: Live::Skip(
            "a SIGMET is addressed by its issue minute, which NOAA stops \
             serving; `a_sigmet_is_fetched_by_an_issue_time_resolved_at_run_time` \
             resolves one from the listing instead",
        ),
    },
    Invocation {
        command: &["aviation", "sigmets"],
        arguments: &[],
        path: "/aviation/sigmets",
        query: Query::Exact(""),
        body: SIGMETS,
        media: GEO_JSON,
        binary: false,
        renders: &["KKCI"],
        live: features(true),
    },
    Invocation {
        command: &["aviation", "sigmets"],
        arguments: &["--atsu", "KKCI"],
        path: "/aviation/sigmets",
        query: Query::Exact("atsu=KKCI"),
        body: SIGMETS,
        media: GEO_JSON,
        binary: false,
        renders: &["KKCI"],
        live: features(false),
    },
    Invocation {
        command: &["aviation", "sigmets"],
        arguments: &["--atsu", "KKCI", "--date", "2026-08-31"],
        path: "/aviation/sigmets",
        query: Query::Exact("date=2026-08-31&atsu=KKCI"),
        body: SIGMETS,
        media: GEO_JSON,
        binary: false,
        renders: &["KKCI"],
        live: features(false),
    },
    Invocation {
        command: &["aviation", "sigmets"],
        arguments: &[
            "--atsu",
            "KKCI",
            "--start",
            "2026-08-31T00:01:00+00:00",
            "--end",
            "2026-08-31T01:55:00+00:00",
        ],
        path: "/aviation/sigmets",
        query: Query::Exact(
            "start=2026-08-31T00%3A01%3A00Z&end=2026-08-31T01%3A55%3A00Z&atsu=KKCI",
        ),
        body: SIGMETS,
        media: GEO_JSON,
        binary: false,
        renders: &["KKCI"],
        live: features(false),
    },
    Invocation {
        command: &["aviation", "sigmets"],
        arguments: &["--sequence", "52C"],
        path: "/aviation/sigmets",
        query: Query::Exact("sequence=52C"),
        body: SIGMETS,
        media: GEO_JSON,
        binary: false,
        renders: &["KKCI"],
        live: features(false),
    },
    Invocation {
        command: &["aviation", "sigmets"],
        arguments: &["--atsu", "KKCI", "--start", "6h"],
        path: "/aviation/sigmets",
        query: Query::Clock(Ages {
            parameters: &["start", "atsu"],
            relative: &[("start", 6)],
        }),
        body: SIGMETS,
        media: GEO_JSON,
        binary: false,
        renders: &["KKCI"],
        live: features(false),
    },
];

/// The `glossary` family, whose one leaf is the top-level command itself.
pub const GLOSSARY_FAMILY: &[Invocation] = &[Invocation {
    command: &["glossary"],
    arguments: &[],
    path: "/glossary",
    query: Query::Exact(""),
    body: GLOSSARY,
    media: JSON_LD,
    binary: false,
    renders: &["Term", "Definition"],
    live: Live::Check(Expectation {
        // A vocabulary of three thousand terms that does not change with the
        // weather, so an empty one is drift.
        payload: "/glossary",
        keys: &["term", "definition"],
        non_empty: true,
        equals: &[],
    }),
}];

/// The `gridpoints` family.
pub const GRIDPOINTS: &[Invocation] = &[
    Invocation {
        command: &["gridpoints", "gridpoint"],
        arguments: &["PSR/159,58"],
        path: "/gridpoints/PSR/159,58",
        query: Query::Exact(""),
        body: GRIDPOINT,
        media: GEO_JSON,
        binary: false,
        renders: &["Gridpoint TOP/31,80"],
        live: Live::Check(Expectation {
            payload: "/properties/temperature/values",
            keys: &[],
            non_empty: true,
            equals: &[],
        }),
    },
    Invocation {
        command: &["gridpoints", "forecast"],
        arguments: &["PSR/159,58"],
        path: "/gridpoints/PSR/159,58/forecast",
        query: Query::Exact(""),
        body: GRIDPOINT_FORECAST,
        media: GEO_JSON,
        binary: false,
        renders: &["Forecast", "Updated"],
        live: Live::Check(Expectation {
            payload: "/properties/periods",
            keys: &[],
            non_empty: true,
            equals: &[],
        }),
    },
    Invocation {
        // The lower-case office code is the point: the parser upper-cases it
        // before the path is built.
        command: &["gridpoints", "forecast-hourly"],
        arguments: &["psr/159,58", "--units", "si"],
        path: "/gridpoints/PSR/159,58/forecast/hourly",
        query: Query::Exact(""),
        body: GRIDPOINT_HOURLY,
        media: GEO_JSON,
        binary: false,
        renders: &["Hourly forecast"],
        live: Live::Check(Expectation {
            payload: "/properties/periods",
            keys: &[],
            non_empty: true,
            equals: &[],
        }),
    },
    Invocation {
        command: &["gridpoints", "stations"],
        arguments: &["PSR/159,58", "--limit", "10"],
        path: "/gridpoints/PSR/159,58/stations",
        query: Query::Exact("limit=10"),
        body: GRIDPOINT_STATIONS,
        media: GEO_JSON,
        binary: false,
        renders: &["Station ID", "KMYZ", "27.4 mi", "71 °"],
        live: features(true),
    },
];

/// The `offices` family.
pub const OFFICES: &[Invocation] = &[
    Invocation {
        command: &["offices", "metadata"],
        arguments: &["--id", "PSR"],
        path: "/offices/PSR",
        query: Query::Exact(""),
        body: OFFICE,
        media: JSON_LD,
        binary: false,
        renders: &["PSR", "Name"],
        live: Live::Check(Expectation {
            payload: "/id",
            keys: &[],
            non_empty: true,
            equals: &[("/id", "PSR")],
        }),
    },
    Invocation {
        // A regional headquarters, which is not a forecast office and was
        // rejected before `OfficeId` stopped restricting itself to the
        // known list.
        command: &["offices", "metadata"],
        arguments: &["--id", "WRH"],
        path: "/offices/WRH",
        query: Query::Exact(""),
        body: OFFICE,
        media: JSON_LD,
        binary: false,
        renders: &["PSR", "Name"],
        live: Live::Check(Expectation {
            payload: "/id",
            keys: &[],
            non_empty: true,
            equals: &[("/id", "WRH")],
        }),
    },
    Invocation {
        command: &["offices", "headlines"],
        arguments: &["--id", "PSR"],
        path: "/offices/PSR/headlines",
        query: Query::Exact(""),
        body: OFFICE_HEADLINES,
        media: JSON_LD,
        binary: false,
        // An office with no news is a quiet week, not a broken route.
        renders: &["Title"],
        live: graph(false),
    },
    Invocation {
        command: &["offices", "headline"],
        arguments: &[
            "--id",
            "PSR",
            "--headline-id",
            "8efe6a38d9d74abb80d62e71fba34189",
        ],
        path: "/offices/PSR/headlines/8efe6a38d9d74abb80d62e71fba34189",
        query: Query::Exact(""),
        body: OFFICE_HEADLINE,
        media: JSON_LD,
        binary: false,
        renders: &["Title"],
        live: Live::Skip(
            "a headline id expires with the headline; \
             `a_headline_is_fetched_by_an_id_resolved_at_run_time` reads a \
             current one out of the listing",
        ),
    },
    Invocation {
        command: &["offices", "briefing"],
        arguments: &["--id", "PSR"],
        path: "/offices/PSR/briefing",
        query: Query::Exact(""),
        body: OFFICE_BRIEFING,
        media: JSON_LD,
        binary: false,
        renders: &["Download", "Starts"],
        live: Live::Check(Expectation {
            // `briefing` is null whenever the office has nothing active,
            // which is most of the time, so this only asserts the envelope
            // arrived and decoded.
            payload: "/@context",
            keys: &[],
            non_empty: false,
            equals: &[],
        }),
    },
    Invocation {
        // A regional headquarters on one of the newer office routes.
        // `OfficeId` accepts any well-formed code, and the question this
        // asks is whether NOAA serves `briefing` for a unit that is not a
        // forecast office — which checking `metadata --id WRH` alone does
        // not answer.
        command: &["offices", "briefing"],
        arguments: &["--id", "WRH"],
        path: "/offices/WRH/briefing",
        query: Query::Exact(""),
        body: OFFICE_BRIEFING,
        media: JSON_LD,
        binary: false,
        renders: &["Download", "Starts"],
        live: Live::Check(Expectation {
            payload: "/@context",
            keys: &[],
            non_empty: false,
            equals: &[],
        }),
    },
    Invocation {
        command: &["offices", "briefing-download"],
        arguments: &["--id", "PSR", "--document-id", "not-a-real-document"],
        path: "/offices/PSR/briefing/download/not-a-real-document",
        query: Query::Exact(""),
        body: BRIEFING_PDF,
        media: PDF,
        binary: true,
        renders: &[],
        live: Live::Skip(
            "a briefing document id only exists while the briefing does; \
             `office_media_downloads_to_files_or_says_there_was_none` \
             resolves one and reports it when there is none",
        ),
    },
    Invocation {
        command: &["offices", "briefing-download-latest"],
        arguments: &["--id", "PSR"],
        path: "/offices/PSR/briefing/download/latest",
        query: Query::Exact(""),
        body: BRIEFING_PDF,
        media: PDF,
        binary: true,
        renders: &[],
        live: Live::Skip(
            "NOAA answers this 404 whenever the office has no active \
             briefing, which it did not when this was written; \
             `office_media_downloads_to_files_or_says_there_was_none` drives \
             it only when `offices briefing` says there is one",
        ),
    },
    Invocation {
        command: &["offices", "weather-stories"],
        arguments: &["--id", "PSR"],
        path: "/offices/PSR/weatherstories",
        query: Query::Exact(""),
        body: OFFICE_WEATHER_STORIES,
        media: JSON_LD,
        binary: false,
        renders: &["Alt Text", "Title"],
        live: Live::Check(Expectation {
            payload: "/stories",
            keys: &[],
            non_empty: false,
            equals: &[],
        }),
    },
    Invocation {
        command: &["offices", "weather-story-image"],
        arguments: &["--id", "PSR", "--story-id", "not-a-real-story"],
        path: "/offices/PSR/weatherstories/download/not-a-real-story",
        query: Query::Exact(""),
        body: STORY_IMAGE,
        media: PNG,
        binary: true,
        renders: &[],
        live: Live::Skip(
            "a weather-story id expires with the story; \
             `office_media_downloads_to_files_or_says_there_was_none` \
             resolves one from the listing",
        ),
    },
];

/// The `points` family.
pub const POINTS: &[Invocation] = &[Invocation {
    command: &["points", "metadata"],
    arguments: &["39.7456,-97.0892"],
    path: "/points/39.7456,-97.0892",
    query: Query::Exact(""),
    body: POINT,
    media: GEO_JSON,
    binary: false,
    renders: &["Grid cell", "TOP/32,81"],
    live: Live::Check(Expectation {
        // The two that matter to a reader and that a rename would hide:
        // `forecastOffice` was already caught once by the completeness test.
        payload: "/properties",
        keys: &["gridId", "forecastOffice", "relativeLocation"],
        non_empty: true,
        equals: &[],
    }),
}];

/// The `products` family.
pub const PRODUCTS: &[Invocation] = &[
    Invocation {
        command: &["products", "list"],
        arguments: &[],
        path: "/products",
        query: Query::Exact("limit=500"),
        body: PRODUCT_LIST,
        media: JSON_LD,
        binary: false,
        renders: &["Product Code"],
        live: graph(true),
    },
    Invocation {
        command: &["products", "list"],
        arguments: &["--location-ids", "PSR"],
        path: "/products",
        query: Query::Exact("location=PSR&limit=500"),
        body: PRODUCT_LIST,
        media: JSON_LD,
        binary: false,
        renders: &["Product Code"],
        live: graph(true),
    },
    Invocation {
        command: &["products", "list"],
        arguments: &["--wmo-ids", "SRUS55"],
        path: "/products",
        query: Query::Exact("wmoid=SRUS55&limit=500"),
        body: PRODUCT_LIST,
        media: JSON_LD,
        binary: false,
        renders: &["Product Code"],
        live: graph(false),
    },
    Invocation {
        command: &["products", "list"],
        arguments: &[
            "--location-ids",
            "PSR",
            "--product-type-codes",
            "AFD",
            "--start-time",
            "2d",
            "--limit",
            "2",
        ],
        path: "/products",
        query: Query::Clock(Ages {
            // `--start-time 2d` is two days, which the runner checks as 48
            // hours; `location` and `type` are the singular names NOAA uses
            // for the comma-joined lists.
            parameters: &["location", "start", "type", "limit"],
            relative: &[("start", 48)],
        }),
        body: PRODUCT_LIST,
        media: JSON_LD,
        binary: false,
        renders: &["Product Code"],
        live: graph(true),
    },
    Invocation {
        command: &["products", "metadata"],
        arguments: &["--id", "dcfd78a0-561b-423e-9fd6-889455b8c535"],
        path: "/products/dcfd78a0-561b-423e-9fd6-889455b8c535",
        query: Query::Exact(""),
        body: PRODUCT,
        media: JSON_LD,
        binary: false,
        renders: &["Issuance Time", "UFUS42"],
        live: Live::Skip(
            "a product id ages out of NOAA's window; \
             `a_product_is_fetched_by_an_id_resolved_at_run_time` reads a \
             current one out of the listing",
        ),
    },
    Invocation {
        command: &["products", "locations"],
        arguments: &[],
        path: "/products/locations",
        query: Query::Exact(""),
        body: PRODUCT_LOCATIONS,
        media: JSON_LD,
        binary: false,
        renders: &["Location ID", "Location Name"],
        live: Live::Check(Expectation {
            // A map of every issuance location, which does not change with
            // the weather.
            payload: "/locations",
            keys: &[],
            non_empty: true,
            equals: &[],
        }),
    },
    Invocation {
        command: &["products", "types"],
        arguments: &[],
        path: "/products/types",
        query: Query::Exact(""),
        body: PRODUCT_TYPES,
        media: JSON_LD,
        binary: false,
        renders: &["Product Code"],
        live: graph(true),
    },
    Invocation {
        command: &["products", "type"],
        arguments: &["--type-id", "AFD"],
        path: "/products/types/AFD",
        query: Query::Exact(""),
        body: PRODUCT_TYPE,
        media: JSON_LD,
        binary: false,
        renders: &["Product Code"],
        live: graph(true),
    },
    Invocation {
        command: &["products", "types-by-location"],
        arguments: &["--type-id", "AFD", "--location-id", "LWX"],
        path: "/products/types/AFD/locations/LWX",
        query: Query::Exact(""),
        body: PRODUCT_TYPE_LOCATION,
        media: JSON_LD,
        binary: false,
        renders: &["Product Code"],
        live: graph(true),
    },
    Invocation {
        command: &["products", "locations-by-type"],
        arguments: &["--type-id", "AFD"],
        path: "/products/types/AFD/locations",
        query: Query::Exact(""),
        body: PRODUCT_TYPE_LOCATIONS,
        media: JSON_LD,
        binary: false,
        renders: &["Location ID"],
        live: Live::Check(Expectation {
            payload: "/locations",
            keys: &[],
            non_empty: true,
            equals: &[],
        }),
    },
    Invocation {
        command: &["products", "products-by-location"],
        arguments: &["--location-id", "PSR"],
        path: "/products/locations/PSR/types",
        query: Query::Exact(""),
        body: PRODUCT_LOCATION_TYPES,
        media: JSON_LD,
        binary: false,
        renders: &["Product Code"],
        live: graph(true),
    },
    Invocation {
        command: &["products", "latest"],
        arguments: &["--type-id", "AFD", "--location-id", "PSR"],
        path: "/products/types/AFD/locations/PSR/latest",
        query: Query::Exact(""),
        body: PRODUCT_LATEST,
        media: JSON_LD,
        binary: false,
        renders: &["Issuance Time", "AFD"],
        live: Live::Check(Expectation {
            // The body is the whole point of a text product, and it is the
            // one member a renamed field would empty without failing the
            // decode.
            payload: "/productText",
            keys: &[],
            non_empty: true,
            equals: &[("/productCode", "AFD")],
        }),
    },
];

/// The `radar` family.
pub const RADAR: &[Invocation] = &[
    Invocation {
        command: &["radar", "data-queue"],
        arguments: &["--host", "rds", "--station", "KIWA"],
        path: "/radar/queues/rds",
        query: Query::Exact("limit=10&station=KIWA"),
        body: RADAR_QUEUE,
        media: JSON_LD,
        binary: false,
        renders: &["Host"],
        live: graph(false),
    },
    Invocation {
        command: &["radar", "server"],
        arguments: &["--id", "ldm1"],
        path: "/radar/servers/ldm1",
        query: Query::Exact(""),
        // Hand-written rather than captured: this one is shared with the
        // model tests, which use it to pin the awkward corners of a radar
        // server's telemetry.
        body: RADAR_SERVER,
        media: JSON_LD,
        binary: false,
        renders: &["Radar Server Status: ldm1"],
        live: Live::Check(Expectation {
            payload: "/id",
            keys: &[],
            non_empty: true,
            equals: &[("/id", "ldm1")],
        }),
    },
    Invocation {
        command: &["radar", "servers"],
        arguments: &[],
        path: "/radar/servers",
        query: Query::Exact(""),
        body: RADAR_SERVERS,
        media: JSON_LD,
        binary: false,
        renders: &["Server"],
        live: graph(true),
    },
    Invocation {
        // Five characters, lower case: NOAA's station list mixes
        // four-character NEXRAD sites with five-character profilers, and the
        // parser upper-cases either.
        command: &["radar", "station"],
        arguments: &["--station-id", "hwpa2"],
        path: "/radar/stations/HWPA2",
        query: Query::Exact(""),
        body: RADAR_STATION,
        media: GEO_JSON,
        binary: false,
        renders: &["Station Information"],
        live: Live::Check(Expectation {
            payload: "/properties",
            keys: &["stationType"],
            non_empty: true,
            equals: &[("/properties/id", "HWPA2")],
        }),
    },
    Invocation {
        command: &["radar", "station-alarms"],
        arguments: &["--station-id", "KABQ"],
        path: "/radar/stations/KABQ/alarms",
        query: Query::Exact(""),
        body: RADAR_ALARMS,
        media: JSON_LD,
        binary: false,
        // A healthy radar has no alarms, and usually does not.
        renders: &["Station ID", "Alarm Time"],
        live: graph(false),
    },
    Invocation {
        command: &["radar", "stations"],
        arguments: &[],
        path: "/radar/stations",
        query: Query::Exact(""),
        body: RADAR_STATIONS,
        media: GEO_JSON,
        binary: false,
        renders: &["Station ID (ICAO)"],
        live: features(true),
    },
    Invocation {
        command: &["radar", "stations"],
        arguments: &["--station-type", "WSR-88D"],
        path: "/radar/stations",
        query: Query::Exact("stationType=WSR-88D"),
        body: RADAR_STATIONS,
        media: GEO_JSON,
        binary: false,
        renders: &["Station ID (ICAO)"],
        live: features(true),
    },
    Invocation {
        command: &["radar", "spgds"],
        arguments: &[],
        path: "/radar/spgds",
        query: Query::Exact(""),
        body: RADAR_SPGDS,
        media: JSON_LD,
        binary: false,
        renders: &["Timestamp"],
        live: graph(true),
    },
    Invocation {
        command: &["radar", "spgds"],
        arguments: &["--published", "2026-08-30T00:00:00Z/2026-08-30T01:00:00Z"],
        path: "/radar/spgds",
        // The offset is spelled out rather than abbreviated to `Z`: an
        // `Interval` serializes its ends the way `jiff` writes them.
        query: Query::Exact(
            "published=2026-08-30T00%3A00%3A00%2B00%3A00%2F2026-08-30T01%3A00%3A00%2B00%3A00",
        ),
        body: RADAR_SPGDS,
        media: JSON_LD,
        binary: false,
        // A one-hour window in the past legitimately matches nothing.
        renders: &["Timestamp"],
        live: graph(false),
    },
    Invocation {
        command: &["radar", "wind-profiler"],
        arguments: &["--id", "HWPA2"],
        path: "/radar/profilers/HWPA2",
        query: Query::Exact(""),
        // Not captured, because nothing answered: see the `Live::Skip`
        // reason below for which ids were tried. The command renders
        // whatever JSON arrives, so an empty envelope is enough to prove the
        // path and the raw-JSON seam.
        body: r#"{"@context":{"@version":"1.1"},"@graph":[]}"#,
        media: JSON_LD,
        binary: false,
        renders: &["@graph"],
        live: Live::Skip(
            "NOAA answers this route with 404. `radar stations \
             --station-type Profiler` lists exactly four — AWPA2, HWPA2, \
             ROCO2 and TLKA2 — and each 404s under /radar/profilers. The \
             spelling matters: `--station-type PROFILER` and `profiler` both \
             come back empty, so a sweep written either way would find \
             nothing and prove nothing.",
        ),
    },
];

/// The `radio` family.
pub const RADIO: &[Invocation] = &[
    Invocation {
        command: &["radio", "transmitters"],
        arguments: &[],
        path: "/radio",
        query: Query::Exact(""),
        body: RADIO_TRANSMITTERS,
        media: JSON_LD,
        binary: false,
        renders: &["Call Sign", "Frequency"],
        live: graph(true),
    },
    Invocation {
        command: &["radio", "transmitter"],
        arguments: &["KEC94"],
        path: "/radio/KEC94",
        query: Query::Exact(""),
        body: RADIO_TRANSMITTER,
        media: JSON_LD,
        binary: false,
        renders: &["Call Sign", "KEC94"],
        live: Live::Check(Expectation {
            payload: "/callSign",
            keys: &[],
            non_empty: true,
            // Non-emptiness would pass on any transmitter NOAA felt like
            // returning; this is the assertion that says it returned the
            // one the path named.
            equals: &[("/callSign", "KEC94")],
        }),
    },
    Invocation {
        command: &["radio", "zone"],
        arguments: &["AZC013"],
        path: "/zones/county/AZC013/radio",
        query: Query::Exact(""),
        body: RADIO_COUNTY,
        media: JSON_LD,
        binary: false,
        renders: &["Call Sign", "Frequency"],
        live: graph(true),
    },
    Invocation {
        command: &["radio", "point"],
        arguments: &["33.4484,-112.0740"],
        path: "/points/33.4484,-112.074/radio",
        query: Query::Exact(""),
        body: RADIO_POINT,
        media: SSML,
        binary: false,
        renders: &["NOAA Weather Radio Broadcast"],
        live: Live::Check(Expectation {
            // A broadcast is SSML, and `--json` emits the decoded document:
            // `p` is the list of spoken paragraphs, and a script with none
            // of them is a script that says nothing.
            payload: "/p",
            keys: &[],
            non_empty: true,
            equals: &[],
        }),
    },
    Invocation {
        command: &["radio", "station"],
        arguments: &["KEC94"],
        path: "/radio/KEC94/broadcast",
        query: Query::Exact(""),
        body: RADIO_BROADCAST,
        media: SSML,
        binary: false,
        renders: &["NOAA Weather Radio Broadcast"],
        live: Live::Check(Expectation {
            payload: "/p",
            keys: &[],
            non_empty: true,
            equals: &[],
        }),
    },
];

/// The `stations` family.
pub const STATIONS: &[Invocation] = &[
    Invocation {
        command: &["stations", "metadata"],
        arguments: &["--id", "KPHX"],
        path: "/stations/KPHX",
        query: Query::Exact(""),
        body: STATION,
        media: GEO_JSON,
        binary: false,
        renders: &["Station ID", "KSLC", "Salt Lake City International Airport"],
        live: Live::Check(Expectation {
            payload: "/properties",
            keys: &["name", "timeZone", "elevation"],
            non_empty: true,
            equals: &[("/properties/stationIdentifier", "KPHX")],
        }),
    },
    Invocation {
        command: &["stations", "list"],
        arguments: &[],
        path: "/stations",
        query: Query::Exact(""),
        body: STATION_LIST,
        media: GEO_JSON,
        binary: false,
        renders: &["Station ID", "0007W", "MesoWest"],
        live: features(true),
    },
    Invocation {
        command: &["stations", "list"],
        arguments: &["--state", "AZ"],
        path: "/stations",
        query: Query::Exact("state=AZ"),
        body: STATION_LIST,
        media: GEO_JSON,
        binary: false,
        renders: &["Station ID", "0007W", "MesoWest"],
        live: features(true),
    },
    Invocation {
        command: &["stations", "list"],
        arguments: &["--state", "AZ", "--limit", "1"],
        path: "/stations",
        query: Query::Exact("state=AZ&limit=1"),
        body: STATION_LIST,
        media: GEO_JSON,
        binary: false,
        renders: &["Station ID", "0007W", "MesoWest"],
        live: features(true),
    },
    Invocation {
        command: &["stations", "list"],
        arguments: &["--id", "KPHX"],
        path: "/stations",
        query: Query::Exact("id=KPHX"),
        body: STATION_LIST,
        media: GEO_JSON,
        binary: false,
        renders: &["Station ID", "0007W", "MesoWest"],
        live: features(true),
    },
    Invocation {
        command: &["stations", "latest-observation"],
        arguments: &["--station-id", "KPHX"],
        path: "/stations/KPHX/observations/latest",
        query: Query::Exact("require_qc=false"),
        body: LATEST_OBSERVATION,
        media: GEO_JSON,
        binary: false,
        renders: &[
            "Station: KSLC - Observation",
            "Clear",
            "Temperature",
            "72 °F",
        ],
        live: Live::Check(Expectation {
            payload: "/properties",
            keys: &["textDescription", "temperature"],
            non_empty: true,
            equals: &[],
        }),
    },
    Invocation {
        command: &["stations", "observations"],
        arguments: &[
            "--station-id",
            "KPHX",
            "--start",
            "6h",
            "--end",
            "1h",
            "--limit",
            "3",
        ],
        path: "/stations/KPHX/observations",
        query: Query::Clock(Ages {
            parameters: &["start", "end", "limit"],
            relative: &[("start", 6), ("end", 1)],
        }),
        body: OBSERVATIONS,
        media: GEO_JSON,
        binary: false,
        renders: &["Station observations", "73 °F", "29.93", "inHg"],
        live: features(true),
    },
    Invocation {
        command: &["stations", "observation"],
        arguments: &["--station-id", "KPHX", "--time", "2026-08-30T12:34:56Z"],
        path: "/stations/KPHX/observations/2026-08-30T12:34:56Z",
        query: Query::Exact(""),
        body: LATEST_OBSERVATION,
        media: GEO_JSON,
        binary: false,
        renders: &[
            "Station: KSLC - Observation",
            "Clear",
            "Temperature",
            "72 °F",
        ],
        live: Live::Skip(
            "NOAA serves this route only for an instant an observation was \
             actually taken at, and only within its retention window; \
             `an_observation_is_fetched_by_a_time_resolved_at_run_time` \
             reads a current one out of the listing",
        ),
    },
    Invocation {
        command: &["stations", "terminal-aerodrome-forecasts"],
        arguments: &["--station-id", "KPHX"],
        path: "/stations/KPHX/tafs",
        query: Query::Exact(""),
        body: TAFS,
        media: JSON_LD,
        binary: false,
        renders: &["Issue Time", "KPHX", "2026-08-30"],
        live: graph(true),
    },
    Invocation {
        command: &["stations", "terminal-aerodrome-forecast"],
        arguments: &["--station-id", "KPHX", "--issued", "2026-08-30T22:54:00Z"],
        path: "/stations/KPHX/tafs/2026-08-30/2254",
        query: Query::Exact(""),
        body: TAF,
        media: IWXXM,
        binary: false,
        renders: &[
            "Terminal Aerodrome Forecast",
            "KPHX",
            "Report state",
            "VCTS",
        ],
        live: Live::Skip(
            "a TAF is addressed by its issue minute, which NOAA stops \
             serving; `test_stations_taf_success` resolves a current one out \
             of the listing",
        ),
    },
];

/// The `zones` family.
pub const ZONES: &[Invocation] = &[
    Invocation {
        command: &["zones", "list"],
        arguments: &[],
        path: "/zones",
        query: Query::Exact(""),
        body: ZONE_LIST,
        media: GEO_JSON,
        binary: false,
        renders: &[
            "Zones",
            "AKC013",
            "Aleutians East",
            "county",
            "AK",
            "America/Anchorage",
            "AFC",
        ],
        live: features(true),
    },
    Invocation {
        command: &["zones", "list"],
        arguments: &["--area", "AZ"],
        path: "/zones",
        query: Query::Exact("area=AZ"),
        body: ZONE_LIST,
        media: GEO_JSON,
        binary: false,
        renders: &["Zones", "AKC013", "Aleutians East", "America/Anchorage"],
        live: features(true),
    },
    Invocation {
        // A single `--type` selects the narrower `/zones/{type}` route and
        // drops the filter, which is the branch in `zones::handle_command`
        // no other test reaches.
        command: &["zones", "list"],
        arguments: &["--type", "land", "--area", "MI"],
        path: "/zones/land",
        query: Query::Exact("area=MI"),
        body: ZONE_LIST,
        media: GEO_JSON,
        binary: false,
        renders: &["Zones", "AKC013", "Aleutians East", "county"],
        live: features(true),
    },
    Invocation {
        command: &["zones", "metadata"],
        arguments: &["--id", "AZZ543", "--type", "public"],
        path: "/zones/public/AZZ543",
        query: Query::Exact(""),
        body: ZONE,
        media: GEO_JSON,
        binary: false,
        renders: &[
            "Zone",
            "UTZ101",
            "Great Salt Lake Desert and Mountains",
            "public",
            "UT",
            "America/Denver",
            "SLC",
            "ARAU1",
        ],
        live: Live::Check(Expectation {
            payload: "/properties",
            keys: &["name", "state", "forecastOffices"],
            non_empty: true,
            equals: &[("/properties/id", "AZZ543")],
        }),
    },
    Invocation {
        command: &["zones", "forecast"],
        arguments: &["--id", "AZZ543", "--type", "public"],
        path: "/zones/public/AZZ543/forecast",
        query: Query::Exact(""),
        body: ZONE_FORECAST,
        media: GEO_JSON,
        binary: false,
        renders: &[
            "Zone forecast",
            "UTZ101",
            "Updated",
            "Day/Night",
            "Today",
            "Sunny in the morning",
        ],
        live: Live::Check(Expectation {
            payload: "/properties/periods",
            keys: &[],
            non_empty: true,
            equals: &[],
        }),
    },
    Invocation {
        command: &["zones", "stations"],
        arguments: &["--id", "AZZ543"],
        path: "/zones/forecast/AZZ543/stations",
        query: Query::Exact(""),
        body: ZONE_STATIONS,
        media: GEO_JSON,
        binary: false,
        renders: &["Station ID", "ARAU1", "ARAGONITE"],
        live: features(true),
    },
    Invocation {
        command: &["zones", "observations"],
        arguments: &["--id", "AZZ543"],
        path: "/zones/forecast/AZZ543/observations",
        query: Query::Exact(""),
        body: ZONE_OBSERVATIONS,
        media: GEO_JSON,
        binary: false,
        renders: &["Zone observations", "KENV", "Clear", "66 °F", "29.90"],
        live: features(true),
    },
];
