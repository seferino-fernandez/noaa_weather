use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};
use noaa_weather_client::models::{
    TextProduct, TextProductCollection, TextProductLocationCollection, TextProductTypeCollection,
};

use crate::utils::format::format_datetime_human_readable;

/// Formats a `TextProduct` into a `comfy_table::Table`.
///
/// This function constructs a table displaying various attributes of a `TextProduct`.
///
pub fn create_product_table(product: &TextProduct) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("ID")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("WMO Collective ID")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Issuing Office")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Issuance Time")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Product Code")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Product Name")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Product Text")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);
    let product_id = product
        .id
        .as_deref()
        .filter(|product_id| !product_id.is_empty())
        .unwrap_or("N/A");

    let issuance_time = product
        .issuance_time
        .as_deref()
        .filter(|issuance_time| !issuance_time.is_empty());
    let issuance_time_readable = format_datetime_human_readable(issuance_time);

    table.add_row(vec![
        Cell::new(product_id),
        Cell::new(product.wmo_collective_id.as_deref().unwrap_or("N/A")),
        Cell::new(product.issuing_office.as_deref().unwrap_or("N/A")),
        Cell::new(issuance_time_readable),
        Cell::new(product.product_code.as_deref().unwrap_or("N/A")),
        Cell::new(product.product_name.as_deref().unwrap_or("N/A")),
        Cell::new(product.product_text.as_deref().unwrap_or("N/A")),
    ]);

    table
}

pub fn create_product_types_table(product_types: &TextProductTypeCollection) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("Product Code")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Product Name")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);

    for product_type in product_types.at_graph.iter().flatten() {
        table.add_row(vec![
            Cell::new(&product_type.product_code),
            Cell::new(&product_type.product_name),
        ]);
    }

    table
}

pub fn create_products_table(products: &TextProductCollection) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("ID")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("WMO Collective ID")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Issuing Office")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Issuance Time")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Product Code")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Product Name")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);

    for product in products.at_graph.iter().flatten() {
        let product_id = product
            .id
            .as_deref()
            .filter(|product_id| !product_id.is_empty())
            .unwrap_or("N/A");

        let issuance_time = product
            .issuance_time
            .as_deref()
            .filter(|issuance_time| !issuance_time.is_empty());
        let issuance_time_readable = format_datetime_human_readable(issuance_time);

        table.add_row(vec![
            Cell::new(product_id),
            Cell::new(product.wmo_collective_id.as_deref().unwrap_or("N/A")),
            Cell::new(product.issuing_office.as_deref().unwrap_or("N/A")),
            Cell::new(issuance_time_readable),
            Cell::new(product.product_code.as_deref().unwrap_or("N/A")),
            Cell::new(product.product_name.as_deref().unwrap_or("N/A")),
        ]);
    }

    table
}

pub fn create_products_locations_table(product_locations: &TextProductLocationCollection) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("Location ID")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Location Name")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);

    for product_location in product_locations.locations.iter().flatten() {
        let location_id = product_location.0;
        let location_name = product_location.1.clone().unwrap_or("N/A".to_owned());
        table.add_row(vec![Cell::new(location_id), Cell::new(location_name)]);
    }

    table
}
