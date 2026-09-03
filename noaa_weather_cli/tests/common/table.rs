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
    ALERT_COUNT, ALERT_ID, ALERT_LIST, ALERT_SINGLE, ALERT_TYPES, GEO_JSON, JSON_LD,
};

/// What the request's query string has to be.
#[derive(Clone, Copy)]
pub enum Query {
    /// Exactly this text; `""` means the request carries no query at all.
    Exact(&'static str),
    /// A query whose value the table cannot spell, because it is resolved
    /// against the clock. The shared runner only requires that one arrived;
    /// the invocation's own test in `tests/alerts.rs` checks the values.
    Clock,
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
    /// A JSON pointer to the collection the payload's items live in.
    pub payload: &'static str,
    /// Keys NOAA must populate on the first item, when there is one.
    pub keys: &'static [&'static str],
    /// Whether an empty collection fails the test.
    ///
    /// Opt-in, because most of these filters legitimately match nothing: a
    /// quiet day has no severe alerts in Arizona, and a suite that goes red
    /// on quiet weather is a suite somebody turns off.
    pub non_empty: bool,
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
});

/// The unfiltered listing, which reaches back over past alerts and so always
/// has something in it.
const ALERTS_LISTED: Live = Live::Check(Expectation {
    payload: "/features",
    keys: ALERT_KEYS,
    non_empty: true,
});

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
pub const FAMILIES: &[Family] = &[Family {
    name: "alerts",
    invocations: ALERTS,
}];

/// Families the suites do not drive yet.
///
/// `tests/completeness.rs` exempts the leaves of every name listed here.
/// Emptying this list is what "the suites cover the whole CLI" means; adding
/// to it is going backwards.
pub const FAMILIES_AWAITING_COVERAGE: &[&str] = &[
    "aviation",
    "glossary",
    "gridpoints",
    "offices",
    "points",
    "products",
    "radar",
    "radio",
    "stations",
    "zones",
];

/// The `alerts` family.
pub const ALERTS: &[Invocation] = &[
    Invocation {
        command: &["alerts", "active"],
        arguments: &[],
        path: "/alerts/active",
        query: Query::Exact(""),
        body: ALERT_LIST,
        media: GEO_JSON,
        live: ALERTS_FOUND,
    },
    Invocation {
        command: &["alerts", "active"],
        arguments: &["--area", "AZ", "--severity", "Severe"],
        path: "/alerts/active",
        query: Query::Exact("area=AZ&severity=Severe"),
        body: ALERT_LIST,
        media: GEO_JSON,
        live: ALERTS_FOUND,
    },
    Invocation {
        command: &["alerts", "active"],
        arguments: &["--point", "39.7456,-97.0892"],
        path: "/alerts/active",
        query: Query::Exact("point=39.7456%2C-97.0892"),
        body: ALERT_LIST,
        media: GEO_JSON,
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
        live: ALERTS_FOUND,
    },
    Invocation {
        command: &["alerts", "area"],
        arguments: &["--area", "AZ"],
        path: "/alerts/active/area/AZ",
        query: Query::Exact(""),
        body: ALERT_LIST,
        media: GEO_JSON,
        live: ALERTS_FOUND,
    },
    Invocation {
        command: &["alerts", "count"],
        arguments: &[],
        path: "/alerts/active/count",
        query: Query::Exact(""),
        body: ALERT_COUNT,
        media: JSON_LD,
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
        }),
    },
    Invocation {
        command: &["alerts", "marine-region"],
        arguments: &["--marine-region", "PI"],
        path: "/alerts/active/region/PI",
        query: Query::Exact(""),
        body: ALERT_LIST,
        media: GEO_JSON,
        live: ALERTS_FOUND,
    },
    Invocation {
        command: &["alerts", "zone"],
        arguments: &["--zone-id", "AZC013"],
        path: "/alerts/active/zone/AZC013",
        query: Query::Exact(""),
        body: ALERT_LIST,
        media: GEO_JSON,
        live: ALERTS_FOUND,
    },
    Invocation {
        command: &["alerts", "list"],
        arguments: &[],
        path: "/alerts",
        query: Query::Exact(""),
        body: ALERT_LIST,
        media: GEO_JSON,
        live: ALERTS_LISTED,
    },
    Invocation {
        command: &["alerts", "list"],
        arguments: &["--status", "actual", "--limit", "5"],
        path: "/alerts",
        query: Query::Exact("status=actual&limit=5"),
        body: ALERT_LIST,
        media: GEO_JSON,
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
        query: Query::Clock,
        body: ALERT_LIST,
        media: GEO_JSON,
        live: ALERTS_LISTED,
    },
    Invocation {
        command: &["alerts", "alert"],
        arguments: &["--id", ALERT_ID],
        path: "/alerts/urn:oid:2.49.0.1.840.0.af7e0442df8bfed7953cb5cae6e4661304cc1f49.001.1",
        query: Query::Exact(""),
        body: ALERT_SINGLE,
        media: GEO_JSON,
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
        live: Live::Check(Expectation {
            // A vocabulary list that defaults to empty when it fails to
            // decode, so an empty one is drift rather than quiet weather.
            payload: "/eventTypes",
            keys: &[],
            non_empty: true,
        }),
    },
];
