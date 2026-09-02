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
