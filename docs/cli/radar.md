# Radar

Get radar profiler data for a station.

```sh
noaa-weather radar profiler <station_id>
```

Get radar queue data for a host.

```sh
noaa-weather radar queue <host>
```

Get radar server data for an ID.

```sh
noaa-weather radar server <id>
```

Get all radar servers.

```sh
noaa-weather radar servers
```

Get radar station data for a station ID.

```sh
noaa-weather radar station <station_id>
```

Get radar station alarms for a station ID.

```sh
noaa-weather radar station-alarms <station_id>
```

Get all radar stations.

```sh
noaa-weather radar stations
```

Get all radar stations with a specific station type.

```sh
noaa-weather radar stations --stationType <type>
```

## Radar Profiler

Currently unable to make a valid request to the radar profiler API.

## Radar Queues

Valid Radar Queue Hosts:
`tds` is a THREDDs Data Server
`rds` is a Remote Data Server that's used to ingest data from private/civilian weather stations

When retrieving radar queue data, there is too much data to return so the API returns a 503 error.
The workaround is to specify more filters to reduce the data amount returned.
See [Radar Host issue here](https://github.com/weather-gov/api/discussions/756)
