#!/usr/bin/env bash
set -euo pipefail

readonly BASE_URL="https://api.weather.gov"
readonly USER_AGENT="noaa-weather-fixtures (+https://github.com/seferino-fernandez/noaa_weather)"
readonly FIXTURE_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
readonly TEMP_RESPONSE="$(mktemp)"

trap 'rm -f -- "$TEMP_RESPONSE"' EXIT

pretty_print() {
    if command -v jq >/dev/null 2>&1; then
        jq . "$TEMP_RESPONSE"
    else
        python3 -m json.tool "$TEMP_RESPONSE"
    fi
}

capture() {
    local relative_path="$1"
    local url="$2"
    local feature_flags="${3:-}"
    local destination="$FIXTURE_DIR/$relative_path"
    local -a curl_args=(
        --fail
        --silent
        --show-error
        --location
        --header "User-Agent: $USER_AGENT"
        --header "Accept: application/geo+json"
        --output "$TEMP_RESPONSE"
    )

    if [[ -n "$feature_flags" ]]; then
        curl_args+=(--header "Feature-Flags: $feature_flags")
    fi

    mkdir -p -- "$(dirname -- "$destination")"
    curl "${curl_args[@]}" "$url"
    pretty_print >"$destination"
    printf 'wrote %s\n' "$relative_path"
}

first_property() {
    python3 - "$1" "$2" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as fixture:
    features = json.load(fixture).get("features", [])
if not features:
    raise SystemExit(1)
value = features[0].get("properties", {}).get(sys.argv[2])
if value is None:
    raise SystemExit(1)
print(value)
PY
}

capture "alerts/list.json" "$BASE_URL/alerts?limit=5"
alert_id="$(first_property "$FIXTURE_DIR/alerts/list.json" id)"
encoded_alert_id="$(python3 -c 'import sys; from urllib.parse import quote; print(quote(sys.argv[1], safe=""))' "$alert_id")"
capture "alerts/single.json" "$BASE_URL/alerts/$encoded_alert_id"

capture "stations/list.json" "$BASE_URL/stations?limit=5"
capture "stations/single.json" "$BASE_URL/stations/KSLC"
capture "stations/observations.json" "$BASE_URL/stations/KSLC/observations?limit=5"
capture "stations/latest.json" "$BASE_URL/stations/KSLC/observations/latest"

capture "points/point.json" "$BASE_URL/points/39.7456,-97.0892"

capture "gridpoints/gridpoint.json" "$BASE_URL/gridpoints/TOP/31,80"
capture \
    "gridpoints/forecast.json" \
    "$BASE_URL/gridpoints/TOP/31,80/forecast" \
    "forecast_temperature_qv,forecast_wind_speed_qv"
capture \
    "gridpoints/hourly.json" \
    "$BASE_URL/gridpoints/TOP/31,80/forecast/hourly" \
    "forecast_temperature_qv,forecast_wind_speed_qv"
capture "gridpoints/stations.json" "$BASE_URL/gridpoints/TOP/31,80/stations?limit=5"

capture "zones/list.json" "$BASE_URL/zones?limit=5"
capture "zones/single.json" "$BASE_URL/zones/forecast/UTZ101"
capture "zones/forecast.json" "$BASE_URL/zones/forecast/UTZ101/forecast"
capture "zones/observations.json" "$BASE_URL/zones/forecast/UTZ101/observations?limit=5"
capture "zones/stations.json" "$BASE_URL/zones/forecast/UTZ101/stations?limit=5"

capture "aviation/sigmets.json" "$BASE_URL/aviation/sigmets"
if sigmet_url="$(first_property "$FIXTURE_DIR/aviation/sigmets.json" id)"; then
    case "$sigmet_url" in
        "$BASE_URL"/aviation/sigmets/*)
            capture "aviation/sigmet.json" "$sigmet_url"
            ;;
        *)
            printf 'first SIGMET id is not an api.weather.gov SIGMET URL: %s\n' "$sigmet_url" >&2
            exit 1
            ;;
    esac
else
    rm -f -- "$FIXTURE_DIR/aviation/sigmet.json"
    printf 'skipping aviation/sigmet.json: the SIGMET list is empty\n'
fi

capture "aviation/cwas.json" "$BASE_URL/aviation/cwsus/ZAB/cwas"
if cwsu="$(first_property "$FIXTURE_DIR/aviation/cwas.json" cwsu)" && \
    issue_time="$(first_property "$FIXTURE_DIR/aviation/cwas.json" issueTime)" && \
    sequence="$(first_property "$FIXTURE_DIR/aviation/cwas.json" sequence)"; then
    issue_date="${issue_time%%T*}"
    capture "aviation/cwa.json" "$BASE_URL/aviation/cwsus/$cwsu/cwas/$issue_date/$sequence"
else
    rm -f -- "$FIXTURE_DIR/aviation/cwa.json"
    printf 'skipping aviation/cwa.json: the CWA list is empty\n'
fi
