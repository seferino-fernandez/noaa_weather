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

# capture <relative path> <url> [feature flags] [accept]
# `accept` defaults to GeoJSON; JSON-LD endpoints pass application/ld+json.
capture() {
    local relative_path="$1"
    local url="$2"
    local feature_flags="${3:-}"
    local accept="${4:-application/geo+json}"
    local destination="$FIXTURE_DIR/$relative_path"
    local -a curl_args=(
        --fail
        --silent
        --show-error
        --location
        --header "User-Agent: $USER_AGENT"
        --header "Accept: $accept"
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

# capture_raw <relative path> <url> <accept>
# Writes the response bytes unchanged, for the XML endpoints.
capture_raw() {
    local relative_path="$1"
    local url="$2"
    local accept="$3"
    local destination="$FIXTURE_DIR/$relative_path"

    mkdir -p -- "$(dirname -- "$destination")"
    curl \
        --fail --silent --show-error --location \
        --header "User-Agent: $USER_AGENT" \
        --header "Accept: $accept" \
        --output "$destination" \
        "$url"
    printf 'wrote %s\n' "$relative_path"
}

# trim <relative path> <array key> <keep>
#
# Some responses are far too large to keep whole: the glossary is over three
# thousand terms and most of a megabyte, and the transmitter list is five
# hundred entries. Trimming is only safe under a rule, or the fixture stops
# describing what NOAA sends:
#
#   keep the envelope untouched, keep the first <keep> elements of the named
#   array, additionally keep any later element that introduces a key path the
#   kept ones do not already have, and drop nothing else.
#
# The second clause is what makes this different from a slice: an optional
# field that only the two-hundredth element populates survives, so a model
# that fails to round-trip it still fails.
#
# The count of dropped elements goes in a sibling `<fixture>.trim` note
# rather than into the JSON. A key NOAA never sent has no business inside a
# file whose whole purpose is to say what NOAA sends.
trim() {
    python3 - "$FIXTURE_DIR/$1" "$2" "$3" "$1" <<'PY'
import json
import sys

path, key, keep, relative = sys.argv[1], sys.argv[2], int(sys.argv[3]), sys.argv[4]

with open(path, encoding="utf-8") as fixture:
    document = json.load(fixture)

elements = document.get(key)
if not isinstance(elements, list):
    raise SystemExit(f"{relative}: {key!r} is not an array")


def key_paths(value, prefix=""):
    """Every key path in `value`, so two elements with the same optional
    fields present count as the same shape."""
    paths = set()
    if isinstance(value, dict):
        for name, child in value.items():
            here = f"{prefix}.{name}" if prefix else name
            paths.add(here)
            paths |= key_paths(child, here)
    elif isinstance(value, list):
        here = f"{prefix}[]"
        paths.add(here)
        for child in value:
            paths |= key_paths(child, here)
    return paths


kept, seen = [], set()
for index, element in enumerate(elements):
    shape = key_paths(element)
    if index < keep or not shape <= seen:
        kept.append(element)
        seen |= shape

document[key] = kept
with open(path, "w", encoding="utf-8") as fixture:
    json.dump(document, fixture, indent=2)
    fixture.write("\n")

dropped = len(elements) - len(kept)
note = (
    f"{relative} is trimmed. Rule: keep the envelope, keep the first {keep} "
    f"elements of {key!r}, keep any later element introducing a key path the "
    f"kept ones lack, drop the rest.\n"
    f"Captured {len(elements)} elements, kept {len(kept)}, dropped {dropped}.\n"
)
with open(f"{path}.trim", "w", encoding="utf-8") as record:
    record.write(note)
print(f"trimmed {relative}: kept {len(kept)} of {len(elements)}")
PY
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

# pick <fixture> <dotted path>
# Reads one value out of a captured document; `[]` steps into the first
# element of an array. Prints nothing and fails when the path is absent, so
# callers can branch on an endpoint that had nothing to return today.
pick() {
    python3 - "$1" "$2" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as fixture:
    value = json.load(fixture)
for step in sys.argv[2].split("."):
    if step == "[]":
        if not isinstance(value, list) or not value:
            raise SystemExit(1)
        value = value[0]
        continue
    if not isinstance(value, dict) or step not in value:
        raise SystemExit(1)
    value = value[step]
if value is None:
    raise SystemExit(1)
print(value)
PY
}

# last_segment <url> [count]
# The last path segment of a URL, or the last `count` segments joined.
last_segment() {
    python3 - "$1" "${2:-1}" <<'PY'
import sys
from urllib.parse import urlsplit

segments = [part for part in urlsplit(sys.argv[1]).path.split("/") if part]
count = int(sys.argv[2])
if len(segments) < count:
    raise SystemExit(1)
print("/".join(segments[-count:]))
PY
}

capture "alerts/list.json" "$BASE_URL/alerts?limit=5"
alert_id="$(first_property "$FIXTURE_DIR/alerts/list.json" id)"
encoded_alert_id="$(python3 -c 'import sys; from urllib.parse import quote; print(quote(sys.argv[1], safe=""))' "$alert_id")"
capture "alerts/single.json" "$BASE_URL/alerts/$encoded_alert_id"
capture "alerts/count.json" "$BASE_URL/alerts/active/count" "" "application/ld+json"
capture "alerts/types.json" "$BASE_URL/alerts/types" "" "application/ld+json"

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

readonly JSON_LD="application/ld+json"
readonly SSML="application/ssml+xml"
readonly IWXXM="application/vnd.wmo.iwxxm+xml"

capture "glossary/terms.json" "$BASE_URL/glossary" "" "$JSON_LD"
trim "glossary/terms.json" glossary 5

capture "offices/office.json" "$BASE_URL/offices/PSR" "" "$JSON_LD"
capture "offices/headlines.json" "$BASE_URL/offices/PSR/headlines" "" "$JSON_LD"
if headline="$(pick "$FIXTURE_DIR/offices/headlines.json" '@graph.[].@id')"; then
    capture "offices/headline.json" "$headline" "" "$JSON_LD"
else
    rm -f -- "$FIXTURE_DIR/offices/headline.json"
    printf 'skipping offices/headline.json: PSR has no headlines today\n'
fi
capture "offices/briefing.json" "$BASE_URL/offices/PSR/briefing" "" "$JSON_LD"
capture "offices/weather_stories.json" "$BASE_URL/offices/PSR/weatherstories" "" "$JSON_LD"

capture "aviation/cwsu.json" "$BASE_URL/aviation/cwsus/ZAB" "" "$JSON_LD"

capture "products/list.json" "$BASE_URL/products?limit=5" "" "$JSON_LD"
if product="$(pick "$FIXTURE_DIR/products/list.json" '@graph.[].id')"; then
    capture "products/product.json" "$BASE_URL/products/$product" "" "$JSON_LD"
else
    rm -f -- "$FIXTURE_DIR/products/product.json"
    printf 'skipping products/product.json: the product listing is empty\n'
fi
capture "products/locations.json" "$BASE_URL/products/locations" "" "$JSON_LD"
capture "products/types.json" "$BASE_URL/products/types" "" "$JSON_LD"
capture "products/type.json" "$BASE_URL/products/types/AFD" "" "$JSON_LD"
trim "products/type.json" '@graph' 5
capture "products/type_locations.json" "$BASE_URL/products/types/AFD/locations" "" "$JSON_LD"
capture "products/type_location.json" "$BASE_URL/products/types/AFD/locations/LWX" "" "$JSON_LD"
capture "products/location_types.json" "$BASE_URL/products/locations/PSR/types" "" "$JSON_LD"
capture "products/latest.json" "$BASE_URL/products/types/AFD/locations/PSR/latest" "" "$JSON_LD"

capture "radar/servers.json" "$BASE_URL/radar/servers" "" "$JSON_LD"
capture "radar/stations.json" "$BASE_URL/radar/stations"
trim "radar/stations.json" features 3
capture "radar/queue.json" "$BASE_URL/radar/queues/rds?limit=5" "" "$JSON_LD"
capture "radar/alarms.json" "$BASE_URL/radar/stations/KABQ/alarms" "" "$JSON_LD"
capture "radar/spgds.json" "$BASE_URL/radar/spgds" "" "$JSON_LD"
trim "radar/spgds.json" '@graph' 2

capture "radio/transmitters.json" "$BASE_URL/radio" "" "$JSON_LD"
trim "radio/transmitters.json" '@graph' 5
capture "radio/transmitter.json" "$BASE_URL/radio/KEC94" "" "$JSON_LD"
capture "radio/county.json" "$BASE_URL/zones/county/AZC013/radio" "" "$JSON_LD"
trim "radio/county.json" '@graph' 5
capture_raw "radio/broadcast.xml" "$BASE_URL/radio/KEC94/broadcast" "$SSML"
capture_raw "radio/point.xml" "$BASE_URL/points/33.4484,-112.0740/radio" "$SSML"

capture "stations/tafs.json" "$BASE_URL/stations/KPHX/tafs" "" "$JSON_LD"
trim "stations/tafs.json" '@graph' 3
if taf="$(pick "$FIXTURE_DIR/stations/tafs.json" '@graph.[].id')" &&
    issued="$(last_segment "$taf" 2)"; then
    capture_raw "stations/taf.xml" "$BASE_URL/stations/KPHX/tafs/$issued" "$IWXXM"
else
    rm -f -- "$FIXTURE_DIR/stations/taf.xml"
    printf 'skipping stations/taf.xml: KPHX has no current TAF\n'
fi
