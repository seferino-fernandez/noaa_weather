use anyhow::Result;
use clap::{Args, Subcommand};
use jiff::Timestamp;
use noaa_weather_client::apis::products::ProductsQuery;
use noaa_weather_client::{Client, OfficeId, ProductId, ProductTypeCode};

use super::parse;
use crate::output::Output;

/// Arguments for commands requiring a product issuance location ID.
#[derive(Args, Debug, Clone)]
pub struct LocationProductsArgs {
    /// Product issuance location ID (e.g., LWX, OKX).
    /// See `locations` subcommand for a list of valid IDs.
    #[arg(long, long_help = parse::office_long_help("Product issuance location ID"))]
    location_id: OfficeId,
}

/// Arguments for commands requiring a specific product ID.
#[derive(Args, Debug, Clone)]
pub struct ProductMetadataArgs {
    /// Unique NWS text product identifier.
    /// Product IDs can be found in the output of the `list` subcommand.
    #[arg(long)]
    id: ProductId,
}

/// Arguments for querying a list of NWS text products.
#[derive(Args, Debug, Clone)]
pub struct ProductsListArgs {
    /// Filter by product issuance location ID(s) (comma-separated).
    #[arg(long, value_delimiter = ',', long_help = parse::office_long_help("Product issuance location IDs, comma-separated"))]
    location_ids: Vec<OfficeId>,

    /// Filter by start time (RFC 3339 timestamp or relative age such as 6h).
    #[arg(long, value_parser = parse::time, value_name = "TIME", long_help = parse::TIME_HELP)]
    start_time: Option<Timestamp>,

    /// Filter by end time (RFC 3339 timestamp or relative age such as 1h).
    #[arg(long, value_parser = parse::time, value_name = "TIME", long_help = parse::TIME_HELP)]
    end_time: Option<Timestamp>,

    /// Filter by NWS office ID(s) (typically WFO ID, comma-separated).
    #[arg(long, value_delimiter = ',', long_help = parse::office_long_help("Issuing office IDs, comma-separated"))]
    office_ids: Vec<OfficeId>,

    /// Filter by WMO header ID(s) (comma-separated).
    #[arg(long, value_delimiter = ',')]
    wmo_ids: Vec<String>,

    /// Filter by product type code(s) (e.g., AFD, HWO, comma-separated).
    /// See `types` subcommand for valid codes.
    #[arg(long, value_name = "TYPE", value_delimiter = ',')]
    product_type_codes: Vec<ProductTypeCode>,

    /// Limit the number of results returned by the API (1 to 500).
    #[arg(long, default_value_t = 500, value_parser = clap::value_parser!(u16).range(1..=500))]
    limit: u16,
}

/// Arguments for commands requiring a product type ID.
#[derive(Args, Debug, Clone)]
pub struct ProductsTypeArgs {
    /// Product type ID (e.g., AFD, HWO).
    /// See `types` subcommand for valid codes.
    #[arg(long)]
    type_id: ProductTypeCode,
}

/// Arguments for commands requiring both a product type ID and location ID.
#[derive(Args, Debug, Clone)]
pub struct ProductsTypeLocationArgs {
    /// Product type ID (e.g., AFD, HWO).
    #[arg(long)]
    type_id: ProductTypeCode,

    /// Product issuance location ID (e.g., LWX, OKX).
    #[arg(long, long_help = parse::office_long_help("Product issuance location ID"))]
    location_id: OfficeId,
}

/// Arguments for listing locations associated with a product type.
#[derive(Args, Debug, Clone)]
pub struct ProductsTypeLocationsArgs {
    /// Product type ID (e.g., AFD, HWO).
    #[arg(long)]
    type_id: ProductTypeCode,
}

/// Arguments for getting the latest product by type and location.
#[derive(Args, Debug, Clone)]
pub struct LatestProductArgs {
    /// Product type ID (e.g., AFD, HWO).
    #[arg(long)]
    pub type_id: ProductTypeCode,
    /// Product issuance location ID (e.g., LWX, PSR).
    #[arg(long, long_help = parse::office_long_help("Product issuance location ID"))]
    pub location_id: OfficeId,
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
/// Dispatches the command to the matching `client.products()` method based
/// on the provided `ProductCommands` variant and arguments.
///
/// # Arguments
///
/// * `command` - The specific product subcommand and its arguments to execute.
/// * `output` - The configured output policy.
/// * `client` - The NOAA API client.
///
pub async fn handle_command(
    command: &ProductCommands,
    output: &Output,
    client: &Client,
) -> Result<()> {
    let products = client.products();
    match command {
        ProductCommands::LocationProducts(args) => {
            output
                .show(
                    format!("getting product types for location {}", args.location_id),
                    products.types_for_location(&args.location_id),
                )
                .await
        }
        ProductCommands::Metadata(args) => {
            output
                .show(
                    format!("getting product {}", args.id),
                    products.get(&args.id),
                )
                .await
        }
        ProductCommands::Locations => {
            output
                .show("getting product locations", products.locations())
                .await
        }
        ProductCommands::Types => output.show("getting product types", products.types()).await,
        ProductCommands::ProductsList(args) => {
            let query = ProductsQuery {
                location_ids: args.location_ids.clone(),
                start: args.start_time,
                end: args.end_time,
                office_ids: args.office_ids.clone(),
                wmo_ids: args.wmo_ids.clone(),
                product_type_codes: args.product_type_codes.clone(),
                limit: Some(args.limit),
            };
            output
                .show("querying products", products.search(&query))
                .await
        }
        ProductCommands::ProductsType(args) => {
            output
                .show(
                    format!("getting products of type {}", args.type_id),
                    products.by_type(&args.type_id),
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
                    products.by_type_and_location(&args.type_id, &args.location_id),
                )
                .await
        }
        ProductCommands::ProductsTypeLocations(args) => {
            output
                .show(
                    format!("getting locations for product type {}", args.type_id),
                    products.locations_for_type(&args.type_id),
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
                    products.latest(&args.type_id, &args.location_id),
                )
                .await
        }
    }
}
