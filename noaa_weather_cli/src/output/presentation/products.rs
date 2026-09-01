use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};
use noaa_weather_client::models::{
    TextProduct, TextProductCollection, TextProductLocationCollection, TextProductTypeCollection,
};

use super::{DefaultPresentation, DefaultPresenter, PresentationError};
use crate::output::PresentationDocument;

/// Formats a `TextProduct` into a `comfy_table::Table`.
///
/// This function constructs a table displaying various attributes of a `TextProduct`.
///
fn create_product_table(
    product: &TextProduct,
    presenter: &DefaultPresenter,
) -> Result<Table, PresentationError> {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
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
    let product_id = presenter.text(product.id.as_deref());
    let issuance_time_readable =
        presenter.timestamp("product.issuance_time", product.issuance_time.as_deref())?;

    table.add_row(vec![
        Cell::new(product_id),
        Cell::new(presenter.text(product.wmo_collective_id.as_deref())),
        Cell::new(presenter.text(product.issuing_office.as_deref())),
        Cell::new(issuance_time_readable),
        Cell::new(presenter.text(product.product_code.as_deref())),
        Cell::new(presenter.text(product.product_name.as_deref())),
        Cell::new(presenter.text(product.product_text.as_deref())),
    ]);

    Ok(table)
}

fn create_product_types_table(product_types: &TextProductTypeCollection) -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
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

fn create_products_table(
    products: &TextProductCollection,
    presenter: &DefaultPresenter,
) -> Result<Table, PresentationError> {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
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

    for (index, product) in products.at_graph.iter().flatten().enumerate() {
        let product_id = presenter.text(product.id.as_deref());
        let issuance_time_readable = presenter.timestamp(
            format!("products.at_graph[{index}].issuance_time"),
            product.issuance_time.as_deref(),
        )?;

        table.add_row(vec![
            Cell::new(product_id),
            Cell::new(presenter.text(product.wmo_collective_id.as_deref())),
            Cell::new(presenter.text(product.issuing_office.as_deref())),
            Cell::new(issuance_time_readable),
            Cell::new(presenter.text(product.product_code.as_deref())),
            Cell::new(presenter.text(product.product_name.as_deref())),
        ]);
    }

    Ok(table)
}

fn create_products_locations_table(
    product_locations: &TextProductLocationCollection,
    presenter: &DefaultPresenter,
) -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
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
        let location_name = presenter.text(product_location.1.as_deref());
        table.add_row(vec![Cell::new(location_id), Cell::new(location_name)]);
    }

    table
}

impl DefaultPresentation for TextProduct {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_product_table(
            self, presenter,
        )?))
    }
}

impl DefaultPresentation for TextProductCollection {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_products_table(
            self, presenter,
        )?))
    }
}

impl DefaultPresentation for TextProductLocationCollection {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(
            create_products_locations_table(self, presenter),
        ))
    }
}

impl DefaultPresentation for TextProductTypeCollection {
    fn present_default(
        &self,
        _presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_product_types_table(
            self,
        )))
    }
}
