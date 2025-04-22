use anyhow::{Result, anyhow};
use clap::{Args, Subcommand};
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::apis::products as products_api;
use serde_json::Value;

#[derive(Args, Debug)]
pub struct LocationProductsArgs {
    /// Product issuance location ID (e.g., LWX, OKX).
    #[arg(long)]
    location_id: String,
}

#[derive(Args, Debug)]
pub struct ProductArgs {
    /// Specific product ID.
    #[arg(long)]
    product_id: String,
}

#[derive(Args, Debug)]
pub struct ProductsListArgs {
    /// Filter by product issuance location ID(s).
    #[arg(long)]
    location: Option<Vec<String>>,

    /// Filter by start time (ISO 8601 format).
    #[arg(long)]
    start: Option<String>,

    /// Filter by end time (ISO 8601 format).
    #[arg(long)]
    end: Option<String>,

    /// Filter by NWS office ID(s).
    #[arg(long)]
    office: Option<Vec<String>>,

    /// Filter by WMO header ID(s).
    #[arg(long)]
    wmo_id: Option<Vec<String>>,

    /// Filter by product type code(s).
    #[arg(long, value_name = "TYPE")]
    r#type: Option<Vec<String>>,

    /// Limit the number of results.
    #[arg(long, default_value_t = 500)]
    limit: i32,
}

#[derive(Args, Debug)]
pub struct ProductsTypeArgs {
    /// Product type ID (e.g., AFD, HWO).
    #[arg(long)]
    type_id: String,
}

#[derive(Args, Debug)]
pub struct ProductsTypeLocationArgs {
    /// Product type ID (e.g., AFD, HWO).
    #[arg(long)]
    type_id: String,

    /// Product issuance location ID (e.g., LWX, OKX).
    #[arg(long)]
    location_id: String,
}

#[derive(Args, Debug)]
pub struct ProductsTypeLocationsArgs {
    /// Product type ID (e.g., AFD, HWO).
    #[arg(long)]
    type_id: String,
}

/// Access NWS text product information.
#[derive(Subcommand, Debug)]
pub enum ProductCommands {
    /// Get product types for a specific location.
    #[clap(name = "products-by-location")]
    LocationProducts(LocationProductsArgs),
    /// Get a specific text product by its ID.
    Product(ProductArgs),
    /// List all available text product issuance locations.
    #[clap(name = "locations")]
    Locations,
    /// List all available text product types and codes.
    #[clap(name = "types")]
    Types,
    /// Query text products with various filters.
    #[clap(name = "list")]
    ProductsList(ProductsListArgs),
    /// List text products of a specific type.
    #[clap(name = "type")]
    ProductsType(ProductsTypeArgs),
    /// List text products of a specific type for a specific location.
    #[clap(name = "types-by-location")]
    ProductsTypeLocation(ProductsTypeLocationArgs),
    /// List valid issuance locations for a specific product type.
    #[clap(name = "locations-by-type")]
    ProductsTypeLocations(ProductsTypeLocationsArgs),
}

// Placeholder for the command handler
pub async fn handle_command(command: ProductCommands, config: &Configuration) -> Result<Value> {
    match command {
        ProductCommands::LocationProducts(args) => {
            let result = products_api::location_products(config, &args.location_id)
                .await
                .map_err(|e| anyhow!("getting location products: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        ProductCommands::Product(args) => {
            let result = products_api::product(config, &args.product_id)
                .await
                .map_err(|e| anyhow!("getting product: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        ProductCommands::Locations => {
            let result = products_api::product_locations(config)
                .await
                .map_err(|e| anyhow!("getting product locations: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        ProductCommands::Types => {
            let result = products_api::product_types(config)
                .await
                .map_err(|e| anyhow!("getting product types: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        ProductCommands::ProductsList(args) => {
            let result = products_api::products_query(
                config,
                args.location,
                args.start,
                args.end,
                args.office,
                args.wmo_id,
                args.r#type,
                Some(args.limit),
            )
            .await
            .map_err(|e| anyhow!("querying products: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        ProductCommands::ProductsType(args) => {
            let result = products_api::products_type(config, &args.type_id)
                .await
                .map_err(|e| anyhow!("getting products by type: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        ProductCommands::ProductsTypeLocation(args) => {
            let result =
                products_api::products_type_location(config, &args.type_id, &args.location_id)
                    .await
                    .map_err(|e| anyhow!("getting products by type and location: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        ProductCommands::ProductsTypeLocations(args) => {
            let result = products_api::products_type_locations(config, &args.type_id)
                .await
                .map_err(|e| anyhow!("getting locations for product type: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
    }
}
