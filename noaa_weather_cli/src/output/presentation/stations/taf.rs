use comfy_table::Attribute;
use jiff::Timestamp;
use noaa_weather_client::models::terminal_aerodrome_forecast::{
    CloudAmount, CloudLayer, CloudType, Comparison, ForecastClouds, ForecastConditions,
    ForecastGroup, ForecastGroupKind, ForecastReport, ForecastValue, ForecastVisibility,
    ForecastWeather, ForecastWind, MissingForecastReason, MissingReason, PermissibleUsage,
    PermissibleUsageReason, ReportStatus, SurfaceWind, TemperatureForecast, TimeRange, Weather,
    WeatherDescriptor, WeatherIntensity, WeatherPhenomenon, WindDirection, WindSpeed,
};
use noaa_weather_client::models::{TerminalAerodromeForecast, TerminalAerodromeForecastsResponse};

use super::*;

/// Creates a table listing TAF metadata for one airport station.
pub fn create_stations_tafs_metadata_table(tafs: &TerminalAerodromeForecastsResponse) -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header([
        header("ID"),
        header("Issue Time"),
        header("Location"),
        header("Start"),
        header("End"),
        header("Geometry"),
    ]);

    for taf in tafs.graph.as_deref().unwrap_or_default() {
        table.add_row([
            Cell::new(&taf.id),
            Cell::new(format_datetime_human_readable(taf.issue_time.as_deref())),
            Cell::new(taf.location.as_deref().unwrap_or("Unavailable")),
            Cell::new(format_datetime_human_readable(taf.start.as_deref())),
            Cell::new(format_datetime_human_readable(taf.end.as_deref())),
            Cell::new(taf.geometry.as_deref().unwrap_or("Unavailable")),
        ]);
    }

    table
}

fn header(value: &str) -> Cell {
    Cell::new(value)
        .add_attribute(Attribute::Bold)
        .set_alignment(CellAlignment::Center)
}

/// Creates a human-readable table from normalized TAF meaning.
pub fn create_stations_taf_table(taf: &TerminalAerodromeForecast) -> Table {
    let mut table = Table::new();
    table
        .load_style(UTF8_FULL_CONDENSED)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header([header_left("Category"), header_left("Details")]);

    add_detail(&mut table, "Airport", taf.aerodrome().icao_identifier());
    if taf.aerodrome().designator() != taf.aerodrome().icao_identifier() {
        add_detail(
            &mut table,
            "Aerodrome designator",
            taf.aerodrome().designator(),
        );
    }
    add_detail(&mut table, "Issued", format_timestamp(taf.issued_at()));
    add_detail(
        &mut table,
        "Status",
        format_status(taf.report_metadata().status()),
    );
    add_detail(
        &mut table,
        "Permissible use",
        format_usage(taf.report_metadata().permissible_usage()),
    );
    add_detail(&mut table, "Bulletin", taf.bulletin_identifier());

    if let Some(translation) = taf.report_metadata().translation() {
        let mut details = Vec::new();
        if let Some(identifier) = translation.source_bulletin_identifier() {
            details.push(format!("source {identifier}"));
        }
        if let Some(received_at) = translation.source_bulletin_received_at() {
            details.push(format!("received {}", format_timestamp(received_at)));
        }
        let centre = match (translation.centre_designator(), translation.centre_name()) {
            (Some(designator), Some(name)) => Some(format!("{designator} ({name})")),
            (Some(designator), None) => Some(designator.to_owned()),
            (None, Some(name)) => Some(name.to_owned()),
            (None, None) => None,
        };
        if let Some(centre) = centre {
            details.push(format!("centre {centre}"));
        }
        if let Some(translated_at) = translation.translated_at() {
            details.push(format!("translated {}", format_timestamp(translated_at)));
        }
        add_detail(&mut table, "Translation", details.join("; "));
    }

    match taf.report() {
        ForecastReport::Forecast {
            valid_period,
            groups,
            ..
        } => {
            add_detail(&mut table, "Report state", "Forecast");
            add_detail(&mut table, "Valid period", format_time_range(*valid_period));
            for group in groups {
                add_forecast_group(&mut table, group);
            }
        }
        ForecastReport::Cancellation {
            cancelled_period, ..
        } => {
            add_detail(&mut table, "Report state", "Cancelled");
            add_detail(
                &mut table,
                "Cancelled period",
                format_time_range(*cancelled_period),
            );
        }
        ForecastReport::Missing { reason, .. } => {
            add_detail(&mut table, "Report state", "Forecast unavailable");
            add_detail(&mut table, "Reason", format_missing_forecast(reason));
        }
        _ => add_detail(&mut table, "Report state", "Unknown report state"),
    }

    table
}

fn header_left(value: &str) -> Cell {
    Cell::new(value)
        .add_attribute(Attribute::Bold)
        .set_alignment(CellAlignment::Left)
}

fn add_detail(table: &mut Table, category: &str, details: impl Into<String>) {
    table.add_row([
        Cell::new(category).add_attribute(Attribute::Bold),
        Cell::new(details.into()),
    ]);
}

fn add_forecast_group(table: &mut Table, group: &ForecastGroup) {
    table.add_row([Cell::new("---"), Cell::new("---")]);
    table.add_row([
        Cell::new(format_group_kind(group.kind()))
            .add_attribute(Attribute::Bold)
            .add_attribute(Attribute::Underlined),
        Cell::new(format_time_range(group.valid_period())),
    ]);

    let conditions = group.conditions();
    let is_base = matches!(group.kind(), ForecastGroupKind::Base);
    add_detail(
        table,
        "CAVOK",
        if conditions.is_cavok() {
            "Yes — visibility at least 10 km; no significant weather or cloud"
        } else {
            "No"
        },
    );
    add_detail(table, "Wind", format_wind(conditions.wind(), is_base));
    add_detail(table, "Visibility", format_visibility(conditions, is_base));
    add_detail(
        table,
        "Weather",
        format_weather(conditions.weather(), conditions.is_cavok(), is_base),
    );
    add_detail(
        table,
        "Clouds",
        format_clouds(conditions.clouds(), conditions.is_cavok(), is_base),
    );
    if !conditions.temperatures().is_empty() {
        add_detail(
            table,
            "Temperatures",
            format_temperatures(conditions.temperatures()),
        );
    }
}

fn format_timestamp(timestamp: Timestamp) -> String {
    let day = timestamp.strftime("%d").to_string();
    format!(
        "{} {} UTC",
        day.trim_start_matches('0'),
        timestamp.strftime("%b %H:%M")
    )
}

fn format_time_range(period: TimeRange) -> String {
    format!(
        "{} to {}",
        format_timestamp(period.start()),
        format_timestamp(period.end())
    )
}

fn format_status(status: &ReportStatus) -> String {
    match status {
        ReportStatus::Normal => "Normal".to_owned(),
        ReportStatus::Amendment => "Amendment".to_owned(),
        ReportStatus::Correction => "Correction".to_owned(),
        ReportStatus::Other { code } => format!("Other ({code})"),
        _ => "Unknown".to_owned(),
    }
}

fn format_usage(usage: &PermissibleUsage) -> String {
    match usage {
        PermissibleUsage::Operational => "Operational".to_owned(),
        PermissibleUsage::NonOperational {
            reason,
            supplementary,
        } => {
            let reason = reason.as_ref().map(format_usage_reason);
            match (reason, supplementary.as_deref()) {
                (Some(reason), Some(details)) => {
                    format!("Non-operational — {reason}: {details}")
                }
                (Some(reason), None) => format!("Non-operational — {reason}"),
                (None, Some(details)) => format!("Non-operational — {details}"),
                (None, None) => "Non-operational".to_owned(),
            }
        }
        PermissibleUsage::Other {
            code,
            reason,
            supplementary,
        } => {
            let reason = reason.as_ref().map(format_usage_reason);
            match (reason, supplementary.as_deref()) {
                (Some(reason), Some(details)) => format!("Other ({code}) — {reason}: {details}"),
                (Some(reason), None) => format!("Other ({code}) — {reason}"),
                (None, Some(details)) => format!("Other ({code}) — {details}"),
                (None, None) => format!("Other ({code})"),
            }
        }
        _ => "Unknown".to_owned(),
    }
}

fn format_usage_reason(reason: &PermissibleUsageReason) -> String {
    match reason {
        PermissibleUsageReason::Test => "test".to_owned(),
        PermissibleUsageReason::Exercise => "exercise".to_owned(),
        PermissibleUsageReason::Other { code } => code.to_string(),
        _ => "unknown reason".to_owned(),
    }
}

fn format_missing_forecast(reason: &MissingForecastReason) -> String {
    match reason {
        MissingForecastReason::NotProvided => "Forecast content was not provided".to_owned(),
        MissingForecastReason::TranslationFailed { tac } => {
            format!("TAC-to-IWXXM translation failed\nSource TAC: {tac}")
        }
        _ => "Unknown reason".to_owned(),
    }
}

fn format_group_kind(kind: &ForecastGroupKind) -> String {
    match kind {
        ForecastGroupKind::Base => "INITIAL FORECAST".to_owned(),
        ForecastGroupKind::From => "CHANGE — FROM (FM)".to_owned(),
        ForecastGroupKind::Becoming => "CHANGE — BECOMING (BECMG)".to_owned(),
        ForecastGroupKind::Temporary => "CHANGE — TEMPORARY (TEMPO)".to_owned(),
        ForecastGroupKind::Probability { percent, temporary } => {
            let temporary = if *temporary { " — TEMPORARY" } else { "" };
            format!("CHANGE — PROBABILITY {percent}%{temporary}")
        }
        ForecastGroupKind::Other { code } => format!("CHANGE — {code}"),
        _ => "CHANGE — UNKNOWN".to_owned(),
    }
}

fn format_wind(wind: &ForecastWind, is_base: bool) -> String {
    match wind {
        ForecastWind::NotReported => unchanged_or_not_reported(is_base),
        ForecastWind::Value(wind) => format_surface_wind(wind),
        ForecastWind::Unavailable { reason } => format!("Unavailable ({})", format_nil(reason)),
        _ => "Unknown wind state".to_owned(),
    }
}

fn format_surface_wind(wind: &SurfaceWind) -> String {
    let direction = match wind.direction() {
        WindDirection::Variable => "Variable (VRB)".to_owned(),
        WindDirection::Degrees(degrees) => format!("{}°", format_number(degrees)),
        _ => "Unknown direction".to_owned(),
    };
    let mut value = format!("{direction} at {}", format_wind_speed(wind.speed()));
    if let Some(gust) = wind.gust() {
        value.push_str("; gusting ");
        value.push_str(&format_wind_speed(gust));
    }
    value
}

fn format_wind_speed(speed: &WindSpeed) -> String {
    format!(
        "{}{} kt",
        format_comparison(speed.comparison()),
        format_number(speed.knots())
    )
}

fn format_visibility(conditions: &ForecastConditions, is_base: bool) -> String {
    if conditions.is_cavok() {
        return "At least 10 km (CAVOK)".to_owned();
    }
    let visibility = match conditions.visibility() {
        ForecastVisibility::NotReported => return unchanged_or_not_reported(is_base),
        ForecastVisibility::Value(visibility) => visibility,
        ForecastVisibility::Unavailable { reason } => {
            return format!("Unavailable ({})", format_nil(reason));
        }
        _ => return "Unknown visibility state".to_owned(),
    };
    let meters = visibility.meters();
    let distance = if meters >= 1_000.0 {
        format!("{} km", format_number(meters / 1_000.0))
    } else {
        format!("{} m", format_number(meters))
    };
    format!(
        "{}{} ({:.1} mi)",
        format_comparison(visibility.comparison()),
        distance,
        meters / 1_609.344
    )
}

fn format_comparison(comparison: &Comparison) -> String {
    match comparison {
        Comparison::Exact => String::new(),
        Comparison::Above => "≥".to_owned(),
        Comparison::Below => "≤".to_owned(),
        Comparison::Other { code } => format!("{code} "),
        _ => "? ".to_owned(),
    }
}

fn format_weather(weather: &ForecastWeather, cavok: bool, is_base: bool) -> String {
    if cavok {
        return "No significant weather (CAVOK)".to_owned();
    }
    match weather {
        ForecastWeather::NotReported => unchanged_or_not_reported(is_base),
        ForecastWeather::NoSignificant => "No significant weather".to_owned(),
        ForecastWeather::Phenomena { items } => items
            .iter()
            .map(format_weather_item)
            .collect::<Vec<_>>()
            .join("\n"),
        ForecastWeather::Unavailable { reason } => {
            format!("Unavailable ({})", format_nil(reason))
        }
        _ => "Unknown weather state".to_owned(),
    }
}

fn format_weather_item(weather: &Weather) -> String {
    let descriptor = weather.descriptor().map(format_descriptor);
    let phenomena = weather
        .phenomena()
        .iter()
        .map(format_phenomenon)
        .collect::<Vec<_>>()
        .join(" and ");
    let mut description = match (descriptor, phenomena.is_empty()) {
        (Some(descriptor), false) => format!("{descriptor} with {phenomena}"),
        (Some(descriptor), true) => descriptor,
        (None, false) => phenomena,
        (None, true) => "unclassified weather".to_owned(),
    };
    match weather.intensity() {
        WeatherIntensity::Light => description.insert_str(0, "light "),
        WeatherIntensity::Heavy => description.insert_str(0, "heavy "),
        WeatherIntensity::Moderate => {}
        _ => description.insert_str(0, "unknown-intensity "),
    }
    if weather.is_in_vicinity() {
        description.push_str(" in the vicinity");
    }
    format!("{} — {description}", weather.code())
}

fn format_descriptor(descriptor: &WeatherDescriptor) -> String {
    match descriptor {
        WeatherDescriptor::Shallow => "shallow".to_owned(),
        WeatherDescriptor::Partial => "partial".to_owned(),
        WeatherDescriptor::Patches => "patches".to_owned(),
        WeatherDescriptor::LowDrifting => "low drifting".to_owned(),
        WeatherDescriptor::Blowing => "blowing".to_owned(),
        WeatherDescriptor::Showers => "showers".to_owned(),
        WeatherDescriptor::Thunderstorm => "thunderstorm".to_owned(),
        WeatherDescriptor::Freezing => "freezing".to_owned(),
        WeatherDescriptor::Other { code } => format!("descriptor {code}"),
        _ => "unknown descriptor".to_owned(),
    }
}

fn format_phenomenon(phenomenon: &WeatherPhenomenon) -> String {
    match phenomenon {
        WeatherPhenomenon::Drizzle => "drizzle".to_owned(),
        WeatherPhenomenon::Rain => "rain".to_owned(),
        WeatherPhenomenon::Snow => "snow".to_owned(),
        WeatherPhenomenon::SnowGrains => "snow grains".to_owned(),
        WeatherPhenomenon::IceCrystals => "ice crystals".to_owned(),
        WeatherPhenomenon::IcePellets => "ice pellets".to_owned(),
        WeatherPhenomenon::Hail => "hail".to_owned(),
        WeatherPhenomenon::SmallHail => "small hail or snow pellets".to_owned(),
        WeatherPhenomenon::UnknownPrecipitation => "unknown precipitation".to_owned(),
        WeatherPhenomenon::Mist => "mist".to_owned(),
        WeatherPhenomenon::Fog => "fog".to_owned(),
        WeatherPhenomenon::Smoke => "smoke".to_owned(),
        WeatherPhenomenon::VolcanicAsh => "volcanic ash".to_owned(),
        WeatherPhenomenon::Dust => "widespread dust".to_owned(),
        WeatherPhenomenon::Sand => "sand".to_owned(),
        WeatherPhenomenon::Haze => "haze".to_owned(),
        WeatherPhenomenon::Spray => "spray".to_owned(),
        WeatherPhenomenon::DustWhirls => "dust or sand whirls".to_owned(),
        WeatherPhenomenon::Squalls => "squalls".to_owned(),
        WeatherPhenomenon::FunnelCloud => "funnel cloud or tornado/waterspout".to_owned(),
        WeatherPhenomenon::Sandstorm => "sandstorm".to_owned(),
        WeatherPhenomenon::Duststorm => "duststorm".to_owned(),
        WeatherPhenomenon::Other { code } => format!("unrecognized phenomenon {code}"),
        _ => "unknown phenomenon".to_owned(),
    }
}

fn format_clouds(clouds: &ForecastClouds, cavok: bool, is_base: bool) -> String {
    if cavok {
        return "No operationally significant cloud (CAVOK)".to_owned();
    }
    match clouds {
        ForecastClouds::NotReported => unchanged_or_not_reported(is_base),
        ForecastClouds::NoSignificant => "No significant cloud".to_owned(),
        ForecastClouds::VerticalVisibility { feet } => match feet {
            ForecastValue::Value(feet) => {
                format!("Vertical visibility {} ft", format_number(*feet))
            }
            ForecastValue::Unavailable { reason } => {
                format!("Vertical visibility unavailable ({})", format_nil(reason))
            }
            _ => "Unknown vertical visibility".to_owned(),
        },
        ForecastClouds::Layers { layers } => layers
            .iter()
            .map(format_cloud_layer)
            .collect::<Vec<_>>()
            .join("\n"),
        ForecastClouds::Unavailable { reason } => {
            format!("Unavailable ({})", format_nil(reason))
        }
        _ => "Unknown cloud state".to_owned(),
    }
}

fn format_cloud_layer(layer: &CloudLayer) -> String {
    let amount = match layer.amount() {
        ForecastValue::Value(amount) => format_cloud_amount(amount),
        ForecastValue::Unavailable { reason } => {
            format!("Amount unavailable ({})", format_nil(reason))
        }
        _ => "Unknown amount".to_owned(),
    };
    let base = match layer.base_feet() {
        ForecastValue::Value(feet) => format!("at {} ft AGL", format_number(*feet)),
        ForecastValue::Unavailable { reason } => {
            format!("at unavailable base ({})", format_nil(reason))
        }
        _ => "at unknown base".to_owned(),
    };
    let cloud_type = layer
        .cloud_type()
        .map_or_else(String::new, |cloud_type| match cloud_type {
            ForecastValue::Value(cloud_type) => format!(" — {}", format_cloud_type(cloud_type)),
            ForecastValue::Unavailable { reason } => {
                format!(" — type unavailable ({})", format_nil(reason))
            }
            _ => " — unknown type".to_owned(),
        });
    format!("{amount} {base}{cloud_type}")
}

fn format_cloud_amount(amount: &CloudAmount) -> String {
    match amount {
        CloudAmount::Few => "Few (FEW)".to_owned(),
        CloudAmount::Scattered => "Scattered (SCT)".to_owned(),
        CloudAmount::Broken => "Broken (BKN)".to_owned(),
        CloudAmount::Overcast => "Overcast (OVC)".to_owned(),
        CloudAmount::NoSignificant => "No significant cloud (NSC)".to_owned(),
        CloudAmount::SkyClear => "Sky clear (SKC/CLR)".to_owned(),
        CloudAmount::Other { code } => format!("Other ({code})"),
        _ => "Unknown amount".to_owned(),
    }
}

fn format_cloud_type(cloud_type: &CloudType) -> String {
    match cloud_type {
        CloudType::Cumulonimbus => "Cumulonimbus (CB)".to_owned(),
        CloudType::ToweringCumulus => "Towering cumulus (TCU)".to_owned(),
        CloudType::Other { code } => format!("Other ({code})"),
        _ => "Unknown type".to_owned(),
    }
}

fn format_temperatures(temperatures: &[TemperatureForecast]) -> String {
    temperatures
        .iter()
        .map(|temperature| {
            format!(
                "Maximum {} °C at {}; minimum {} °C at {}",
                format_number(temperature.maximum().celsius()),
                format_timestamp(temperature.maximum().occurs_at()),
                format_number(temperature.minimum().celsius()),
                format_timestamp(temperature.minimum().occurs_at()),
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn unchanged_or_not_reported(is_base: bool) -> String {
    if is_base {
        "Not reported".to_owned()
    } else {
        "Unchanged from prevailing conditions".to_owned()
    }
}

fn format_nil(reason: &MissingReason) -> String {
    match reason {
        MissingReason::NoSignificant => "nothing operationally significant".to_owned(),
        MissingReason::NotObservable => "not observable".to_owned(),
        MissingReason::Missing => "missing".to_owned(),
        MissingReason::Withheld => "withheld".to_owned(),
        MissingReason::Other { code } => format!("{code}"),
        _ => "unknown reason".to_owned(),
    }
}

fn format_number(value: f64) -> String {
    if value.fract().abs() < f64::EPSILON {
        format!("{value:.0}")
    } else {
        format!("{value:.1}")
    }
}
