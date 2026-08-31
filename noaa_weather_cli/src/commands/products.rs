use anyhow::Result;
use clap::{Args, Subcommand};
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::apis::products as products_api;
use noaa_weather_client::apis::products::ProductsQueryParams;
use noaa_weather_client::models::NwsForecastOfficeId;

use crate::output::Output;

/// Arguments for commands requiring a product issuance location ID.
#[derive(Args, Debug, Clone)]
pub struct LocationProductsArgs {
    /// Product issuance location ID (e.g., LWX, OKX).
    /// See `locations` subcommand for a list of valid IDs.
    #[arg(long, value_enum)]
    location_id: NwsForecastOfficeId,
}

/// Arguments for commands requiring a specific product ID.
#[derive(Args, Debug, Clone)]
pub struct ProductMetadataArgs {
    /// Unique NWS text product identifier.
    /// Product IDs can be found in the output of the `list` subcommand.
    #[arg(long)]
    id: String,
}

/// Arguments for querying a list of NWS text products.
#[derive(Args, Debug, Clone)]
pub struct ProductsListArgs {
    /// Filter by product issuance location ID(s) (comma-separated).
    #[arg(long, value_delimiter = ',', value_enum)]
    location_ids: Option<Vec<NwsForecastOfficeId>>,

    /// Filter by start time (ISO 8601 format, e.g., "2023-10-27T12:00:00Z").
    #[arg(long)]
    start_time: Option<String>,

    /// Filter by end time (ISO 8601 format).
    #[arg(long)]
    end_time: Option<String>,

    /// Filter by NWS office ID(s) (typically WFO ID, comma-separated).
    #[arg(long, value_delimiter = ',', value_enum)]
    office_ids: Option<Vec<NwsForecastOfficeId>>,

    /// Filter by WMO header ID(s) (comma-separated).
    #[arg(long, value_delimiter = ',')]
    wmo_ids: Option<Vec<String>>,

    /// Filter by product type code(s) (e.g., AFD, HWO, comma-separated).
    /// See `types` subcommand for valid codes.
    #[arg(long, value_name = "TYPE", value_delimiter = ',')]
    product_type_codes: Option<Vec<String>>,

    /// Limit the number of results returned by the API.
    #[arg(long, default_value_t = 500, value_parser = clap::value_parser!(i32).range(1..=500))]
    limit: i32,
}

/// Arguments for commands requiring a product type ID.
#[derive(Args, Debug, Clone)]
pub struct ProductsTypeArgs {
    /// Product type ID (e.g., AFD, HWO).
    /// See `types` subcommand for valid codes.
    #[arg(long)]
    type_id: String,
}

/// Arguments for commands requiring both a product type ID and location ID.
#[derive(Args, Debug, Clone)]
pub struct ProductsTypeLocationArgs {
    /// Product type ID (e.g., AFD, HWO).
    #[arg(long)]
    type_id: String,

    /// Product issuance location ID (e.g., LWX, OKX).
    #[arg(long, value_enum)]
    location_id: NwsForecastOfficeId,
}

/// Arguments for listing locations associated with a product type.
#[derive(Args, Debug, Clone)]
pub struct ProductsTypeLocationsArgs {
    /// Product type ID (e.g., AFD, HWO).
    #[arg(long)]
    type_id: String,
}

/// Arguments for getting the latest product by type and location.
#[derive(Args, Debug, Clone)]
pub struct LatestProductArgs {
    /// Product type ID (e.g., AFD, HWO).
    #[arg(long)]
    pub type_id: String,
    /// Product issuance location ID (e.g., LWX, PSR).
    #[arg(long)]
    pub location_id: String,
}

/// Access information about NWS text products.
#[derive(Subcommand, Debug, Clone)]
pub enum ProductCommands {
    /// Get available product types for a specific issuance location.
    ///
    /// Example: `noaa-weather products products-by-location --location-id PSR`
    #[clap(name = "products-by-location")]
    LocationProducts(LocationProductsArgs),
    /// Get a specific text product by its ID.
    ///
    /// Example: `noaa-weather products metadata --id "NWS-PRODUCT-ID"`
    Metadata(ProductMetadataArgs),
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
    /// Query text products with various filters (location ids, time, office ids, wmo ids, product type codes, etc.).
    ///
    /// Example: `noaa-weather products list --location-ids LWX --product-type-codes AFD --limit 10`
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
    /// Get the latest text product of a specific type for a specific issuance location.
    ///
    /// Example: `noaa-weather products latest --type-id AFD --location-id PSR`
    #[clap(name = "latest")]
    Latest(LatestProductArgs),
}

/// Handles the execution of product-related subcommands.
///
/// Dispatches the command to the appropriate API function based on the
/// provided `ProductCommands` variant and arguments.
///
/// # Arguments
///
/// * `command` - The specific product subcommand and its arguments to execute.
/// * `output` - The configured output policy.
/// * `config` - The application configuration containing API details.
///
pub async fn handle_command(
    command: &ProductCommands,
    output: &Output,
    config: &Configuration,
) -> Result<()> {
    match command {
        ProductCommands::LocationProducts(args) => {
            output
                .show(
                    "getting products for a location",
                    products_api::get_products_by_location(config, &args.location_id),
                )
                .await
        }
        ProductCommands::Metadata(args) => {
            output
                .show(
                    format!("getting product {}", args.id),
                    products_api::get_product(config, &args.id),
                )
                .await
        }
        ProductCommands::Locations => {
            output
                .show(
                    "getting product locations",
                    products_api::get_product_locations(config),
                )
                .await
        }
        ProductCommands::Types => {
            output
                .show(
                    "getting product types",
                    products_api::get_product_types(config),
                )
                .await
        }
        ProductCommands::ProductsList(args) => {
            let params = ProductsQueryParams {
                location_ids: args.location_ids.clone(),
                start_time: args.start_time.clone(),
                end_time: args.end_time.clone(),
                office_ids: args.office_ids.clone(),
                wmo_ids: args.wmo_ids.clone(),
                product_type_codes: args.product_type_codes.clone(),
                limit: Some(args.limit),
            };
            output
                .show(
                    "querying products",
                    products_api::get_products_query(config, params),
                )
                .await
        }
        ProductCommands::ProductsType(args) => {
            output
                .show(
                    format!("getting products of type {}", args.type_id),
                    products_api::get_products_by_type(config, &args.type_id),
                )
                .await
        }
        ProductCommands::ProductsTypeLocation(args) => {
            output
                .show(
                    format!(
                        "getting products of type {} for location {}",
                        args.type_id, args.location_id
                    ),
                    products_api::get_products_by_type_and_location(
                        config,
                        &args.type_id,
                        &args.location_id,
                    ),
                )
                .await
        }
        ProductCommands::ProductsTypeLocations(args) => {
            output
                .show(
                    format!("getting locations for product type {}", args.type_id),
                    products_api::get_product_issuance_locations_by_type(config, &args.type_id),
                )
                .await
        }
        ProductCommands::Latest(args) => {
            output
                .show(
                    format!(
                        "getting latest product of type {} for location {}",
                        args.type_id, args.location_id
                    ),
                    products_api::get_latest_product_by_type_and_location(
                        config,
                        &args.type_id,
                        &args.location_id,
                    ),
                )
                .await
        }
    }
}
