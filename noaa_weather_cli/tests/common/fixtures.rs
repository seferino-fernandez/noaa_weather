//! Captured NOAA responses, shared by every suite that needs a body.
//!
//! These are the same files `noaa_weather_client`'s tests replay, so a
//! response shape only has to be captured once.

/// The `Content-Type` the client requires for GeoJSON responses.
pub const GEO_JSON: &str = "application/geo+json";

/// The `Content-Type` the client requires for JSON-LD responses.
pub const JSON_LD: &str = "application/ld+json";

/// A collection of five alerts.
pub const ALERT_LIST: &str =
    include_str!("../../../noaa_weather_client/tests/fixtures/alerts/list.json");

/// The single alert whose id is [`ALERT_ID`].
pub const ALERT_SINGLE: &str =
    include_str!("../../../noaa_weather_client/tests/fixtures/alerts/single.json");

/// Active alert counts by area, region, and zone.
pub const ALERT_COUNT: &str =
    include_str!("../../../noaa_weather_client/tests/fixtures/alerts/count.json");

/// The event types NOAA recognizes.
pub const ALERT_TYPES: &str =
    include_str!("../../../noaa_weather_client/tests/fixtures/alerts/types.json");

/// A collection of zones.
pub const ZONE_LIST: &str =
    include_str!("../../../noaa_weather_client/tests/fixtures/zones/list.json");

/// The id of the alert in [`ALERT_SINGLE`], so a request and its reply agree.
pub const ALERT_ID: &str = "urn:oid:2.49.0.1.840.0.af7e0442df8bfed7953cb5cae6e4661304cc1f49.001.1";
