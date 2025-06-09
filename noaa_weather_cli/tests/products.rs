use assert_cmd::Command;

#[test]
fn test_products_list_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("products");
    cmd.arg("list");
    cmd.assert().success();
}

#[test]
fn test_products_list_with_location_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("products");
    cmd.arg("list");
    cmd.arg("--location-ids");
    cmd.arg("PSR");
    cmd.assert().success();
}

#[test]
fn test_products_types_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("products");
    cmd.arg("types");
    cmd.assert().success();
}

#[test]
fn test_products_type_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("products");
    cmd.arg("type");
    cmd.arg("--type-id");
    cmd.arg("AFD");
    cmd.assert().success();
}

#[test]
fn test_products_types_by_location_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("products");
    cmd.arg("types-by-location");
    cmd.arg("--type-id");
    cmd.arg("AFD");
    cmd.arg("--location-id");
    cmd.arg("LWX");
    cmd.assert().success();
}

#[test]
fn test_products_locations_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("products");
    cmd.arg("locations");
    cmd.assert().success();
}

#[test]
fn test_product_locations_by_type_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("products");
    cmd.arg("locations-by-type");
    cmd.arg("--type-id");
    cmd.arg("AFD");
    cmd.assert().success();
}

#[test]
fn test_products_by_location_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("products");
    cmd.arg("products-by-location");
    cmd.arg("--location-id");
    cmd.arg("PSR");
    cmd.assert().success();
}

#[ignore = "Update to dynamically get a product id"]
#[test]
fn test_product_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("products");
    cmd.arg("metadata");
    cmd.arg("--id");
    cmd.arg("a4791428-298e-473c-8e6f-5796701c9e4a");
    cmd.assert().success();
}
