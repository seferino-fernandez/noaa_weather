use serde::{Deserialize, Serialize};

use super::{JsonLdContext, PaginationInfo};

/// Metadata for one NOAA Weather Radio transmitter.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RadioTransmitter {
    /// JSON-LD context, present on direct transmitter responses.
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<Box<JsonLdContext>>,
    /// Canonical transmitter URL.
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// JSON-LD transmitter type.
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Identifier of the transmitter data set.
    #[serde(rename = "setId", skip_serializing_if = "Option::is_none")]
    pub set_id: Option<String>,
    /// Transmitter call sign.
    #[serde(rename = "callSign", skip_serializing_if = "Option::is_none")]
    pub call_sign: Option<String>,
    /// Transmitter frequency as supplied by the service.
    #[serde(
        rename = "transmitterFrequency",
        skip_serializing_if = "Option::is_none"
    )]
    pub frequency: Option<String>,
    /// Transmitter site name.
    #[serde(rename = "siteName", skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,
    /// Transmitter site city.
    #[serde(rename = "siteCity", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Transmitter site state.
    #[serde(rename = "siteState", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// SAME codes covered by the transmitter, preserving order and duplicates.
    #[serde(rename = "sameCodes", default)]
    pub same_codes: Vec<String>,
    /// County codes covered by the transmitter, preserving order and duplicates.
    #[serde(default)]
    pub counties: Vec<String>,
}

/// A paginated collection of NOAA Weather Radio transmitters.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RadioTransmitterCollection {
    /// JSON-LD context supplied with the response.
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<Box<JsonLdContext>>,
    /// Transmitters mapped from the JSON-LD graph.
    #[serde(rename = "@graph", default)]
    pub transmitters: Vec<RadioTransmitter>,
    /// Pagination links when more transmitters are available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationInfo>,
}

#[cfg(test)]
mod tests {
    use super::{RadioTransmitter, RadioTransmitterCollection};

    #[test]
    fn direct_transmitter_defaults_lists() {
        let transmitter: RadioTransmitter = serde_json::from_str(
            r#"{"@id":"https://api.weather.gov/radio/KAAA","callSign":"KAAA"}"#,
        )
        .unwrap();
        assert_eq!(transmitter.call_sign.as_deref(), Some("KAAA"));
        assert!(transmitter.same_codes.is_empty());
        assert!(transmitter.counties.is_empty());
    }

    #[test]
    fn collection_preserves_order_duplicates_and_optional_pagination() {
        let collection: RadioTransmitterCollection = serde_json::from_str(
            r#"{
                "@graph": [{
                    "callSign":"KAAA",
                    "sameCodes":["004013","004013","004019"],
                    "counties":["AZC013","AZC013"]
                }, {"callSign":"KBBB"}],
                "pagination":{"next":"https://api.weather.gov/radio?cursor=next"}
            }"#,
        )
        .unwrap();
        assert_eq!(
            collection.transmitters[0].same_codes[0..2],
            ["004013", "004013"]
        );
        assert_eq!(collection.transmitters[0].counties, ["AZC013", "AZC013"]);
        assert_eq!(
            collection.transmitters[1].call_sign.as_deref(),
            Some("KBBB")
        );
        assert!(collection.pagination.is_some());

        let without_pagination: RadioTransmitterCollection =
            serde_json::from_str(r#"{"@graph":[]}"#).unwrap();
        assert_eq!(without_pagination.pagination, None);
    }
}
