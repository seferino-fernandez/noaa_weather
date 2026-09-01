//! NWS text products (Area Forecast Discussions, watches, warnings, etc.).
//!
//! Covers the `/products` endpoints for querying, listing, and retrieving
//! the full text of NWS-issued products by type, location, or issuance time.

use super::Error;
use crate::client::{Client, http};
use crate::models;

/// Parameters for the [`get_products_query`] function.
///
/// This struct encapsulates the query parameters for filtering text products.
#[derive(Debug, Clone, Default)]
pub struct ProductsQueryParams {
    /// Filter by issuance location ID (e.g., "LWX", "PQR").
    pub location_ids: Option<Vec<models::NwsForecastOfficeId>>,
    /// Start time for the query period (ISO 8601 format).
    pub start_time: Option<String>,
    /// End time for the query period (ISO 8601 format).
    pub end_time: Option<String>,
    /// Filter by issuing office ID (typically WFO ID, e.g., "LWX", "PQR").
    pub office_ids: Option<Vec<models::NwsForecastOfficeId>>,
    /// Filter by WMO header ID.
    pub wmo_ids: Option<Vec<String>>,
    /// Filter by product type code (e.g., "AFD", "HWO").
    pub product_type_codes: Option<Vec<String>>,
    /// Limit the number of results returned.
    pub limit: Option<i32>,
}

/// Returns a list of valid text product types for a given issuance location.
///
/// Corresponds to the `/products/locations/{locationId}/types` endpoint.
///
/// # Parameters
///
/// * `client`: The API client.
/// * `location_id`: The ID of the issuance location (e.g., "LWX", "PQR").
///
/// # Returns
///
/// A `Result` containing a [`models::TextProductTypeCollection`] on success, listing
/// the product types available for the location.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_products_by_location(
    client: &Client,
    location_id: &models::NwsForecastOfficeId,
) -> Result<models::TextProductTypeCollection, Error> {
    http::request(client, "/products/locations")
        .path_segment(location_id)
        .literal_path("types")
        .json(http::JsonMedia::JsonLd)
        .await
}

/// Returns a specific NWS text product by its unique product ID.
///
/// Corresponds to the `/products/{productId}` endpoint.
///
/// # Parameters
///
/// * `client`: The API client.
/// * `product_id`: The unique ID of the product.
///
/// # Returns
///
/// A `Result` containing the [`models::TextProduct`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails (e.g., product not found)
/// or the response cannot be parsed.
pub async fn get_product(client: &Client, product_id: &str) -> Result<models::TextProduct, Error> {
    http::request(client, "/products")
        .path_segment(product_id)
        .json(http::JsonMedia::JsonLd)
        .await
}

/// Returns a list of valid NWS text product issuance locations.
///
/// Corresponds to the `/products/locations` endpoint.
///
/// # Parameters
///
/// * `client`: The API client.
///
/// # Returns
///
/// A `Result` containing a [`models::TextProductLocationCollection`] on success, listing
/// valid location IDs and their names.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_product_locations(
    client: &Client,
) -> Result<models::TextProductLocationCollection, Error> {
    http::request(client, "/products/locations")
        .json(http::JsonMedia::JsonLd)
        .await
}

/// Returns a list of valid NWS text product types and their codes.
///
/// Corresponds to the `/products/types` endpoint.
///
/// # Parameters
///
/// * `client`: The API client.
///
/// # Returns
///
/// A `Result` containing a [`models::TextProductTypeCollection`] on success, listing
/// product codes and their names.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_product_types(
    client: &Client,
) -> Result<models::TextProductTypeCollection, Error> {
    http::request(client, "/products/types")
        .json(http::JsonMedia::JsonLd)
        .await
}

/// Returns a list of text products based on specified query parameters.
///
/// Corresponds to the `/products` endpoint.
/// Allows filtering by location, time range, office, WMO ID, and product type.
///
/// # Parameters
///
/// * `client`: The API client.
/// * `params`: A [`ProductsQueryParams`] struct containing the query parameters.
///
/// # Returns
///
/// A `Result` containing a [`models::TextProductCollection`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_products_query(
    client: &Client,
    params: ProductsQueryParams,
) -> Result<models::TextProductCollection, Error> {
    http::request(client, "/products")
        .query_csv("location", params.location_ids)
        .query_scalar("start", params.start_time)
        .query_scalar("end", params.end_time)
        .query_csv("office", params.office_ids)
        .query_csv("wmoid", params.wmo_ids)
        .query_csv("type", params.product_type_codes)
        .query_scalar("limit", params.limit)
        .json(http::JsonMedia::JsonLd)
        .await
}

/// Returns a list of text products of a specific type.
///
/// Corresponds to the `/products/types/{typeId}` endpoint.
///
/// # Parameters
///
/// * `client`: The API client.
/// * `type_id`: The NWS product type code (e.g., "AFD", "HWO").
///
/// # Returns
///
/// A `Result` containing a [`models::TextProductCollection`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_products_by_type(
    client: &Client,
    type_id: &str,
) -> Result<models::TextProductCollection, Error> {
    http::request(client, "/products/types")
        .path_segment(type_id)
        .json(http::JsonMedia::JsonLd)
        .await
}

/// Returns a list of text products of a specific type for a specific issuance location.
///
/// Corresponds to the `/products/types/{typeId}/locations/{locationId}` endpoint.
///
/// # Parameters
///
/// * `client`: The API client.
/// * `type_id`: The NWS product type code.
/// * `location_id`: The ID of the issuance location.
///
/// # Returns
///
/// A `Result` containing a [`models::TextProductCollection`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_products_by_type_and_location(
    client: &Client,
    type_id: &str,
    location_id: &models::NwsForecastOfficeId,
) -> Result<models::TextProductCollection, Error> {
    http::request(client, "/products/types")
        .path_segment(type_id)
        .literal_path("locations")
        .path_segment(location_id)
        .json(http::JsonMedia::JsonLd)
        .await
}

/// Returns a list of valid text product issuance locations for a given product type.
///
/// Corresponds to the `/products/types/{typeId}/locations` endpoint.
///
/// # Parameters
///
/// * `client`: The API client.
/// * `type_id`: The NWS product type code.
///
/// # Returns
///
/// A `Result` containing a [`models::TextProductLocationCollection`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_product_issuance_locations_by_type(
    client: &Client,
    type_id: &str,
) -> Result<models::TextProductLocationCollection, Error> {
    http::request(client, "/products/types")
        .path_segment(type_id)
        .literal_path("locations")
        .json(http::JsonMedia::JsonLd)
        .await
}

/// Returns the latest text product of a specific type for a specific issuance location.
///
/// Corresponds to the `/products/types/{typeId}/locations/{locationId}/latest` endpoint.
///
/// # Parameters
///
/// * `client`: The API client.
/// * `type_id`: The NWS product type code (e.g., "AFD", "HWO").
/// * `location_id`: The ID of the issuance location (e.g., "LWX", "PQR").
///
/// # Returns
///
/// A `Result` containing the [`models::TextProduct`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails (e.g., no product
/// found for the given type and location) or the response cannot be parsed.
pub async fn get_latest_product_by_type_and_location(
    client: &Client,
    type_id: &str,
    location_id: &str,
) -> Result<models::TextProduct, Error> {
    http::request(client, "/products/types")
        .path_segment(type_id)
        .literal_path("locations")
        .path_segment(location_id)
        .literal_path("latest")
        .json(http::JsonMedia::JsonLd)
        .await
}

#[cfg(test)]
mod tests {
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers::method};

    use super::{
        ProductsQueryParams, get_latest_product_by_type_and_location, get_product,
        get_product_issuance_locations_by_type, get_product_locations, get_product_types,
        get_products_by_location, get_products_by_type, get_products_by_type_and_location,
        get_products_query,
    };
    use crate::{client::test_support::client_for, models::NwsForecastOfficeId};

    #[tokio::test]
    async fn products_by_type_and_location_encodes_dynamic_segments_and_requests_json_ld() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw("{}", "application/ld+json"))
            .expect(1)
            .mount(&server)
            .await;
        let client = client_for(&server);

        get_products_by_type_and_location(
            &client,
            "space slash/percent%",
            &NwsForecastOfficeId::Lwx,
        )
        .await
        .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests[0].url.path(),
            "/products/types/space%20slash%2Fpercent%25/locations/LWX"
        );
        assert_eq!(
            requests[0].headers["accept"].to_str().unwrap(),
            "application/ld+json"
        );
    }

    #[tokio::test]
    async fn products_query_preserves_csv_scalar_empty_and_omitted_values() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw("{}", "application/ld+json"))
            .expect(1)
            .mount(&server)
            .await;
        let client = client_for(&server);

        get_products_query(
            &client,
            ProductsQueryParams {
                location_ids: Some(vec![NwsForecastOfficeId::Lwx, NwsForecastOfficeId::Pqr]),
                start_time: Some(String::new()),
                end_time: None,
                office_ids: Some(vec![]),
                wmo_ids: Some(vec!["TTAA 00".to_owned(), "TT/BB%".to_owned()]),
                product_type_codes: None,
                limit: Some(0),
            },
        )
        .await
        .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.path(), "/products");
        assert_eq!(
            requests[0].url.query(),
            Some("location=LWX%2CPQR&start=&office=&wmoid=TTAA+00%2CTT%2FBB%25&limit=0")
        );
        let query_pairs = requests[0].url.query_pairs().collect::<Vec<_>>();
        for name in ["location", "start", "office", "wmoid", "limit"] {
            assert_eq!(
                query_pairs.iter().filter(|(key, _)| key == name).count(),
                1,
                "{name} must appear once"
            );
        }
        assert!(query_pairs.iter().all(|(key, _)| key != "end"));
        assert!(query_pairs.iter().all(|(key, _)| key != "type"));
        assert_eq!(
            requests[0].headers["accept"].to_str().unwrap(),
            "application/ld+json"
        );
    }

    #[tokio::test]
    async fn remaining_products_routes_encode_segments_and_request_json_ld() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw("{}", "application/ld+json"))
            .expect(7)
            .mount(&server)
            .await;
        let client = client_for(&server);

        get_products_by_location(&client, &NwsForecastOfficeId::Lwx)
            .await
            .unwrap();
        get_product(&client, "space slash/percent%").await.unwrap();
        get_product_locations(&client).await.unwrap();
        get_product_types(&client).await.unwrap();
        get_products_by_type(&client, "type slash/%").await.unwrap();
        get_product_issuance_locations_by_type(&client, "issue slash/%")
            .await
            .unwrap();
        get_latest_product_by_type_and_location(&client, "latest type/%", "latest location/%")
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        let routes_and_media = requests
            .iter()
            .map(|request| {
                (
                    request.url.path().to_owned(),
                    request.headers["accept"].to_str().unwrap().to_owned(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            routes_and_media,
            [
                (
                    "/products/locations/LWX/types".to_owned(),
                    "application/ld+json".to_owned(),
                ),
                (
                    "/products/space%20slash%2Fpercent%25".to_owned(),
                    "application/ld+json".to_owned(),
                ),
                (
                    "/products/locations".to_owned(),
                    "application/ld+json".to_owned(),
                ),
                (
                    "/products/types".to_owned(),
                    "application/ld+json".to_owned(),
                ),
                (
                    "/products/types/type%20slash%2F%25".to_owned(),
                    "application/ld+json".to_owned(),
                ),
                (
                    "/products/types/issue%20slash%2F%25/locations".to_owned(),
                    "application/ld+json".to_owned(),
                ),
                (
                    "/products/types/latest%20type%2F%25/locations/latest%20location%2F%25/latest"
                        .to_owned(),
                    "application/ld+json".to_owned(),
                ),
            ]
        );
    }
}
