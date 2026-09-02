//! Aviation hazards: Center Weather Advisories and SIGMETs, the `/aviation`
//! family.
//!
//! Obtain the handle with [`Client::aviation`]. Center Weather Service Units
//! are addressed by [`CwsuId`] and Air Traffic Service Units by [`AtsuId`].
//! Date-only path segments take a [`jiff::civil::Date`]; the SIGMET
//! date-plus-time segment takes a [`jiff::Timestamp`] split in UTC.
//!
//! ```no_run
//! use noaa_weather_client::{AtsuId, Client, apis::aviation::SigmetsQuery};
//!
//! # async fn run() -> Result<(), noaa_weather_client::Error> {
//! let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
//! let kkci: AtsuId = "KKCI".parse()?;
//! let sigmets = client
//!     .aviation()
//!     .sigmets(&SigmetsQuery { atsu: Some(kkci), ..Default::default() })
//!     .await?;
//! # let _ = sigmets;
//! # Ok(())
//! # }
//! ```

use jiff::{Timestamp, civil::Date};
use serde::{Deserialize, Serialize};

use super::Error;
use crate::client::{Client, http};
use crate::geo::{Feature, FeatureCollection};
use crate::ids::{AtsuId, CwsuId};
use crate::models;

/// Filters for [`Aviation::sigmets`].
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default)]
pub struct SigmetsQuery {
    /// Earliest issuance time to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(with = "Option<String>"))]
    pub start: Option<Timestamp>,
    /// Latest issuance time to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(with = "Option<String>"))]
    pub end: Option<Timestamp>,
    /// Only products issued on this UTC date.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(with = "Option<String>"))]
    pub date: Option<Date>,
    /// Only products from this Air Traffic Service Unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atsu: Option<AtsuId>,
    /// Only products with this server-assigned sequence (for example `52C`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<String>,
}

impl http::QueryParams for SigmetsQuery {
    fn append_to(&self, request: &mut http::ContractRequest<'_>) {
        request.instant("start", self.start.as_ref());
        request.instant("end", self.end.as_ref());
        request.scalar("date", self.date.as_ref());
        request.scalar("atsu", self.atsu.as_ref());
        request.scalar("sequence", self.sequence.as_ref());
    }
}

/// The `/aviation` endpoints, obtained from [`Client::aviation`].
#[derive(Clone, Copy, Debug)]
pub struct Aviation<'a> {
    client: &'a Client,
}

impl Client {
    /// Returns the handle for the `/aviation` endpoints.
    #[must_use]
    pub fn aviation(&self) -> Aviation<'_> {
        Aviation { client: self }
    }
}

impl Aviation<'_> {
    fn cwsu_request(&self, cwsu: &CwsuId) -> http::ContractRequest<'_> {
        http::request(self.client, "/aviation/cwsus").path_segment(cwsu)
    }

    fn atsu_request(&self, atsu: &AtsuId) -> http::ContractRequest<'_> {
        http::request(self.client, "/aviation/sigmets").path_segment(atsu)
    }

    /// Returns metadata for one Center Weather Service Unit.
    ///
    /// `GET /aviation/cwsus/{cwsuId}`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, CwsuId};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let zab: CwsuId = "ZAB".parse()?;
    /// let unit = client.aviation().cwsu(&zab).await?;
    /// # let _ = unit;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn cwsu(&self, cwsu: &CwsuId) -> Result<models::CwsuOffice, Error> {
        self.cwsu_request(cwsu).json(http::JsonMedia::JsonLd).await
    }

    /// Returns the current Center Weather Advisories from one unit.
    ///
    /// `GET /aviation/cwsus/{cwsuId}/cwas`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, CwsuId};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let zab: CwsuId = "ZAB".parse()?;
    /// let advisories = client.aviation().cwas(&zab).await?;
    /// # let _ = advisories;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn cwas(
        &self,
        cwsu: &CwsuId,
    ) -> Result<FeatureCollection<models::CenterWeatherAdvisory>, Error> {
        self.cwsu_request(cwsu)
            .literal_path("cwas")
            .json(http::JsonMedia::GeoJson)
            .await
    }

    /// Returns one Center Weather Advisory by issue date and sequence number
    /// (NOAA numbers them from 100).
    ///
    /// `GET /aviation/cwsus/{cwsuId}/cwas/{date}/{sequence}`
    ///
    /// ```no_run
    /// use jiff::civil::date;
    /// use noaa_weather_client::{Client, CwsuId};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let zab: CwsuId = "ZAB".parse()?;
    /// let advisory = client.aviation().cwa(&zab, date(2026, 8, 30), 101).await?;
    /// # let _ = advisory;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails, the advisory is unknown, or
    /// the response cannot be decoded.
    pub async fn cwa(
        &self,
        cwsu: &CwsuId,
        date: Date,
        sequence: u32,
    ) -> Result<Feature<models::CenterWeatherAdvisory>, Error> {
        self.cwsu_request(cwsu)
            .literal_path("cwas")
            .path_segment(date)
            .path_segment(sequence)
            .json(http::JsonMedia::GeoJson)
            .await
    }

    /// Returns SIGMET and AIRMET products matching `query`.
    ///
    /// `GET /aviation/sigmets`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, apis::aviation::SigmetsQuery};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let products = client
    ///     .aviation()
    ///     .sigmets(&SigmetsQuery {
    ///         start: Some("2026-08-30T00:00:00Z".parse().unwrap()),
    ///         ..Default::default()
    ///     })
    ///     .await?;
    /// # let _ = products;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn sigmets(
        &self,
        query: &SigmetsQuery,
    ) -> Result<FeatureCollection<models::Sigmet>, Error> {
        http::request(self.client, "/aviation/sigmets")
            .query(query)
            .json(http::JsonMedia::GeoJson)
            .await
    }

    /// Returns SIGMET and AIRMET products from one Air Traffic Service Unit.
    ///
    /// `GET /aviation/sigmets/{atsu}`
    ///
    /// ```no_run
    /// use noaa_weather_client::{AtsuId, Client};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let kkci: AtsuId = "KKCI".parse()?;
    /// let products = client.aviation().sigmets_for_atsu(&kkci).await?;
    /// # let _ = products;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn sigmets_for_atsu(
        &self,
        atsu: &AtsuId,
    ) -> Result<FeatureCollection<models::Sigmet>, Error> {
        self.atsu_request(atsu).json(http::JsonMedia::GeoJson).await
    }

    /// Returns SIGMET and AIRMET products from one unit on one UTC date.
    ///
    /// `GET /aviation/sigmets/{atsu}/{date}`
    ///
    /// ```no_run
    /// use jiff::civil::date;
    /// use noaa_weather_client::{AtsuId, Client};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let kkci: AtsuId = "KKCI".parse()?;
    /// let products = client
    ///     .aviation()
    ///     .sigmets_for_atsu_on(&kkci, date(2026, 8, 30))
    ///     .await?;
    /// # let _ = products;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn sigmets_for_atsu_on(
        &self,
        atsu: &AtsuId,
        date: Date,
    ) -> Result<FeatureCollection<models::Sigmet>, Error> {
        self.atsu_request(atsu)
            .path_segment(date)
            .json(http::JsonMedia::GeoJson)
            .await
    }

    /// Returns one SIGMET or AIRMET by its unit and issue instant.
    ///
    /// `GET /aviation/sigmets/{atsu}/{date}/{time}`
    ///
    /// NOAA addresses the product by its issue date and `HHMM` time in UTC,
    /// so `issued` is split into those two segments in UTC and any seconds
    /// are dropped (minute precision).
    ///
    /// ```no_run
    /// use noaa_weather_client::{AtsuId, Client};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let kkci: AtsuId = "KKCI".parse()?;
    /// let issued = "2026-08-30T14:30:00Z".parse().unwrap();
    /// let product = client.aviation().sigmet(&kkci, issued).await?;
    /// # let _ = product;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails, the product is unknown, or
    /// the response cannot be decoded.
    pub async fn sigmet(
        &self,
        atsu: &AtsuId,
        issued: Timestamp,
    ) -> Result<Feature<models::Sigmet>, Error> {
        self.atsu_request(atsu)
            .path_segment(issued.strftime("%Y-%m-%d"))
            .path_segment(issued.strftime("%H%M"))
            .json(http::JsonMedia::GeoJson)
            .await
    }
}

#[cfg(test)]
mod tests {
    use jiff::civil::date;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method, path},
    };

    use super::SigmetsQuery;
    use crate::client::test_support::client_for;

    const FEATURE: &str = r#"{"type":"Feature","geometry":null,"properties":{}}"#;
    const COLLECTION: &str = r#"{"type":"FeatureCollection","features":[]}"#;

    async fn mount(server: &MockServer, route: &str, body: &'static str, media: &'static str) {
        Mock::given(method("GET"))
            .and(path(route))
            .and(header("Accept", media))
            .respond_with(ResponseTemplate::new(200).set_body_raw(body, media))
            .expect(1)
            .mount(server)
            .await;
    }

    #[tokio::test]
    async fn sigmet_splits_the_issue_instant_into_utc_date_and_hhmm() {
        let server = MockServer::start().await;
        mount(
            &server,
            "/aviation/sigmets/KKCI/2026-08-31/0030",
            FEATURE,
            "application/geo+json",
        )
        .await;

        // 19:30:59 in UTC-5 is 00:30 UTC the next day; seconds are dropped.
        client_for(&server)
            .aviation()
            .sigmet(
                &"kkci".parse().unwrap(),
                "2026-08-30T19:30:59-05:00".parse().unwrap(),
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn cwa_places_typed_id_iso_date_and_sequence_in_the_path() {
        let server = MockServer::start().await;
        mount(
            &server,
            "/aviation/cwsus/ZAB/cwas/2026-08-30/101",
            FEATURE,
            "application/geo+json",
        )
        .await;

        client_for(&server)
            .aviation()
            .cwa(&"zab".parse().unwrap(), date(2026, 8, 30), 101)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn cwsu_routes_request_their_media_types() {
        let server = MockServer::start().await;
        mount(&server, "/aviation/cwsus/ZAB", "{}", "application/ld+json").await;
        mount(
            &server,
            "/aviation/cwsus/ZAB/cwas",
            COLLECTION,
            "application/geo+json",
        )
        .await;

        let client = client_for(&server);
        let zab = "ZAB".parse().unwrap();
        client.aviation().cwsu(&zab).await.unwrap();
        client.aviation().cwas(&zab).await.unwrap();
    }

    #[tokio::test]
    async fn atsu_routes_place_the_unit_and_date_in_the_path() {
        let server = MockServer::start().await;
        mount(
            &server,
            "/aviation/sigmets/KKCI",
            COLLECTION,
            "application/geo+json",
        )
        .await;
        mount(
            &server,
            "/aviation/sigmets/KKCI/2026-08-30",
            COLLECTION,
            "application/geo+json",
        )
        .await;

        let client = client_for(&server);
        let kkci = "KKCI".parse().unwrap();
        client.aviation().sigmets_for_atsu(&kkci).await.unwrap();
        client
            .aviation()
            .sigmets_for_atsu_on(&kkci, date(2026, 8, 30))
            .await
            .unwrap();
        for request in server.received_requests().await.unwrap() {
            assert_eq!(request.url.query(), None);
        }
    }

    #[tokio::test]
    async fn sigmets_query_encodes_every_field_in_order() {
        let server = MockServer::start().await;
        mount(
            &server,
            "/aviation/sigmets",
            COLLECTION,
            "application/geo+json",
        )
        .await;

        client_for(&server)
            .aviation()
            .sigmets(&SigmetsQuery {
                start: Some("2026-08-30T00:00:00+00:00".parse().unwrap()),
                end: Some("2026-08-30T12:00:00Z".parse().unwrap()),
                date: Some(date(2026, 8, 30)),
                atsu: Some("kkci".parse().unwrap()),
                sequence: Some("52C".to_owned()),
            })
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests[0].url.query(),
            Some(
                "start=2026-08-30T00%3A00%3A00Z&end=2026-08-30T12%3A00%3A00Z&date=2026-08-30\
                 &atsu=KKCI&sequence=52C"
            )
        );
    }

    #[tokio::test]
    async fn sigmets_default_query_sends_nothing() {
        let server = MockServer::start().await;
        mount(
            &server,
            "/aviation/sigmets",
            COLLECTION,
            "application/geo+json",
        )
        .await;

        client_for(&server)
            .aviation()
            .sigmets(&SigmetsQuery::default())
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.query(), None);
    }
}
