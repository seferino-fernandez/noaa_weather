# Products

## Get text products by specific location

```bash
noaa-weather products products-by-location --location-id <LOCATION_ID>
```

## Get text product by specific ID

```bash
noaa-weather products product --product-id <PRODUCT_ID>
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

## Get all text products

```bash
noaa-weather products list
```

## Get all locations by product type

```bash
noaa-weather products locations-by-type --type-id <TYPE_ID>
```

## Get all product issuance locations

```bash
noaa-weather products locations
```
