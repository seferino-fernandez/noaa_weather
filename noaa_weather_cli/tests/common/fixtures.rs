//! Captured NOAA responses, shared by every suite that needs a body.
//!
//! These are the same files `noaa_weather_client`'s tests replay, so a
//! response shape only has to be captured once, and
//! `noaa_weather_client/tests/fixtures/capture.sh` refreshes all of them.
//!
//! A handful are trimmed, because the glossary is over three thousand terms
//! and the transmitter list is five hundred: the rule and the number of
//! elements dropped are recorded in a `.trim` note beside each one.

/// The `Content-Type` the client requires for GeoJSON responses.
pub const GEO_JSON: &str = "application/geo+json";

/// The `Content-Type` the client requires for JSON-LD responses.
pub const JSON_LD: &str = "application/ld+json";

/// The `Content-Type` the client requires for radio broadcast scripts.
pub const SSML: &str = "application/ssml+xml";

/// The `Content-Type` the client requires for a single TAF.
pub const IWXXM: &str = "application/vnd.wmo.iwxxm+xml";

/// The `Content-Type` an office briefing document arrives with.
pub const PDF: &str = "application/pdf";

/// The `Content-Type` a weather-story image arrives with.
pub const PNG: &str = "image/png";

macro_rules! fixture {
    ($(#[$note:meta])* $name:ident = $path:literal) => {
        $(#[$note])*
        pub const $name: &str =
            include_str!(concat!("../../../noaa_weather_client/tests/fixtures/", $path));
    };
}

fixture!(
    /// A collection of five alerts.
    ALERT_LIST = "alerts/list.json"
);
fixture!(
    /// The single alert whose id is [`ALERT_ID`].
    ALERT_SINGLE = "alerts/single.json"
);
fixture!(
    /// Active alert counts by area, region, and zone.
    ALERT_COUNT = "alerts/count.json"
);
fixture!(
    /// The event types NOAA recognizes.
    ALERT_TYPES = "alerts/types.json"
);

fixture!(
    /// Current SIGMETs and AIRMETs.
    SIGMETS = "aviation/sigmets.json"
);
fixture!(
    /// One SIGMET.
    SIGMET = "aviation/sigmet.json"
);
fixture!(
    /// The current CWAs for one CWSU.
    CWAS = "aviation/cwas.json"
);
fixture!(
    /// One CWA.
    CWA = "aviation/cwa.json"
);
fixture!(
    /// Metadata for one Center Weather Service Unit.
    CWSU = "aviation/cwsu.json"
);

fixture!(
    /// The NWS glossary, trimmed to five terms.
    GLOSSARY = "glossary/terms.json"
);

fixture!(
    /// Raw forecast layers for one grid cell.
    GRIDPOINT = "gridpoints/gridpoint.json"
);
fixture!(
    /// The twelve-hour forecast for one grid cell.
    GRIDPOINT_FORECAST = "gridpoints/forecast.json"
);
fixture!(
    /// The hourly forecast for one grid cell.
    GRIDPOINT_HOURLY = "gridpoints/hourly.json"
);
fixture!(
    /// Observation stations serving one grid cell.
    GRIDPOINT_STATIONS = "gridpoints/stations.json"
);

fixture!(
    /// Metadata for one forecast office.
    OFFICE = "offices/office.json"
);
fixture!(
    /// One office's news headlines.
    OFFICE_HEADLINES = "offices/headlines.json"
);
fixture!(
    /// One office news headline.
    OFFICE_HEADLINE = "offices/headline.json"
);
fixture!(
    /// An office briefing slot with nothing in it, which is what PSR
    /// returned when this was captured and what most offices return most of
    /// the time.
    OFFICE_BRIEFING = "offices/briefing.json"
);
fixture!(
    /// One office's active weather stories.
    OFFICE_WEATHER_STORIES = "offices/weather_stories.json"
);

fixture!(
    /// Metadata for one geographic point.
    POINT = "points/point.json"
);

fixture!(
    /// Five recent text products.
    PRODUCT_LIST = "products/list.json"
);
fixture!(
    /// One text product, body included.
    PRODUCT = "products/product.json"
);
fixture!(
    /// Every issuance location, as an id-to-name map.
    PRODUCT_LOCATIONS = "products/locations.json"
);
fixture!(
    /// Every product type and its code.
    PRODUCT_TYPES = "products/types.json"
);
fixture!(
    /// Recent products of one type, trimmed to five.
    PRODUCT_TYPE = "products/type.json"
);
fixture!(
    /// The issuance locations for one product type.
    PRODUCT_TYPE_LOCATIONS = "products/type_locations.json"
);
fixture!(
    /// Recent products of one type from one location.
    PRODUCT_TYPE_LOCATION = "products/type_location.json"
);
fixture!(
    /// The product types one location issues.
    PRODUCT_LOCATION_TYPES = "products/location_types.json"
);
fixture!(
    /// The latest product of one type from one location.
    PRODUCT_LATEST = "products/latest.json"
);

fixture!(
    /// One radar server.
    RADAR_SERVER = "radar/server.json"
);
fixture!(
    /// Every radar server.
    RADAR_SERVERS = "radar/servers.json"
);
fixture!(
    /// One radar station.
    RADAR_STATION = "radar/station.json"
);
fixture!(
    /// Every radar station, trimmed to three.
    RADAR_STATIONS = "radar/stations.json"
);
fixture!(
    /// Five entries from one radar data queue.
    RADAR_QUEUE = "radar/queue.json"
);
fixture!(
    /// One radar station's alarms, of which it had none.
    RADAR_ALARMS = "radar/alarms.json"
);
fixture!(
    /// SPGDS host telemetry, trimmed to two hosts.
    RADAR_SPGDS = "radar/spgds.json"
);

fixture!(
    /// A page of NOAA Weather Radio transmitters, trimmed to five.
    RADIO_TRANSMITTERS = "radio/transmitters.json"
);
fixture!(
    /// One transmitter's metadata.
    RADIO_TRANSMITTER = "radio/transmitter.json"
);
fixture!(
    /// The transmitters serving one county zone, trimmed to five.
    RADIO_COUNTY = "radio/county.json"
);
fixture!(
    /// One transmitter's broadcast script, as SSML.
    RADIO_BROADCAST = "radio/broadcast.xml"
);
fixture!(
    /// The broadcast script for one point, as SSML.
    RADIO_POINT = "radio/point.xml"
);

fixture!(
    /// Five observation stations.
    STATION_LIST = "stations/list.json"
);
fixture!(
    /// One observation station.
    STATION = "stations/single.json"
);
fixture!(
    /// Five observations from one station.
    OBSERVATIONS = "stations/observations.json"
);
fixture!(
    /// One station's latest observation.
    LATEST_OBSERVATION = "stations/latest.json"
);
fixture!(
    /// One station's TAF metadata, trimmed to three forecasts.
    TAFS = "stations/tafs.json"
);
fixture!(
    /// One TAF, as IWXXM.
    TAF = "stations/taf.xml"
);

fixture!(
    /// A collection of zones.
    ZONE_LIST = "zones/list.json"
);
fixture!(
    /// One zone.
    ZONE = "zones/single.json"
);
fixture!(
    /// One zone's text forecast.
    ZONE_FORECAST = "zones/forecast.json"
);
fixture!(
    /// Recent observations from one zone's stations.
    ZONE_OBSERVATIONS = "zones/observations.json"
);
fixture!(
    /// The observation stations in one zone.
    ZONE_STATIONS = "zones/stations.json"
);

/// A body standing in for an office briefing PDF.
///
/// Not captured: the client checks the response's `Content-Type` and hands
/// the bytes to the destination without looking at them, so a real PDF in
/// the repository would prove nothing a header does not.
pub const BRIEFING_PDF: &str = "%PDF-1.7\n%%EOF\n";

/// A body standing in for a weather-story image, for the same reason.
pub const STORY_IMAGE: &str = "not a real PNG, and nothing decodes it";

/// The id of the alert in [`ALERT_SINGLE`], so a request and its reply agree.
pub const ALERT_ID: &str = "urn:oid:2.49.0.1.840.0.af7e0442df8bfed7953cb5cae6e4661304cc1f49.001.1";
