//! NWS text products such as Area Forecast Discussions: the `/products`
//! family.
//!
//! Obtain the handle with [`Client::products`]. Product types are
//! [`ProductTypeCode`]s (`AFD`, `HWO`), issuance locations are [`OfficeId`]s,
//! and individual products are addressed by their server-issued
//! [`ProductId`].
//!
//! ```no_run
//! use noaa_weather_client::{Client, OfficeId, ProductTypeCode};
//!
//! # async fn run() -> Result<(), noaa_weather_client::Error> {
//! let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
//! let afd: ProductTypeCode = "AFD".parse()?;
//! let office: OfficeId = "PSR".parse()?;
//! let latest = client.products().latest(&afd, &office).await?;
//! println!("{}", latest.product_text.unwrap_or_default());
//! # Ok(())
//! # }
//! ```

use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use super::Error;
use crate::client::{Client, http};
use crate::ids::{OfficeId, ProductId, ProductTypeCode};
use crate::models;

/// Filters for [`Products::search`].
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default)]
pub struct ProductsQuery {
    /// Issuance locations to include (`location` on the wire).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub location_ids: Vec<OfficeId>,
    /// Earliest issuance time to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(with = "Option<String>"))]
    pub start: Option<Timestamp>,
    /// Latest issuance time to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(with = "Option<String>"))]
    pub end: Option<Timestamp>,
    /// Issuing offices to include (`office` on the wire).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub office_ids: Vec<OfficeId>,
    /// WMO header identifiers to include (`wmoid` on the wire).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub wmo_ids: Vec<String>,
    /// Product type codes to include (`type` on the wire).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub product_type_codes: Vec<ProductTypeCode>,
    /// Maximum number of products to return (1 to 500).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(range(min = 1, max = 500)))]
    pub limit: Option<u16>,
}

impl http::QueryParams for ProductsQuery {
    fn append_to(&self, request: &mut http::ContractRequest<'_>) {
        request.list("location", &self.location_ids);
        request.instant("start", self.start.as_ref());
        request.instant("end", self.end.as_ref());
        request.list("office", &self.office_ids);
        request.list("wmoid", &self.wmo_ids);
        request.list("type", &self.product_type_codes);
        request.scalar("limit", self.limit.as_ref());
    }
}

/// The `/products` endpoints, obtained from [`Client::products`].
#[derive(Clone, Copy, Debug)]
pub struct Products<'a> {
    client: &'a Client,
}

impl Client {
    /// Returns the handle for the `/products` endpoints.
    #[must_use]
    pub fn products(&self) -> Products<'_> {
        Products { client: self }
    }
}

impl Products<'_> {
    fn product_type(&self, code: &ProductTypeCode) -> http::ContractRequest<'_> {
        http::request(self.client, "/products/types").path_segment(code)
    }

    /// Returns products matching `query`.
    ///
    /// `GET /products`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, apis::products::ProductsQuery};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let products = client
    ///     .products()
    ///     .search(&ProductsQuery {
    ///         location_ids: vec!["LWX".parse()?],
    ///         product_type_codes: vec!["AFD".parse()?],
    ///         limit: Some(10),
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
    pub async fn search(
        &self,
        query: &ProductsQuery,
    ) -> Result<models::TextProductCollection, Error> {
        http::request(self.client, "/products")
            .query(query)
            .json(http::JsonMedia::JsonLd)
            .await
    }

    /// Returns one product, including its full text, by id.
    ///
    /// `GET /products/{productId}`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, ProductId};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let id: ProductId = "a4791428-298e-473c-8e6f-5796701c9e4a".parse()?;
    /// let product = client.products().get(&id).await?;
    /// # let _ = product;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails, the product is unknown, or
    /// the response cannot be decoded.
    pub async fn get(&self, id: &ProductId) -> Result<models::TextProduct, Error> {
        http::request(self.client, "/products")
            .path_segment(id)
            .json(http::JsonMedia::JsonLd)
            .await
    }

    /// Returns every product issuance location.
    ///
    /// `GET /products/locations`
    ///
    /// ```no_run
    /// use noaa_weather_client::Client;
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let locations = client.products().locations().await?;
    /// # let _ = locations;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn locations(&self) -> Result<models::TextProductLocationCollection, Error> {
        http::request(self.client, "/products/locations")
            .json(http::JsonMedia::JsonLd)
            .await
    }

    /// Returns every product type and its code.
    ///
    /// `GET /products/types`
    ///
    /// ```no_run
    /// use noaa_weather_client::Client;
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let types = client.products().types().await?;
    /// # let _ = types;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn types(&self) -> Result<models::TextProductTypeCollection, Error> {
        http::request(self.client, "/products/types")
            .json(http::JsonMedia::JsonLd)
            .await
    }

    /// Returns the product types issued at one location.
    ///
    /// `GET /products/locations/{locationId}/types`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, OfficeId};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let office: OfficeId = "LWX".parse()?;
    /// let types = client.products().types_for_location(&office).await?;
    /// # let _ = types;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn types_for_location(
        &self,
        location: &OfficeId,
    ) -> Result<models::TextProductTypeCollection, Error> {
        http::request(self.client, "/products/locations")
            .path_segment(location)
            .literal_path("types")
            .json(http::JsonMedia::JsonLd)
            .await
    }

    /// Returns recent products of one type from every location.
    ///
    /// `GET /products/types/{typeId}`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, ProductTypeCode};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let hwo: ProductTypeCode = "HWO".parse()?;
    /// let products = client.products().by_type(&hwo).await?;
    /// # let _ = products;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn by_type(
        &self,
        code: &ProductTypeCode,
    ) -> Result<models::TextProductCollection, Error> {
        self.product_type(code).json(http::JsonMedia::JsonLd).await
    }

    /// Returns recent products of one type from one location.
    ///
    /// `GET /products/types/{typeId}/locations/{locationId}`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, OfficeId, ProductTypeCode};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let afd: ProductTypeCode = "AFD".parse()?;
    /// let office: OfficeId = "LWX".parse()?;
    /// let products = client.products().by_type_and_location(&afd, &office).await?;
    /// # let _ = products;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn by_type_and_location(
        &self,
        code: &ProductTypeCode,
        location: &OfficeId,
    ) -> Result<models::TextProductCollection, Error> {
        self.product_type(code)
            .literal_path("locations")
            .path_segment(location)
            .json(http::JsonMedia::JsonLd)
            .await
    }

    /// Returns the locations that issue one product type.
    ///
    /// `GET /products/types/{typeId}/locations`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, ProductTypeCode};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let hwo: ProductTypeCode = "HWO".parse()?;
    /// let locations = client.products().locations_for_type(&hwo).await?;
    /// # let _ = locations;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn locations_for_type(
        &self,
        code: &ProductTypeCode,
    ) -> Result<models::TextProductLocationCollection, Error> {
        self.product_type(code)
            .literal_path("locations")
            .json(http::JsonMedia::JsonLd)
            .await
    }

    /// Returns the most recent product of one type from one location,
    /// including its full text.
    ///
    /// `GET /products/types/{typeId}/locations/{locationId}/latest`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, OfficeId, ProductTypeCode};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let afd: ProductTypeCode = "AFD".parse()?;
    /// let office: OfficeId = "PSR".parse()?;
    /// let latest = client.products().latest(&afd, &office).await?;
    /// # let _ = latest;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails, no product exists for the
    /// pair, or the response cannot be decoded.
    pub async fn latest(
        &self,
        code: &ProductTypeCode,
        location: &OfficeId,
    ) -> Result<models::TextProduct, Error> {
        self.product_type(code)
            .literal_path("locations")
            .path_segment(location)
            .literal_path("latest")
            .json(http::JsonMedia::JsonLd)
            .await
    }
}

#[cfg(test)]
mod tests {
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers::method};

    use super::ProductsQuery;
    use crate::client::test_support::client_for;

    async fn mount_json_ld(server: &MockServer, expected: u64) {
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw("{}", "application/ld+json"))
            .expect(expected)
            .mount(server)
            .await;
    }

    #[tokio::test]
    async fn search_encodes_lists_as_csv_and_times_as_rfc_3339() {
        let server = MockServer::start().await;
        mount_json_ld(&server, 1).await;

        client_for(&server)
            .products()
            .search(&ProductsQuery {
                location_ids: vec!["lwx".parse().unwrap(), "PQR".parse().unwrap()],
                start: Some("2026-08-30T00:00:00Z".parse().unwrap()),
                end: Some("2026-08-30T12:00:00Z".parse().unwrap()),
                office_ids: vec!["LWX".parse().unwrap()],
                wmo_ids: vec!["TTAA 00".to_owned(), "TT/BB%".to_owned()],
                product_type_codes: vec!["afd".parse().unwrap(), "HWO".parse().unwrap()],
                limit: Some(5),
            })
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.path(), "/products");
        assert_eq!(
            requests[0].url.query(),
            Some(
                "location=LWX%2CPQR&start=2026-08-30T00%3A00%3A00Z&end=2026-08-30T12%3A00%3A00Z\
                 &office=LWX&wmoid=TTAA+00%2CTT%2FBB%25&type=AFD%2CHWO&limit=5"
            )
        );
        assert_eq!(requests[0].headers["accept"], "application/ld+json");
    }

    #[tokio::test]
    async fn search_with_default_query_sends_nothing() {
        let server = MockServer::start().await;
        mount_json_ld(&server, 1).await;

        client_for(&server)
            .products()
            .search(&ProductsQuery::default())
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.query(), None);
    }

    #[tokio::test]
    async fn path_routes_normalize_typed_segments_and_request_json_ld() {
        let server = MockServer::start().await;
        mount_json_ld(&server, 7).await;
        let client = client_for(&server);
        let products = client.products();
        let afd = "afd".parse().unwrap();
        let lwx = "lwx".parse().unwrap();

        products.types_for_location(&lwx).await.unwrap();
        products
            .get(&"a4791428-298e-473c-8e6f-5796701c9e4a".parse().unwrap())
            .await
            .unwrap();
        products.locations().await.unwrap();
        products.types().await.unwrap();
        products.by_type(&afd).await.unwrap();
        products.locations_for_type(&afd).await.unwrap();
        products.by_type_and_location(&afd, &lwx).await.unwrap();

        let requests = server.received_requests().await.unwrap();
        let routes = requests
            .iter()
            .map(|request| {
                assert_eq!(request.url.query(), None);
                assert_eq!(request.headers["accept"], "application/ld+json");
                request.url.path().to_owned()
            })
            .collect::<Vec<_>>();
        assert_eq!(
            routes,
            [
                "/products/locations/LWX/types",
                "/products/a4791428-298e-473c-8e6f-5796701c9e4a",
                "/products/locations",
                "/products/types",
                "/products/types/AFD",
                "/products/types/AFD/locations",
                "/products/types/AFD/locations/LWX",
            ]
        );
    }

    #[tokio::test]
    async fn latest_appends_the_latest_literal() {
        let server = MockServer::start().await;
        mount_json_ld(&server, 1).await;

        client_for(&server)
            .products()
            .latest(&"AFD".parse().unwrap(), &"psr".parse().unwrap())
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests[0].url.path(),
            "/products/types/AFD/locations/PSR/latest"
        );
    }
}
