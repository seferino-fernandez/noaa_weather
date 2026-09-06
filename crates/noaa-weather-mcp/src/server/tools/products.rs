//! Text product tools.

use noaa_weather_client::products::{
    ProductsQuery, TextProduct, TextProductCollection, TextProductLocationCollection,
    TextProductTypeCollection,
};
use noaa_weather_client::{OfficeId, ProductId, ProductTypeCode};
use rmcp::Json;
use rmcp::handler::server::wrapper::Parameters;
use schemars::JsonSchema;
use serde::Deserialize;

use super::super::NoaaWeatherServer;
use super::error::ToolFailure;

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code, reason = "used when the family router is composed")]
struct ProductArguments {
    product_id: ProductId,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code, reason = "used when the family router is composed")]
struct LocationArguments {
    location_id: OfficeId,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code, reason = "used when the family router is composed")]
struct ProductTypeArguments {
    product_type_code: ProductTypeCode,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code, reason = "used when the family router is composed")]
struct ProductTypeLocationArguments {
    product_type_code: ProductTypeCode,
    location_id: OfficeId,
}

#[rmcp::tool_router(router = products_router, vis = "pub(super)")]
impl NoaaWeatherServer {
    #[rmcp::tool(
        name = "products_search",
        description = "Return NWS text products matching optional location, time, office, WMO heading, product type, and limit filters.",
        annotations(
            title = "Search NWS Text Products",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn products_search(
        &self,
        Parameters(query): Parameters<ProductsQuery>,
    ) -> Result<Json<TextProductCollection>, ToolFailure> {
        self.client
            .products()
            .search(&query)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "products_get",
        description = "Return one NWS text product, including its full text, by server-issued product identifier.",
        annotations(
            title = "Get NWS Text Product",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn products_get(
        &self,
        Parameters(ProductArguments { product_id }): Parameters<ProductArguments>,
    ) -> Result<Json<TextProduct>, ToolFailure> {
        self.client
            .products()
            .get(&product_id)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "products_locations",
        description = "Return the catalog of NWS text-product issuance locations.",
        annotations(
            title = "List NWS Product Locations",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn products_locations(&self) -> Result<Json<TextProductLocationCollection>, ToolFailure> {
        self.client
            .products()
            .locations()
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "products_types",
        description = "Return the catalog of NWS text-product type codes and names.",
        annotations(
            title = "List NWS Product Types",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn products_types(&self) -> Result<Json<TextProductTypeCollection>, ToolFailure> {
        self.client
            .products()
            .types()
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "products_types_for_location",
        description = "Return the NWS text-product types issued at one location.",
        annotations(
            title = "List Product Types for Location",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn products_types_for_location(
        &self,
        Parameters(LocationArguments { location_id }): Parameters<LocationArguments>,
    ) -> Result<Json<TextProductTypeCollection>, ToolFailure> {
        self.client
            .products()
            .types_for_location(&location_id)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "products_by_type",
        description = "Return recent NWS text products of one product type from every location.",
        annotations(
            title = "List NWS Products by Type",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn products_by_type(
        &self,
        Parameters(ProductTypeArguments { product_type_code }): Parameters<ProductTypeArguments>,
    ) -> Result<Json<TextProductCollection>, ToolFailure> {
        self.client
            .products()
            .by_type(&product_type_code)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "products_by_type_and_location",
        description = "Return recent NWS text products of one product type from one issuance location.",
        annotations(
            title = "List Products by Type and Location",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn products_by_type_and_location(
        &self,
        Parameters(ProductTypeLocationArguments {
            product_type_code,
            location_id,
        }): Parameters<ProductTypeLocationArguments>,
    ) -> Result<Json<TextProductCollection>, ToolFailure> {
        self.client
            .products()
            .by_type_and_location(&product_type_code, &location_id)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "products_locations_for_type",
        description = "Return the NWS issuance locations that publish one text-product type.",
        annotations(
            title = "List Locations for Product Type",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn products_locations_for_type(
        &self,
        Parameters(ProductTypeArguments { product_type_code }): Parameters<ProductTypeArguments>,
    ) -> Result<Json<TextProductLocationCollection>, ToolFailure> {
        self.client
            .products()
            .locations_for_type(&product_type_code)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "products_latest",
        description = "Return the latest NWS text product of one type from one issuance location, including its full text.",
        annotations(
            title = "Get Latest NWS Text Product",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn products_latest(
        &self,
        Parameters(ProductTypeLocationArguments {
            product_type_code,
            location_id,
        }): Parameters<ProductTypeLocationArguments>,
    ) -> Result<Json<TextProduct>, ToolFailure> {
        self.client
            .products()
            .latest(&product_type_code, &location_id)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }
}
