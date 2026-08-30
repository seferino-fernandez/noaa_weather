//! NWS text products (Area Forecast Discussions, watches, warnings, etc.).
//!
//! Covers the `/products` endpoints for querying, listing, and retrieving
//! the full text of NWS-issued products by type, location, or issuance time.

use super::{Error, configuration, http};
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
/// * `configuration`: The API client configuration.
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
    configuration: &configuration::Configuration,
    location_id: &models::NwsForecastOfficeId,
) -> Result<models::TextProductTypeCollection, Error> {
    let uri_str = format!(
        "/products/locations/{locationId}/types",
        locationId = location_id
    );
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}

/// Returns a specific NWS text product by its unique product ID.
///
/// Corresponds to the `/products/{productId}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
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
pub async fn get_product(
    configuration: &configuration::Configuration,
    product_id: &str,
) -> Result<models::TextProduct, Error> {
    let uri_str = format!(
        "/products/{productId}",
        productId = crate::apis::urlencode(product_id)
    );
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}

/// Returns a list of valid NWS text product issuance locations.
///
/// Corresponds to the `/products/locations` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
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
    configuration: &configuration::Configuration,
) -> Result<models::TextProductLocationCollection, Error> {
    let uri_str = "/products/locations".to_owned();
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}

/// Returns a list of valid NWS text product types and their codes.
///
/// Corresponds to the `/products/types` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
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
    configuration: &configuration::Configuration,
) -> Result<models::TextProductTypeCollection, Error> {
    let uri_str = "/products/types".to_owned();
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}

/// Returns a list of text products based on specified query parameters.
///
/// Corresponds to the `/products` endpoint.
/// Allows filtering by location, time range, office, WMO ID, and product type.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
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
    configuration: &configuration::Configuration,
    params: ProductsQueryParams,
) -> Result<models::TextProductCollection, Error> {
    let uri_str = "/products".to_owned();
    let mut req_builder = http::get(configuration, &uri_str);

    if let Some(param_value) = params.location_ids {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("location".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "location",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }
    if let Some(param_value) = params.start_time {
        req_builder = req_builder.query(&[("start", &param_value)]);
    }
    if let Some(param_value) = params.end_time {
        req_builder = req_builder.query(&[("end", &param_value)]);
    }
    if let Some(param_value) = params.office_ids {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("office".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "office",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }
    if let Some(param_value) = params.wmo_ids {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("wmoid".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "wmoid",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }
    if let Some(param_value) = params.product_type_codes {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("type".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "type",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }
    if let Some(param_value) = params.limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }

    req_builder.json().await
}

/// Returns a list of text products of a specific type.
///
/// Corresponds to the `/products/types/{typeId}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
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
    configuration: &configuration::Configuration,
    type_id: &str,
) -> Result<models::TextProductCollection, Error> {
    let uri_str = format!(
        "/products/types/{typeId}",
        typeId = crate::apis::urlencode(type_id)
    );
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}

/// Returns a list of text products of a specific type for a specific issuance location.
///
/// Corresponds to the `/products/types/{typeId}/locations/{locationId}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
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
    configuration: &configuration::Configuration,
    type_id: &str,
    location_id: &models::NwsForecastOfficeId,
) -> Result<models::TextProductCollection, Error> {
    let uri_str = format!(
        "/products/types/{typeId}/locations/{locationId}",
        typeId = type_id,
        locationId = location_id
    );
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}

/// Returns a list of valid text product issuance locations for a given product type.
///
/// Corresponds to the `/products/types/{typeId}/locations` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
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
    configuration: &configuration::Configuration,
    type_id: &str,
) -> Result<models::TextProductLocationCollection, Error> {
    let uri_str = format!(
        "/products/types/{typeId}/locations",
        typeId = crate::apis::urlencode(type_id)
    );
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}

/// Returns the latest text product of a specific type for a specific issuance location.
///
/// Corresponds to the `/products/types/{typeId}/locations/{locationId}/latest` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
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
    configuration: &configuration::Configuration,
    type_id: &str,
    location_id: &str,
) -> Result<models::TextProduct, Error> {
    let uri_str = format!(
        "/products/types/{type_id}/locations/{location_id}/latest",
        type_id = crate::apis::urlencode(type_id),
        location_id = crate::apis::urlencode(location_id)
    );
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}
