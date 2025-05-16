use super::{ContentType, Error, configuration};
use crate::apis::ResponseContent;
use crate::models;
use reqwest;
use serde::de::Error as _;
use serde::{Deserialize, Serialize};

/// Errors that can occur when calling the [`get_products_by_location`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LocationProductsError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_product`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProductError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_product_locations`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProductLocationsError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_product_types`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProductTypesError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_products_query`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProductsQueryError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_products_by_type`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProductsTypeError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_products_by_type_and_location`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProductsTypeLocationError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_product_issuance_locations_by_type`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProductsTypeLocationsError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Parameters for the [`get_products_query`] function.
///
/// This struct encapsulates the query parameters for filtering text products.
#[derive(Debug, Clone, Default)]
pub struct ProductsQueryParams {
    /// Filter by issuance location ID (e.g., "LWX", "PQR").
    pub location_ids: Option<Vec<String>>,
    /// Start time for the query period (ISO 8601 format).
    pub start_time: Option<String>,
    /// End time for the query period (ISO 8601 format).
    pub end_time: Option<String>,
    /// Filter by issuing office ID (typically WFO ID, e.g., "LWX", "PQR").
    pub office_ids: Option<Vec<String>>,
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
/// Returns an [`Error<LocationProductsError>`] if the request fails or the response
/// cannot be parsed.
pub async fn get_products_by_location(
    configuration: &configuration::Configuration,
    location_id: &str,
) -> Result<models::TextProductTypeCollection, Error<LocationProductsError>> {
    let uri_str = format!(
        "{}/products/locations/{locationId}/types",
        configuration.base_path,
        locationId = crate::apis::urlencode(location_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header(reqwest::header::USER_AGENT, value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|header| header.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `models::TextProductTypeCollection`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `models::TextProductTypeCollection`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::TextProductTypeCollection`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<LocationProductsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
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
/// Returns an [`Error<ProductError>`] if the request fails (e.g., product not found)
/// or the response cannot be parsed.
pub async fn get_product(
    configuration: &configuration::Configuration,
    product_id: &str,
) -> Result<models::TextProduct, Error<ProductError>> {
    let uri_str = format!(
        "{}/products/{productId}",
        configuration.base_path,
        productId = crate::apis::urlencode(product_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header(reqwest::header::USER_AGENT, value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|header| header.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `models::TextProduct`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `models::TextProduct`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::TextProduct`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ProductError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
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
/// Returns an [`Error<ProductLocationsError>`] if the request fails or the response
/// cannot be parsed.
pub async fn get_product_locations(
    configuration: &configuration::Configuration,
) -> Result<models::TextProductLocationCollection, Error<ProductLocationsError>> {
    let uri_str = format!("{}/products/locations", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header(reqwest::header::USER_AGENT, value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|header| header.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `models::TextProductLocationCollection`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `models::TextProductLocationCollection`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::TextProductLocationCollection`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ProductLocationsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
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
/// Returns an [`Error<ProductTypesError>`] if the request fails or the response
/// cannot be parsed.
pub async fn get_product_types(
    configuration: &configuration::Configuration,
) -> Result<models::TextProductTypeCollection, Error<ProductTypesError>> {
    let uri_str = format!("{}/products/types", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header(reqwest::header::USER_AGENT, value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|header| header.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `models::TextProductTypeCollection`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `models::TextProductTypeCollection`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::TextProductTypeCollection`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ProductTypesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
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
/// Returns an [`Error<ProductsQueryError>`] if the request fails or the response
/// cannot be parsed.
pub async fn get_products_query(
    configuration: &configuration::Configuration,
    params: ProductsQueryParams,
) -> Result<models::TextProductCollection, Error<ProductsQueryError>> {
    let uri_str = format!("{}/products", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = params.location_ids {
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
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = params.start_time {
        req_builder = req_builder.query(&[("start", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.end_time {
        req_builder = req_builder.query(&[("end", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.office_ids {
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
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = params.wmo_ids {
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
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = params.product_type_codes {
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
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = params.limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header(reqwest::header::USER_AGENT, value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|header| header.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `models::TextProductCollection`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `models::TextProductCollection`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::TextProductCollection`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ProductsQueryError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
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
/// Returns an [`Error<ProductsTypeError>`] if the request fails or the response
/// cannot be parsed.
pub async fn get_products_by_type(
    configuration: &configuration::Configuration,
    type_id: &str,
) -> Result<models::TextProductCollection, Error<ProductsTypeError>> {
    let uri_str = format!(
        "{}/products/types/{typeId}",
        configuration.base_path,
        typeId = crate::apis::urlencode(type_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header(reqwest::header::USER_AGENT, value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|header| header.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `models::TextProductCollection`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `models::TextProductCollection`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::TextProductCollection`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ProductsTypeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
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
/// Returns an [`Error<ProductsTypeLocationError>`] if the request fails or the response
/// cannot be parsed.
pub async fn get_products_by_type_and_location(
    configuration: &configuration::Configuration,
    type_id: &str,
    location_id: &str,
) -> Result<models::TextProductCollection, Error<ProductsTypeLocationError>> {
    let uri_str = format!(
        "{}/products/types/{typeId}/locations/{locationId}",
        configuration.base_path,
        typeId = crate::apis::urlencode(type_id),
        locationId = crate::apis::urlencode(location_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header(reqwest::header::USER_AGENT, value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|header| header.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `models::TextProductCollection`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `models::TextProductCollection`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::TextProductCollection`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ProductsTypeLocationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
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
/// Returns an [`Error<ProductsTypeLocationsError>`] if the request fails or the response
/// cannot be parsed.
pub async fn get_product_issuance_locations_by_type(
    configuration: &configuration::Configuration,
    type_id: &str,
) -> Result<models::TextProductLocationCollection, Error<ProductsTypeLocationsError>> {
    let uri_str = format!(
        "{}/products/types/{typeId}/locations",
        configuration.base_path,
        typeId = crate::apis::urlencode(type_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header(reqwest::header::USER_AGENT, value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|header| header.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `models::TextProductLocationCollection`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `models::TextProductLocationCollection`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::TextProductLocationCollection`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ProductsTypeLocationsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
