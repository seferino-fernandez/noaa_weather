use anyhow::{Result, anyhow};
use clap::{Args, Subcommand};
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::apis::products as products_api;
use noaa_weather_client::apis::products::ProductsQueryParams;
use serde_json::Value;

/// Arguments for commands requiring a product issuance location ID.
#[derive(Args, Debug)]
pub struct LocationProductsArgs {
    /// Product issuance location ID (e.g., LWX, OKX).
    /// See `locations` subcommand for a list of valid IDs.
    #[arg(long)]
    location_id: String,
}

/// Arguments for commands requiring a specific product ID.
#[derive(Args, Debug)]
pub struct ProductArgs {
    /// Unique NWS text product identifier.
    /// Product IDs can be found in the output of the `list` subcommand.
    #[arg(long)]
    product_id: String,
}

/// Arguments for querying a list of NWS text products.
#[derive(Args, Debug)]
pub struct ProductsListArgs {
    /// Filter by product issuance location ID(s) (comma-separated).
    #[arg(long, value_delimiter = ',')]
    location: Option<Vec<String>>,

    /// Filter by start time (ISO 8601 format, e.g., "2023-10-27T12:00:00Z").
    #[arg(long)]
    start: Option<String>,

    /// Filter by end time (ISO 8601 format).
    #[arg(long)]
    end: Option<String>,

    /// Filter by NWS office ID(s) (typically WFO ID, comma-separated).
    #[arg(long, value_delimiter = ',')]
    office: Option<Vec<String>>,

    /// Filter by WMO header ID(s) (comma-separated).
    #[arg(long, value_delimiter = ',')]
    wmo_id: Option<Vec<String>>,

    /// Filter by product type code(s) (e.g., AFD, HWO, comma-separated).
    /// See `types` subcommand for valid codes.
    #[arg(long, value_name = "TYPE", value_delimiter = ',')]
    r#type: Option<Vec<String>>,

    /// Limit the number of results returned by the API.
    #[arg(long, default_value_t = 500)]
    limit: i32,
}

/// Arguments for commands requiring a product type ID.
#[derive(Args, Debug)]
pub struct ProductsTypeArgs {
    /// Product type ID (e.g., AFD, HWO).
    /// See `types` subcommand for valid codes.
    #[arg(long)]
    type_id: String,
}

/// Arguments for commands requiring both a product type ID and location ID.
#[derive(Args, Debug)]
pub struct ProductsTypeLocationArgs {
    /// Product type ID (e.g., AFD, HWO).
    #[arg(long)]
    type_id: String,

    /// Product issuance location ID (e.g., LWX, OKX).
    #[arg(long)]
    location_id: String,
}

/// Arguments for listing locations associated with a product type.
#[derive(Args, Debug)]
pub struct ProductsTypeLocationsArgs {
    /// Product type ID (e.g., AFD, HWO).
    #[arg(long)]
    type_id: String,
}

/// Access information about NWS text products.
#[derive(Subcommand, Debug)]
pub enum ProductCommands {
    /// Get available product types for a specific issuance location.
    ///
    /// Example: `noaa-weather products products-by-location --location-id LWX`
    #[clap(name = "products-by-location")]
    LocationProducts(LocationProductsArgs),
    /// Get a specific text product by its ID.
    ///
    /// Example: `noaa-weather products product --product-id "NWS-PRODUCT-ID"`
    Product(ProductArgs),
    /// List all available text product issuance locations and their names.
    ///
    /// Example: `noaa-weather products locations`
    #[clap(name = "locations")]
    Locations,
    /// List all available text product types and their associated codes.
    ///
    /// Example: `noaa-weather products types`
    #[clap(name = "types")]
    Types,
    /// Query text products with various filters (location, time, office, type, etc.).
    ///
    /// Example: `noaa-weather products list --location LWX --type AFD --limit 10`
    #[clap(name = "list")]
    ProductsList(ProductsListArgs),
    /// List recent text products of a specific type.
    ///
    /// Example: `noaa-weather products type --type-id AFD`
    #[clap(name = "type")]
    ProductsType(ProductsTypeArgs),
    /// List recent text products of a specific type for a specific issuance location.
    ///
    /// Example: `noaa-weather products types-by-location --type-id AFD --location-id LWX`
    #[clap(name = "types-by-location")]
    ProductsTypeLocation(ProductsTypeLocationArgs),
    /// List valid issuance locations for a specific product type.
    ///
    /// Example: `noaa-weather products locations-by-type --type-id HWO`
    #[clap(name = "locations-by-type")]
    ProductsTypeLocations(ProductsTypeLocationsArgs),
}

/// Handles the execution of product-related subcommands.
///
/// Dispatches the command to the appropriate API function based on the
/// provided `ProductCommands` variant and arguments.
///
/// # Arguments
///
/// * `command` - The specific product subcommand and its arguments to execute.
/// * `config` - The application configuration containing API details.
///
/// # Returns
///
/// A `Result` containing the JSON `Value` of the API response on success,
/// or an `anyhow::Error` if an error occurs during the API call or processing.
pub async fn handle_command(command: ProductCommands, config: &Configuration) -> Result<Value> {
    match command {
        ProductCommands::LocationProducts(args) => {
            let result = products_api::get_products_by_location(config, &args.location_id)
                .await
                .map_err(|e| anyhow!("getting location products: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        ProductCommands::Product(args) => {
            let result = products_api::get_product(config, &args.product_id)
                .await
                .map_err(|e| anyhow!("getting product: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        ProductCommands::Locations => {
            let result = products_api::get_product_locations(config)
                .await
                .map_err(|e| anyhow!("getting product locations: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        ProductCommands::Types => {
            let result = products_api::get_product_types(config)
                .await
                .map_err(|e| anyhow!("getting product types: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        ProductCommands::ProductsList(args) => {
            let params = ProductsQueryParams {
                location: args.location,
                start: args.start,
                end: args.end,
                office: args.office,
                wmoid: args.wmo_id,
                product_type: args.r#type,
                limit: Some(args.limit),
            };
            let result = products_api::get_products_query(config, params)
                .await
                .map_err(|e| anyhow!("querying products: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        ProductCommands::ProductsType(args) => {
            let result = products_api::get_products_by_type(config, &args.type_id)
                .await
                .map_err(|e| anyhow!("getting products by type: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        ProductCommands::ProductsTypeLocation(args) => {
            let result = products_api::get_products_by_type_and_location(
                config,
                &args.type_id,
                &args.location_id,
            )
            .await
            .map_err(|e| anyhow!("getting products by type and location: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        ProductCommands::ProductsTypeLocations(args) => {
            let result =
                products_api::get_product_issuance_locations_by_type(config, &args.type_id)
                    .await
                    .map_err(|e| anyhow!("getting locations for product type: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
    }
}
