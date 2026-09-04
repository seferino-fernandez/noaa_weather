//! NOAA Weather Radio transmitters and broadcast transcripts: the `/radio`
//! family.
//!
//! Obtain the handle with [`Client::radio`]. Transmitter metadata comes from
//! `/radio`, `/radio/{callSign}`, and `/zones/county/{zoneId}/radio` as
//! [`RadioTransmitter`](crate::models::RadioTransmitter) records. Broadcast
//! scripts come from `/points/{point}/radio` and `/radio/{callSign}/broadcast`
//! as SSML documents decoded into
//! [`RadioBroadcast`](crate::models::RadioBroadcast), whose paragraphs render
//! as plain text via [`Sentence::full_text`](crate::models::Sentence::full_text).
//!
//! ```no_run
//! use noaa_weather_client::{CallSign, Client};
//!
//! # async fn run() -> Result<(), noaa_weather_client::Error> {
//! let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
//! let kec94: CallSign = "KEC94".parse()?;
//! let broadcast = client.radio().broadcast(&kec94).await?;
//! println!("{}", broadcast.lang);
//! # Ok(())
//! # }
//! ```

use serde::{Deserialize, Serialize};

use super::Error;
use crate::client::{Client, http};
use crate::geo::Coordinates;
use crate::ids::{CallSign, Cursor, ZoneId};
use crate::models;

/// Paging for [`Radio::transmitters`].
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default)]
pub struct TransmittersQuery {
    /// Opaque pagination cursor from a previous page, taken from the
    /// `cursor` parameter of the previous response's `pagination.next` link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Cursor>,
}

impl http::QueryParams for TransmittersQuery {
    fn append_to(&self, request: &mut http::ContractRequest<'_>) {
        request.scalar("cursor", self.cursor.as_ref());
    }
}

/// The `/radio` endpoints, obtained from [`Client::radio`].
#[derive(Clone, Copy, Debug)]
pub struct Radio<'a> {
    client: &'a Client,
}

impl Client {
    /// Returns the handle for the NOAA Weather Radio endpoints.
    #[must_use]
    pub fn radio(&self) -> Radio<'_> {
        Radio { client: self }
    }
}

impl Radio<'_> {
    /// Returns the broadcast script for the transmitter covering `point`.
    ///
    /// `GET /points/{latitude},{longitude}/radio`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, Coordinates};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let broadcast = client
    ///     .radio()
    ///     .for_point(Coordinates::new(33.4484, -112.074)?)
    ///     .await?;
    /// # let _ = broadcast;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the SSML cannot be
    /// decoded.
    pub async fn for_point(&self, point: Coordinates) -> Result<models::RadioBroadcast, Error> {
        http::request(self.client, "/points")
            .path_segment(point)
            .literal_path("radio")
            .xml(http::XmlMedia::Ssml)
            .await
    }

    /// Returns the current broadcast script of one transmitter.
    ///
    /// `GET /radio/{callSign}/broadcast`
    ///
    /// ```no_run
    /// use noaa_weather_client::{CallSign, Client};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let kec94: CallSign = "KEC94".parse()?;
    /// let broadcast = client.radio().broadcast(&kec94).await?;
    /// # let _ = broadcast;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails, the call sign is unknown,
    /// or the SSML cannot be decoded.
    pub async fn broadcast(&self, call_sign: &CallSign) -> Result<models::RadioBroadcast, Error> {
        http::request(self.client, "/radio")
            .path_segment(call_sign)
            .literal_path("broadcast")
            .xml(http::XmlMedia::Ssml)
            .await
    }

    /// Returns a page of transmitters.
    ///
    /// `GET /radio`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, apis::radio::TransmittersQuery};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let page = client.radio().transmitters(&TransmittersQuery::default()).await?;
    /// # let _ = page;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn transmitters(
        &self,
        query: &TransmittersQuery,
    ) -> Result<models::RadioTransmitterCollection, Error> {
        http::request(self.client, "/radio")
            .query(query)
            .json(http::JsonMedia::JsonLd)
            .await
    }

    /// Returns one transmitter's metadata.
    ///
    /// `GET /radio/{callSign}`
    ///
    /// ```no_run
    /// use noaa_weather_client::{CallSign, Client};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let kec94: CallSign = "KEC94".parse()?;
    /// let transmitter = client.radio().transmitter(&kec94).await?;
    /// # let _ = transmitter;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn transmitter(
        &self,
        call_sign: &CallSign,
    ) -> Result<models::RadioTransmitter, Error> {
        http::request(self.client, "/radio")
            .path_segment(call_sign)
            .json(http::JsonMedia::JsonLd)
            .await
    }

    /// Returns the transmitters serving one county zone.
    ///
    /// `GET /zones/county/{zoneId}/radio`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, ZoneId};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let county: ZoneId = "AZC013".parse()?;
    /// let transmitters = client.radio().transmitters_for_county(&county).await?;
    /// # let _ = transmitters;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn transmitters_for_county(
        &self,
        county: &ZoneId,
    ) -> Result<models::RadioTransmitterCollection, Error> {
        http::request(self.client, "/zones/county")
            .path_segment(county)
            .literal_path("radio")
            .json(http::JsonMedia::JsonLd)
            .await
    }
}

#[cfg(test)]
mod tests {
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method, path},
    };

    use super::TransmittersQuery;
    use crate::{Coordinates, Error, client::test_support::client_for};

    const TRANSMITTERS: &str = r#"{
        "@graph": [{
            "@id": "https://api.weather.gov/radio/KAAA",
            "@type": "wx:Transmitter",
            "setId": "nwr-transmitters-test",
            "callSign": "KAAA",
            "transmitterFrequency": "162.550",
            "siteName": "Alpha",
            "siteCity": "Phoenix",
            "siteState": "AZ",
            "sameCodes": ["004013", "004013"],
            "counties": ["AZC013", "AZC013"]
        }, {
            "@id": "https://api.weather.gov/radio/KBBB",
            "@type": "wx:Transmitter",
            "setId": "nwr-transmitters-test",
            "callSign": "KBBB",
            "transmitterFrequency": "162.400",
            "siteName": "Bravo",
            "siteCity": "Tucson",
            "siteState": "AZ",
            "sameCodes": [],
            "counties": []
        }],
        "pagination": {"next": "https://api.weather.gov/radio?cursor=next-page"}
    }"#;
    const RADIO_BROADCAST: &str = r#"<speak version="1.1" xml:lang="en-US"></speak>"#;

    #[tokio::test]
    async fn transmitter_broadcast_normalizes_call_sign_and_requests_ssml() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/radio/KEC94/broadcast"))
            .and(header("Accept", "application/ssml+xml"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(RADIO_BROADCAST, "application/ssml+xml"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let broadcast = client_for(&server)
            .radio()
            .broadcast(&"kec94".parse().unwrap())
            .await
            .unwrap();
        assert_eq!(broadcast.version, "1.1");
        assert_eq!(broadcast.lang, "en-US");
    }

    #[tokio::test]
    async fn point_broadcast_encodes_one_coordinate_segment_and_requests_ssml() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/points/33.4484,-112.074/radio"))
            .and(header("Accept", "application/ssml+xml"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(RADIO_BROADCAST, "application/ssml+xml"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let broadcast = client_for(&server)
            .radio()
            .for_point(Coordinates::new(33.4484, -112.074).unwrap())
            .await
            .unwrap();
        assert_eq!(broadcast.version, "1.1");
    }

    #[tokio::test]
    async fn transmitter_broadcast_rejects_generic_xml_and_malformed_ssml() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/radio/KEC94/broadcast"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(RADIO_BROADCAST, "application/xml"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let Error::Protocol(error) = client_for(&server)
            .radio()
            .broadcast(&"KEC94".parse().unwrap())
            .await
            .unwrap_err()
        else {
            panic!("expected protocol error");
        };
        assert_eq!(error.expected(), Some("application/ssml+xml"));
        assert_eq!(error.actual(), Some("application/xml"));

        let server = MockServer::start().await;
        Mock::given(path("/radio/KEC94/broadcast"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw("<speak>", "application/ssml+xml"),
            )
            .mount(&server)
            .await;
        assert!(matches!(
            client_for(&server)
                .radio()
                .broadcast(&"KEC94".parse().unwrap())
                .await,
            Err(Error::Xml(_))
        ));
    }

    #[tokio::test]
    async fn transmitter_list_omits_or_encodes_opaque_cursor_and_preserves_pagination() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/radio"))
            .and(header("Accept", "application/ld+json"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(TRANSMITTERS, "application/ld+json"),
            )
            .expect(2)
            .mount(&server)
            .await;

        let client = client_for(&server);
        let first_page = client
            .radio()
            .transmitters(&TransmittersQuery::default())
            .await
            .unwrap();
        let second_page = client
            .radio()
            .transmitters(&TransmittersQuery {
                cursor: Some("opaque+/=_-".parse().unwrap()),
            })
            .await
            .unwrap();

        assert_eq!(
            first_page.pagination.unwrap().next,
            "https://api.weather.gov/radio?cursor=next-page"
        );
        assert_eq!(second_page.transmitters[0].frequency, "162.550");
        assert_eq!(second_page.transmitters[0].same_codes, ["004013", "004013"]);

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.query(), None);
        assert_eq!(requests[1].url.query(), Some("cursor=opaque%2B%2F%3D_-"));
    }

    #[tokio::test]
    async fn transmitter_detail_normalizes_call_sign_and_returns_an_object() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/radio/KAAA"))
            .and(header("Accept", "application/ld+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{
                    "@context": "https://geojson.org/geojson-ld/geojson-context.jsonld",
                    "@id": "https://api.weather.gov/radio/KAAA",
                    "@type": "wx:Transmitter",
                    "setId": "nwr-transmitters-test",
                    "callSign": "KAAA",
                    "transmitterFrequency": "162.550",
                    "siteName": "Alpha",
                    "siteCity": "Phoenix",
                    "siteState": "AZ",
                    "sameCodes": [],
                    "counties": []
                }"#,
                "application/ld+json",
            ))
            .expect(1)
            .mount(&server)
            .await;

        let transmitter = client_for(&server)
            .radio()
            .transmitter(&"kaaa".parse().unwrap())
            .await
            .unwrap();

        assert_eq!(transmitter.call_sign.as_str(), "KAAA");
        assert_eq!(transmitter.frequency, "162.550");
        assert!(transmitter.same_codes.is_empty());
    }

    #[tokio::test]
    async fn county_zone_transmitters_use_the_county_path_without_pagination() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/zones/county/AZC013/radio"))
            .and(header("Accept", "application/ld+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"@graph":[{
                        "@id":"https://api.weather.gov/radio/KPHX",
                        "@type":"wx:Transmitter",
                        "setId":"nwr-transmitters-test",
                        "callSign":"KPHX",
                        "transmitterFrequency":"162.400",
                        "siteName":"Phoenix",
                        "siteCity":"Phoenix",
                        "siteState":"AZ",
                        "sameCodes":[],
                        "counties":[]
                    }]}"#,
                "application/ld+json",
            ))
            .expect(1)
            .mount(&server)
            .await;

        let transmitters = client_for(&server)
            .radio()
            .transmitters_for_county(&"azc013".parse().unwrap())
            .await
            .unwrap();

        assert_eq!(transmitters.pagination, None);
        assert_eq!(transmitters.transmitters[0].call_sign.as_str(), "KPHX");
        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.query(), None);
    }
}
