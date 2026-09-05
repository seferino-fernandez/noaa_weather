# Products

Product type codes (`AFD`, `HWO`, ...) and issuance locations (`LWX`, `PSR`, ...) are validated before any request is made; a malformed value is a usage error (exit code 2). Location `--help` lists the known forecast offices as a hint without restricting the value.

## Get text products by specific location

```bash
noaa-weather products products-by-location --location-id <LOCATION_ID>
```

## Get text product by specific ID

```bash
noaa-weather products metadata --id <PRODUCT_ID>
```

## Get all product types and codes

```bash
noaa-weather products types
```

## Get text products by specific product type

```bash
noaa-weather products type --type-id <TYPE_ID>
```

## Get text products by specific location and product type

```bash
noaa-weather products types-by-location --type-id <TYPE_ID> --location-id <LOCATION_ID>
```

## Query text products

`--start-time` and `--end-time` accept an RFC 3339 timestamp or a relative age such as `2d`.

```bash
noaa-weather products list [--location-ids <ID,...>] [--office-ids <ID,...>] [--wmo-ids <ID,...>] [--product-type-codes <TYPE,...>] [--start-time <TIME>] [--end-time <TIME>] [--limit <1-500>]
```

## Get all locations by product type

```bash
noaa-weather products locations-by-type --type-id <TYPE_ID>
```

## Get all product issuance locations

```bash
noaa-weather products locations
```

## Get the latest product by type and location

```bash
noaa-weather products latest --type-id <TYPE_ID> --location-id <LOCATION_ID>
```

<!-- BEGIN GENERATED SHOWN/OMITTED -->

## Human-summary property coverage

The table is generated from the summary contracts. `Shown` properties appear in keyed human-summary content; `Otherwise accounted for` properties are deliberately handled without a keyed table or fact.

| Response | Property | Treatment | Reason |
| :--- | :--- | :--- | :--- |
| Product location list | `locations` | Shown | — |
| Product type list | `@graph` | Shown | — |
| Product type list | `productCode` | Shown | — |
| Product type list | `productName` | Shown | — |
| Text product | `@id` | Otherwise accounted for | the server-issued product identifier is shown |
| Text product | `id` | Shown | — |
| Text product | `issuanceTime` | Shown | — |
| Text product | `issuingOffice` | Shown | — |
| Text product | `productCode` | Shown | — |
| Text product | `productName` | Shown | — |
| Text product | `productText` | Shown | — |
| Text product | `wmoCollectiveId` | Shown | — |
| Text product list | `@graph` | Otherwise accounted for | each product is one table row |
| Text product list | `@id` | Otherwise accounted for | the server-issued product identifier is shown |
| Text product list | `id` | Shown | — |
| Text product list | `issuanceTime` | Shown | — |
| Text product list | `issuingOffice` | Shown | — |
| Text product list | `productCode` | Shown | — |
| Text product list | `productName` | Otherwise accounted for | the product code is compact; names are available from the product-types command |
| Text product list | `productText` | Otherwise accounted for | catalog endpoints omit the full product text |
| Text product list | `wmoCollectiveId` | Otherwise accounted for | the product code, office, and issuance time identify catalog rows |

<!-- END GENERATED SHOWN/OMITTED -->
