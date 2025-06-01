//! WMO Unit Codes, derived from WMO Codelist.
//! See: http://codes.wmo.int/def/common/unit (or the source CSV for more precise references)

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WmoUnitCode {
    /// Original label: "minute (angle)", Code: "111", Notation: ""
    #[serde(rename = "wmoUnit:\'")]
    MinuteAngle,
    /// Original label: "second (angle)", Code: "112", Notation: ""
    #[serde(rename = "wmoUnit:\"")]
    SecondAngle,
    /// Original label: "parts per thousand", Code: "301", Notation: "0.001"
    #[serde(rename = "wmoUnit:0.001")]
    PartsPerThousand,
    /// Original label: "Dimensionless", Code: "000", Notation: "1"
    #[serde(rename = "wmoUnit:1")]
    Dimensionless,
    /// Original label: "ampere", Code: "004", Notation: "A"
    #[serde(rename = "wmoUnit:A")]
    Ampere,
    /// Original label: "astronomic unit", Code: "170", Notation: "AU"
    #[serde(rename = "wmoUnit:AU")]
    AstronomicUnit,
    /// Original label: "becquerel", Code: "080", Notation: "Bq"
    #[serde(rename = "wmoUnit:Bq")]
    Becquerel,
    /// Original label: "becquerels per litre", Code: "750", Notation: "Bq_l-1"
    #[serde(rename = "wmoUnit:Bq_l-1")]
    BecquerelsPerLitre,
    /// Original label: "becquerels per square metre", Code: "751", Notation: "Bq_m-2"
    #[serde(rename = "wmoUnit:Bq_m-2")]
    BecquerelsPerSquareMetre,
    /// Original label: "becquerels per cubic metre", Code: "752", Notation: "Bq_m-3"
    #[serde(rename = "wmoUnit:Bq_m-3")]
    BecquerelsPerCubicMetre,
    /// Original label: "becquerel seconds per cubic metre", Code: "830", Notation: "Bq_s_m-3"
    #[serde(rename = "wmoUnit:Bq_s_m-3")]
    BecquerelSecondsPerCubicMetre,
    /// Original label: "coulomb", Code: "035", Notation: "C"
    #[serde(rename = "wmoUnit:C")]
    Coulomb,
    /// Original label: "degrees Celsius per 100 metres", Code: "352", Notation: "C_-1"
    #[serde(rename = "wmoUnit:C_-1")]
    DegreesCelsiusPer100Metres,
    /// Original label: "degrees Celsius per metre", Code: "351", Notation: "C_m-1"
    #[serde(rename = "wmoUnit:C_m-1")]
    DegreesCelsiusPerMetre,
    /// Original label: "degree Celsius", Code: "060", Notation: "Cel"
    #[serde(rename = "wmoUnit:Cel")]
    DegreeCelsius,
    /// Original label: "Dobson Unit (9)", Code: "360", Notation: "DU"
    #[serde(rename = "wmoUnit:DU")]
    DobsonUnit9,
    /// Original label: "farad", Code: "037", Notation: "F"
    #[serde(rename = "wmoUnit:F")]
    Farad,
    /// Original label: "gray", Code: "081", Notation: "Gy"
    #[serde(rename = "wmoUnit:Gy")]
    Gray,
    /// Original label: "henry", Code: "042", Notation: "H"
    #[serde(rename = "wmoUnit:H")]
    Henry,
    /// Original label: "hertz", Code: "030", Notation: "Hz"
    #[serde(rename = "wmoUnit:Hz")]
    Hertz,
    /// Original label: "joule", Code: "033", Notation: "J"
    #[serde(rename = "wmoUnit:J")]
    Joule,
    /// Original label: "joules per kilogram", Code: "806", Notation: "J_kg-1"
    #[serde(rename = "wmoUnit:J_kg-1")]
    JoulesPerKilogram,
    /// Original label: "joules per square metre", Code: "805", Notation: "J_m-2"
    #[serde(rename = "wmoUnit:J_m-2")]
    JoulesPerSquareMetre,
    /// Original label: "kelvin", Code: "005", Notation: "K"
    #[serde(rename = "wmoUnit:K")]
    Kelvin,
    /// Original label: "kelvins per metre", Code: "786", Notation: "K_m-1"
    #[serde(rename = "wmoUnit:K_m-1")]
    KelvinsPerMetre,
    /// Original label: "kelvin square metres per kilogram per second", Code: "787", Notation: "K_m2_kg-1_s-1"
    #[serde(rename = "wmoUnit:K_m2_kg-1_s-1")]
    KelvinSquareMetresPerKilogramPerSecond,
    /// Original label: "kelvin metres per second", Code: "785", Notation: "K_m_s-1"
    #[serde(rename = "wmoUnit:K_m_s-1")]
    KelvinMetresPerSecond,
    /// Original label: "newton", Code: "031", Notation: "N"
    #[serde(rename = "wmoUnit:N")]
    Newton,
    /// Original label: "newtons per square metre", Code: "795", Notation: "N_m-2"
    #[serde(rename = "wmoUnit:N_m-2")]
    NewtonsPerSquareMetre,
    /// Original label: "N units", Code: "842", Notation: "N_units"
    #[serde(rename = "wmoUnit:N_units")]
    NUnits,
    /// Original label: "ohm", Code: "038", Notation: "Ohm"
    #[serde(rename = "wmoUnit:Ohm")]
    Ohm,
    /// Original label: "pascal", Code: "032", Notation: "Pa"
    #[serde(rename = "wmoUnit:Pa")]
    Pascal,
    /// Original label: "pascals per second", Code: "800", Notation: "Pa_s-1"
    #[serde(rename = "wmoUnit:Pa_s-1")]
    PascalsPerSecond,
    /// Original label: "siemens", Code: "039", Notation: "S"
    #[serde(rename = "wmoUnit:S")]
    Siemens,
    /// Original label: "siemens per metre", Code: "820", Notation: "S_m-1"
    #[serde(rename = "wmoUnit:S_m-1")]
    SiemensPerMetre,
    /// Original label: "sievert", Code: "082", Notation: "Sv"
    #[serde(rename = "wmoUnit:Sv")]
    Sievert,
    /// Original label: "tesla", Code: "041", Notation: "T"
    #[serde(rename = "wmoUnit:T")]
    Tesla,
    /// Original label: "volt", Code: "036", Notation: "V"
    #[serde(rename = "wmoUnit:V")]
    Volt,

    /// Original label: "watt", Code: "034", Notation: "W"
    #[serde(rename = "wmoUnit:W")]
    Watt,
    /// Original label: "kilowatt", Notation: "kW"
    #[serde(rename = "wmoUnit:kW")]
    Kilowatt,
    /// Original label: "watts per metre per steradian", Code: "810", Notation: "W_m-1_sr-1"
    #[serde(rename = "wmoUnit:W_m-1_sr-1")]
    WattsPerMetrePerSteradian,
    /// Original label: "watts per square metre", Code: "811", Notation: "W_m-2"
    #[serde(rename = "wmoUnit:W_m-2")]
    WattsPerSquareMetre,
    /// Original label: "watts per square metre per steradian", Code: "812", Notation: "W_m-2_sr-1"
    #[serde(rename = "wmoUnit:W_m-2_sr-1")]
    WattsPerSquareMetrePerSteradian,
    /// Original label: "watts per square metre per steradian centimetre", Code: "813", Notation: "W_m-2_sr-1_cm"
    #[serde(rename = "wmoUnit:W_m-2_sr-1_cm")]
    WattsPerSquareMetrePerSteradianCentimetre,
    /// Original label: "watts per square metre per steradian metre", Code: "814", Notation: "W_m-2_sr-1_m"
    #[serde(rename = "wmoUnit:W_m-2_sr-1_m")]
    WattsPerSquareMetrePerSteradianMetre,
    /// Original label: "watts per cubic metre per steradian", Code: "815", Notation: "W_m-3_sr-1"
    #[serde(rename = "wmoUnit:W_m-3_sr-1")]
    WattsPerCubicMetrePerSteradian,
    /// Original label: "weber", Code: "040", Notation: "Wb"
    #[serde(rename = "wmoUnit:Wb")]
    Weber,
    /// Original label: "year", Code: "231", Notation: "a"
    #[serde(rename = "wmoUnit:a")]
    Year,
    /// Original label: "centibars per 12 hours", Code: "522", Notation: "cb_-1"
    #[serde(rename = "wmoUnit:cb_-1")]
    CentibarsPer12Hours,
    /// Original label: "centibars per second", Code: "521", Notation: "cb_s-1"
    #[serde(rename = "wmoUnit:cb_s-1")]
    CentibarsPerSecond,
    /// Original label: "candela", Code: "007", Notation: "cd"
    #[serde(rename = "wmoUnit:cd")]
    Candela,
    /// Original label: "centimetre", Code: "715", Notation: "cm"
    #[serde(rename = "wmoUnit:cm")]
    Centimetre,
    /// Original label: "centimetres per hour", Code: "717", Notation: "cm_h-1"
    #[serde(rename = "wmoUnit:cm_h-1")]
    CentimetresPerHour,
    /// Original label: "centimetres per second", Code: "716", Notation: "cm_s-1"
    #[serde(rename = "wmoUnit:cm_s-1")]
    CentimetresPerSecond,
    /// Original label: "day", Code: "132", Notation: "d"
    #[serde(rename = "wmoUnit:d")]
    Day,
    /// Original label: "decibel (6)", Code: "210", Notation: "dB"
    #[serde(rename = "wmoUnit:dB")]
    Decibel6,
    /// Original label: "decibels per degree", Code: "836", Notation: "dB_deg-1"
    #[serde(rename = "wmoUnit:dB_deg-1")]
    DecibelsPerDegree,
    /// Original label: "decibels per metre", Code: "835", Notation: "dB_m-1"
    #[serde(rename = "wmoUnit:dB_m-1")]
    DecibelsPerMetre,
    /// Original label: "decipascals per second (microbar per second)", Code: "520", Notation: "dPa_s-1"
    #[serde(rename = "wmoUnit:dPa_s-1")]
    DecipascalsPerSecondMicrobarPerSecond,
    /// Original label: "dekapascal", Code: "523", Notation: "daPa"
    #[serde(rename = "wmoUnit:daPa")]
    Dekapascal,
    /// Original label: "square degrees", Code: "825", Notation: "deg2"
    #[serde(rename = "wmoUnit:deg2")]
    SquareDegrees,
    /// Original label: "degrees Celsius (8)", Code: "350", Notation: "degC"
    #[serde(rename = "wmoUnit:degC")]
    DegreesCelsius8,
    /// Original label: "degrees per second", Code: "321", Notation: "deg_s-1"
    #[serde(rename = "wmoUnit:deg_s-1")]
    DegreesPerSecond,
    /// Original label: "degree (angle)", Code: "110", Notation: "degree_(angle)"
    #[serde(rename = "wmoUnit:degree_(angle)")]
    DegreeAngle,
    /// Original label: "degrees true", Code: "320", Notation: "degrees_true"
    #[serde(rename = "wmoUnit:degrees_true")]
    DegreesTrue,
    /// Original label: "decimetre", Code: "720", Notation: "dm"
    #[serde(rename = "wmoUnit:dm")]
    Decimetre,
    /// Original label: "electron volt", Code: "160", Notation: "eV"
    #[serde(rename = "wmoUnit:eV")]
    ElectronVolt,
    /// Original label: "foot", Code: "510", Notation: "ft"
    #[serde(rename = "wmoUnit:ft")]
    Foot,
    /// Original label: "acceleration due to gravity", Code: "630", Notation: "g"
    #[serde(rename = "wmoUnit:g")]
    AccelerationDueToGravity,
    /// Original label: "grams per kilogram", Code: "620", Notation: "g_kg-1"
    #[serde(rename = "wmoUnit:g_kg-1")]
    GramsPerKilogram,
    /// Original label: "grams per kilogram per second", Code: "621", Notation: "g_kg-1_s-1"
    #[serde(rename = "wmoUnit:g_kg-1_s-1")]
    GramsPerKilogramPerSecond,
    /// Original label: "geopotential metre", Code: "631", Notation: "gpm"
    #[serde(rename = "wmoUnit:gpm")]
    GeopotentialMetre,
    /// Original label: "hour", Code: "131", Notation: "h"
    #[serde(rename = "wmoUnit:h")]
    Hour,
    /// Original label: "hectopascal", Code: "530", Notation: "hPa"
    #[serde(rename = "wmoUnit:hPa")]
    Hectopascal,
    /// Original label: "hectopascals per 3 hours", Code: "533", Notation: "hPa_-1"
    #[serde(rename = "wmoUnit:hPa_-1")]
    HectopascalsPer3Hours,
    /// Original label: "hectopascals per hour", Code: "532", Notation: "hPa_h-1"
    #[serde(rename = "wmoUnit:hPa_h-1")]
    HectopascalsPerHour,
    /// Original label: "hectopascals per second", Code: "531", Notation: "hPa_s-1"
    #[serde(rename = "wmoUnit:hPa_s-1")]
    HectopascalsPerSecond,
    /// Original label: "hectare", Code: "220", Notation: "ha"
    #[serde(rename = "wmoUnit:ha")]
    Hectare,
    /// Original label: "kilopascal", Code: "801", Notation: "kPa"
    #[serde(rename = "wmoUnit:kPa")]
    Kilopascal,
    /// Original label: "kilogram", Code: "002", Notation: "kg"
    #[serde(rename = "wmoUnit:kg")]
    Kilogram,
    /// Original label: "per square kilogram per second", Code: "778", Notation: "kg-2_s-1"
    #[serde(rename = "wmoUnit:kg-2_s-1")]
    PerSquareKilogramPerSecond,
    /// Original label: "kilograms per kilogram", Code: "622", Notation: "kg_kg-1"
    #[serde(rename = "wmoUnit:kg_kg-1")]
    KilogramsPerKilogram,
    /// Original label: "kilograms per kilogram per second", Code: "623", Notation: "kg_kg-1_s-1"
    #[serde(rename = "wmoUnit:kg_kg-1_s-1")]
    KilogramsPerKilogramPerSecond,
    /// Original label: "kilograms per metre", Code: "775", Notation: "kg_m-1"
    #[serde(rename = "wmoUnit:kg_m-1")]
    KilogramsPerMetre,
    /// Original label: "kilograms per square metre", Code: "624", Notation: "kg_m-2"
    #[serde(rename = "wmoUnit:kg_m-2")]
    KilogramsPerSquareMetre,
    /// Original label: "kilograms per square metre per second", Code: "776", Notation: "kg_m-2_s-1"
    #[serde(rename = "wmoUnit:kg_m-2_s-1")]
    KilogramsPerSquareMetrePerSecond,
    /// Original label: "kilograms per cubic metre", Code: "777", Notation: "kg_m-3"
    #[serde(rename = "wmoUnit:kg_m-3")]
    KilogramsPerCubicMetre,
    /// Original label: "kilometre", Code: "740", Notation: "km"
    #[serde(rename = "wmoUnit:km")]
    Kilometre,
    /// Original label: "kilometres per day", Code: "742", Notation: "km_d-1"
    #[serde(rename = "wmoUnit:km_d-1")]
    KilometresPerDay,
    /// Original label: "kilometres per hour", Code: "741", Notation: "km_h-1"
    #[serde(rename = "wmoUnit:km_h-1")]
    KilometresPerHour,
    /// Original label: "knot", Code: "201", Notation: "kt"
    #[serde(rename = "wmoUnit:kt")]
    Knot,
    /// Original label: "knots per 1000 metres", Code: "501", Notation: "kt_km-1"
    #[serde(rename = "wmoUnit:kt_km-1")]
    KnotsPer1000Metres,
    /// Original label: "litre", Code: "120", Notation: "l"
    #[serde(rename = "wmoUnit:l")]
    Litre,
    /// Original label: "lumen", Code: "070", Notation: "lm"
    #[serde(rename = "wmoUnit:lm")]
    Lumen,
    /// Original label: "logarithm per metre", Code: "772", Notation: "log_(m-1)"
    #[serde(rename = "wmoUnit:log_(m-1)")]
    LogarithmPerMetre,
    /// Original label: "logarithm per square metre", Code: "773", Notation: "log_(m-2)"
    #[serde(rename = "wmoUnit:log_(m-2)")]
    LogarithmPerSquareMetre,
    /// Original label: "lux", Code: "071", Notation: "lx"
    #[serde(rename = "wmoUnit:lx")]
    Lux,
    /// Original label: "metre", Code: "001", Notation: "m"
    #[serde(rename = "wmoUnit:m")]
    Metre,
    /// Original label: "per metre", Code: "743", Notation: "m-1"
    #[serde(rename = "wmoUnit:m-1")]
    PerMetre,
    /// Original label: "square metres", Code: "734", Notation: "m2"
    #[serde(rename = "wmoUnit:m2")]
    SquareMetres,
    /// Original label: "metres to the two thirds power per second", Code: "769", Notation: "m2_-1"
    #[serde(rename = "wmoUnit:m2_-1")]
    MetresToTheTwoThirdsPowerPerSecond,
    /// Original label: "square metres per hertz", Code: "764", Notation: "m2_Hz-1"
    #[serde(rename = "wmoUnit:m2_Hz-1")]
    SquareMetresPerHertz,
    /// Original label: "square metres per radian squared", Code: "763", Notation: "m2_rad-1_s"
    #[serde(rename = "wmoUnit:m2_rad-1_s")]
    SquareMetresPerRadianSquared,
    /// Original label: "square metres second", Code: "761", Notation: "m2_s"
    #[serde(rename = "wmoUnit:m2_s")]
    SquareMetresSecond,
    /// Original label: "square metres per second", Code: "735", Notation: "m2_s-1"
    #[serde(rename = "wmoUnit:m2_s-1")]
    SquareMetresPerSecond,
    /// Original label: "square metres per second squared", Code: "762", Notation: "m2_s-2"
    #[serde(rename = "wmoUnit:m2_s-2")]
    SquareMetresPerSecondSquared,
    /// Original label: "cubic metres", Code: "765", Notation: "m3"
    #[serde(rename = "wmoUnit:m3")]
    CubicMetres,
    /// Original label: "cubic metres per cubic metre", Code: "767", Notation: "m3_m-3"
    #[serde(rename = "wmoUnit:m3_m-3")]
    CubicMetresPerCubicMetre,
    /// Original label: "cubic metres per second", Code: "766", Notation: "m3_s-1"
    #[serde(rename = "wmoUnit:m3_s-1")]
    CubicMetresPerSecond,
    /// Original label: "metres to the fourth power", Code: "768", Notation: "m4"
    #[serde(rename = "wmoUnit:m4")]
    MetresToTheFourthPower,
    /// Original label: "millisievert", Code: "753", Notation: "mSv"
    #[serde(rename = "wmoUnit:mSv")]
    Millisievert,
    /// Original label: "metres per second", Code: "731", Notation: "m_s-1"
    #[serde(rename = "wmoUnit:m_s-1")]
    MetresPerSecond,
    /// Original label: "metres per second per 1000 metres", Code: "733", Notation: "m_s-1_km-1"
    #[serde(rename = "wmoUnit:m_s-1_km-1")]
    MetresPerSecondPer1000Metres,
    /// Original label: "metres per second per metre", Code: "732", Notation: "m_s-1_m-1"
    #[serde(rename = "wmoUnit:m_s-1_m-1")]
    MetresPerSecondPerMetre,
    /// Original label: "metres per second squared", Code: "760", Notation: "m_s-2"
    #[serde(rename = "wmoUnit:m_s-2")]
    MetresPerSecondSquared,
    /// Original label: "minute (time)", Code: "130", Notation: "min"
    #[serde(rename = "wmoUnit:min")]
    MinuteTime,
    /// Original label: "millimetre", Code: "710", Notation: "mm"
    #[serde(rename = "wmoUnit:mm")]
    Millimetre,
    /// Original label: "millimetres per the sixth power per cubic metre", Code: "713", Notation: "mm6_m-3"
    #[serde(rename = "wmoUnit:mm6_m-3")]
    MillimetresPerTheSixthPowerPerCubicMetre,
    /// Original label: "millimetres per hour", Code: "712", Notation: "mm_h-1"
    #[serde(rename = "wmoUnit:mm_h-1")]
    MillimetresPerHour,
    /// Original label: "millimetres per seconds", Code: "711", Notation: "mm_s-1"
    #[serde(rename = "wmoUnit:mm_s-1")]
    MillimetresPerSeconds,
    /// Original label: "mole", Code: "006", Notation: "mol"
    #[serde(rename = "wmoUnit:mol")]
    Mole,
    /// Original label: "moles per mole", Code: "788", Notation: "mol_mol-1"
    #[serde(rename = "wmoUnit:mol_mol-1")]
    MolesPerMole,
    /// Original label: "month", Code: "430", Notation: "mon"
    #[serde(rename = "wmoUnit:mon")]
    Month,
    /// Original label: "nautical mile", Code: "200", Notation: "nautical_mile"
    #[serde(rename = "wmoUnit:nautical_mile")]
    NauticalMile,
    /// Original label: "nanobar = hPa 10^-6", Code: "535", Notation: "nbar"
    #[serde(rename = "wmoUnit:nbar")]
    NanobarHpa106,
    /// Original label: "eighths of cloud", Code: "310", Notation: "okta"
    #[serde(rename = "wmoUnit:okta")]
    EighthsOfCloud,
    /// Original label: "pH unit", Code: "841", Notation: "pH_unit"
    #[serde(rename = "wmoUnit:pH_unit")]
    PhUnit,
    /// Original label: "parsec", Code: "171", Notation: "pc"
    #[serde(rename = "wmoUnit:pc")]
    Parsec,
    /// Original label: "per cent", Code: "300", Notation: "percent"
    #[serde(rename = "wmoUnit:percent")]
    PerCent,
    /// Original label: "radian", Code: "021", Notation: "rad"
    #[serde(rename = "wmoUnit:rad")]
    Radian,
    /// Original label: "radians per metre", Code: "790", Notation: "rad_m-1"
    #[serde(rename = "wmoUnit:rad_m-1")]
    RadiansPerMetre,
    /// Original label: "second", Code: "003", Notation: "s"
    #[serde(rename = "wmoUnit:s")]
    Second,
    /// Original label: "per second (same as hertz)", Code: "441", Notation: "s-1"
    #[serde(rename = "wmoUnit:s-1")]
    PerSecondSameAsHertz,
    /// Original label: "per second squared", Code: "442", Notation: "s-2"
    #[serde(rename = "wmoUnit:s-2")]
    PerSecondSquared,
    /// Original label: "seconds per metre", Code: "779", Notation: "s_m-1"
    #[serde(rename = "wmoUnit:s_m-1")]
    SecondsPerMetre,
    /// Original label: "steradian", Code: "022", Notation: "sr"
    #[serde(rename = "wmoUnit:sr")]
    Steradian,
    /// Original label: "tonne", Code: "150", Notation: "t"
    #[serde(rename = "wmoUnit:t")]
    Tonne,
    /// Original label: "atomic mass unit", Code: "161", Notation: "u"
    #[serde(rename = "wmoUnit:u")]
    AtomicMassUnit,
    /// Original label: "week", Code: "230", Notation: "week"
    #[serde(rename = "wmoUnit:week")]
    Week,
}

impl WmoUnitCode {
    /// Returns the original `skos:prefLabel` for the unit.
    pub fn pref_label(&self) -> &'static str {
        match self {
            WmoUnitCode::MinuteAngle => "minute (angle)",
            WmoUnitCode::SecondAngle => "second (angle)",
            WmoUnitCode::PartsPerThousand => "parts per thousand",
            WmoUnitCode::Dimensionless => "Dimensionless",
            WmoUnitCode::Ampere => "ampere",
            WmoUnitCode::AstronomicUnit => "astronomic unit",
            WmoUnitCode::Becquerel => "becquerel",
            WmoUnitCode::BecquerelsPerLitre => "becquerels per litre",
            WmoUnitCode::BecquerelsPerSquareMetre => "becquerels per square metre",
            WmoUnitCode::BecquerelsPerCubicMetre => "becquerels per cubic metre",
            WmoUnitCode::BecquerelSecondsPerCubicMetre => "becquerel seconds per cubic metre",
            WmoUnitCode::Coulomb => "coulomb",
            WmoUnitCode::DegreesCelsiusPer100Metres => "degrees Celsius per 100 metres",
            WmoUnitCode::DegreesCelsiusPerMetre => "degrees Celsius per metre",
            WmoUnitCode::DegreeCelsius => "degree Celsius",
            WmoUnitCode::DobsonUnit9 => "Dobson Unit (9)",
            WmoUnitCode::Farad => "farad",
            WmoUnitCode::Gray => "gray",
            WmoUnitCode::Henry => "henry",
            WmoUnitCode::Hertz => "hertz",
            WmoUnitCode::Joule => "joule",
            WmoUnitCode::JoulesPerKilogram => "joules per kilogram",
            WmoUnitCode::JoulesPerSquareMetre => "joules per square metre",
            WmoUnitCode::Kelvin => "kelvin",
            WmoUnitCode::KelvinsPerMetre => "kelvins per metre",
            WmoUnitCode::KelvinSquareMetresPerKilogramPerSecond => {
                "kelvin square metres per kilogram per second"
            }
            WmoUnitCode::KelvinMetresPerSecond => "kelvin metres per second",
            WmoUnitCode::Newton => "newton",
            WmoUnitCode::NewtonsPerSquareMetre => "newtons per square metre",
            WmoUnitCode::NUnits => "N units",
            WmoUnitCode::Ohm => "ohm",
            WmoUnitCode::Pascal => "pascal",
            WmoUnitCode::PascalsPerSecond => "pascals per second",
            WmoUnitCode::Siemens => "siemens",
            WmoUnitCode::SiemensPerMetre => "siemens per metre",
            WmoUnitCode::Sievert => "sievert",
            WmoUnitCode::Tesla => "tesla",
            WmoUnitCode::Volt => "volt",
            WmoUnitCode::Watt => "watt",
            WmoUnitCode::Kilowatt => "kilowatt",
            WmoUnitCode::WattsPerMetrePerSteradian => "watts per metre per steradian",
            WmoUnitCode::WattsPerSquareMetre => "watts per square metre",
            WmoUnitCode::WattsPerSquareMetrePerSteradian => "watts per square metre per steradian",
            WmoUnitCode::WattsPerSquareMetrePerSteradianCentimetre => {
                "watts per square metre per steradian centimetre"
            }
            WmoUnitCode::WattsPerSquareMetrePerSteradianMetre => {
                "watts per square metre per steradian metre"
            }
            WmoUnitCode::WattsPerCubicMetrePerSteradian => "watts per cubic metre per steradian",
            WmoUnitCode::Weber => "weber",
            WmoUnitCode::Year => "year",
            WmoUnitCode::CentibarsPer12Hours => "centibars per 12 hours",
            WmoUnitCode::CentibarsPerSecond => "centibars per second",
            WmoUnitCode::Candela => "candela",
            WmoUnitCode::Centimetre => "centimetre",
            WmoUnitCode::CentimetresPerHour => "centimetres per hour",
            WmoUnitCode::CentimetresPerSecond => "centimetres per second",
            WmoUnitCode::Day => "day",
            WmoUnitCode::Decibel6 => "decibel (6)",
            WmoUnitCode::DecibelsPerDegree => "decibels per degree",
            WmoUnitCode::DecibelsPerMetre => "decibels per metre",
            WmoUnitCode::DecipascalsPerSecondMicrobarPerSecond => {
                "decipascals per second (microbar per second)"
            }
            WmoUnitCode::Dekapascal => "dekapascal",
            WmoUnitCode::SquareDegrees => "square degrees",
            WmoUnitCode::DegreesCelsius8 => "degrees Celsius (8)",
            WmoUnitCode::DegreesPerSecond => "degrees per second",
            WmoUnitCode::DegreeAngle => "degree (angle)",
            WmoUnitCode::DegreesTrue => "degrees true",
            WmoUnitCode::Decimetre => "decimetre",
            WmoUnitCode::ElectronVolt => "electron volt",
            WmoUnitCode::Foot => "foot",
            WmoUnitCode::AccelerationDueToGravity => "acceleration due to gravity",
            WmoUnitCode::GramsPerKilogram => "grams per kilogram",
            WmoUnitCode::GramsPerKilogramPerSecond => "grams per kilogram per second",
            WmoUnitCode::GeopotentialMetre => "geopotential metre",
            WmoUnitCode::Hour => "hour",
            WmoUnitCode::Hectopascal => "hectopascal",
            WmoUnitCode::HectopascalsPer3Hours => "hectopascals per 3 hours",
            WmoUnitCode::HectopascalsPerHour => "hectopascals per hour",
            WmoUnitCode::HectopascalsPerSecond => "hectopascals per second",
            WmoUnitCode::Hectare => "hectare",
            WmoUnitCode::Kilopascal => "kilopascal",
            WmoUnitCode::Kilogram => "kilogram",
            WmoUnitCode::PerSquareKilogramPerSecond => "per square kilogram per second",
            WmoUnitCode::KilogramsPerKilogram => "kilograms per kilogram",
            WmoUnitCode::KilogramsPerKilogramPerSecond => "kilograms per kilogram per second",
            WmoUnitCode::KilogramsPerMetre => "kilograms per metre",
            WmoUnitCode::KilogramsPerSquareMetre => "kilograms per square metre",
            WmoUnitCode::KilogramsPerSquareMetrePerSecond => {
                "kilograms per square metre per second"
            }
            WmoUnitCode::KilogramsPerCubicMetre => "kilograms per cubic metre",
            WmoUnitCode::Kilometre => "kilometre",
            WmoUnitCode::KilometresPerDay => "kilometres per day",
            WmoUnitCode::KilometresPerHour => "kilometres per hour",
            WmoUnitCode::Knot => "knot",
            WmoUnitCode::KnotsPer1000Metres => "knots per 1000 metres",
            WmoUnitCode::Litre => "litre",
            WmoUnitCode::Lumen => "lumen",
            WmoUnitCode::LogarithmPerMetre => "logarithm per metre",
            WmoUnitCode::LogarithmPerSquareMetre => "logarithm per square metre",
            WmoUnitCode::Lux => "lux",
            WmoUnitCode::Metre => "metre",
            WmoUnitCode::PerMetre => "per metre",
            WmoUnitCode::SquareMetres => "square metres",
            WmoUnitCode::MetresToTheTwoThirdsPowerPerSecond => {
                "metres to the two thirds power per second"
            }
            WmoUnitCode::SquareMetresPerHertz => "square metres per hertz",
            WmoUnitCode::SquareMetresPerRadianSquared => "square metres per radian squared",
            WmoUnitCode::SquareMetresSecond => "square metres second",
            WmoUnitCode::SquareMetresPerSecond => "square metres per second",
            WmoUnitCode::SquareMetresPerSecondSquared => "square metres per second squared",
            WmoUnitCode::CubicMetres => "cubic metres",
            WmoUnitCode::CubicMetresPerCubicMetre => "cubic metres per cubic metre",
            WmoUnitCode::CubicMetresPerSecond => "cubic metres per second",
            WmoUnitCode::MetresToTheFourthPower => "metres to the fourth power",
            WmoUnitCode::Millisievert => "millisievert",
            WmoUnitCode::MetresPerSecond => "metres per second",
            WmoUnitCode::MetresPerSecondPer1000Metres => "metres per second per 1000 metres",
            WmoUnitCode::MetresPerSecondPerMetre => "metres per second per metre",
            WmoUnitCode::MetresPerSecondSquared => "metres per second squared",
            WmoUnitCode::MinuteTime => "minute (time)",
            WmoUnitCode::Millimetre => "millimetre",
            WmoUnitCode::MillimetresPerTheSixthPowerPerCubicMetre => {
                "millimetres per the sixth power per cubic metre"
            }
            WmoUnitCode::MillimetresPerHour => "millimetres per hour",
            WmoUnitCode::MillimetresPerSeconds => "millimetres per seconds",
            WmoUnitCode::Mole => "mole",
            WmoUnitCode::MolesPerMole => "moles per mole",
            WmoUnitCode::Month => "month",
            WmoUnitCode::NauticalMile => "nautical mile",
            WmoUnitCode::NanobarHpa106 => "nanobar = hPa 10^-6",
            WmoUnitCode::EighthsOfCloud => "eighths of cloud",
            WmoUnitCode::PhUnit => "pH unit",
            WmoUnitCode::Parsec => "parsec",
            WmoUnitCode::PerCent => "per cent",
            WmoUnitCode::Radian => "radian",
            WmoUnitCode::RadiansPerMetre => "radians per metre",
            WmoUnitCode::Second => "second",
            WmoUnitCode::PerSecondSameAsHertz => "per second (same as hertz)",
            WmoUnitCode::PerSecondSquared => "per second squared",
            WmoUnitCode::SecondsPerMetre => "seconds per metre",
            WmoUnitCode::Steradian => "steradian",
            WmoUnitCode::Tonne => "tonne",
            WmoUnitCode::AtomicMassUnit => "atomic mass unit",
            WmoUnitCode::Week => "week",
        }
    }

    /// Returns the `code_figure` for the unit (often a numeric ID).
    pub fn code(&self) -> &'static str {
        match self {
            WmoUnitCode::MinuteAngle => "111",
            WmoUnitCode::SecondAngle => "112",
            WmoUnitCode::PartsPerThousand => "301",
            WmoUnitCode::Dimensionless => "000",
            WmoUnitCode::Ampere => "004",
            WmoUnitCode::AstronomicUnit => "170",
            WmoUnitCode::Becquerel => "080",
            WmoUnitCode::BecquerelsPerLitre => "750",
            WmoUnitCode::BecquerelsPerSquareMetre => "751",
            WmoUnitCode::BecquerelsPerCubicMetre => "752",
            WmoUnitCode::BecquerelSecondsPerCubicMetre => "830",
            WmoUnitCode::Coulomb => "035",
            WmoUnitCode::DegreesCelsiusPer100Metres => "352",
            WmoUnitCode::DegreesCelsiusPerMetre => "351",
            WmoUnitCode::DegreeCelsius => "060",
            WmoUnitCode::DobsonUnit9 => "360",
            WmoUnitCode::Farad => "037",
            WmoUnitCode::Gray => "081",
            WmoUnitCode::Henry => "042",
            WmoUnitCode::Hertz => "030",
            WmoUnitCode::Joule => "033",
            WmoUnitCode::JoulesPerKilogram => "806",
            WmoUnitCode::JoulesPerSquareMetre => "805",
            WmoUnitCode::Kelvin => "005",
            WmoUnitCode::KelvinsPerMetre => "786",
            WmoUnitCode::KelvinSquareMetresPerKilogramPerSecond => "787",
            WmoUnitCode::KelvinMetresPerSecond => "785",
            WmoUnitCode::Newton => "031",
            WmoUnitCode::NewtonsPerSquareMetre => "795",
            WmoUnitCode::NUnits => "842",
            WmoUnitCode::Ohm => "038",
            WmoUnitCode::Pascal => "032",
            WmoUnitCode::PascalsPerSecond => "800",
            WmoUnitCode::Siemens => "039",
            WmoUnitCode::SiemensPerMetre => "820",
            WmoUnitCode::Sievert => "082",
            WmoUnitCode::Tesla => "041",
            WmoUnitCode::Volt => "036",
            WmoUnitCode::Watt => "034",
            WmoUnitCode::Kilowatt => "807",
            WmoUnitCode::WattsPerMetrePerSteradian => "810",
            WmoUnitCode::WattsPerSquareMetre => "811",
            WmoUnitCode::WattsPerSquareMetrePerSteradian => "812",
            WmoUnitCode::WattsPerSquareMetrePerSteradianCentimetre => "813",
            WmoUnitCode::WattsPerSquareMetrePerSteradianMetre => "814",
            WmoUnitCode::WattsPerCubicMetrePerSteradian => "815",
            WmoUnitCode::Weber => "040",
            WmoUnitCode::Year => "231",
            WmoUnitCode::CentibarsPer12Hours => "522",
            WmoUnitCode::CentibarsPerSecond => "521",
            WmoUnitCode::Candela => "007",
            WmoUnitCode::Centimetre => "715",
            WmoUnitCode::CentimetresPerHour => "717",
            WmoUnitCode::CentimetresPerSecond => "716",
            WmoUnitCode::Day => "132",
            WmoUnitCode::Decibel6 => "210",
            WmoUnitCode::DecibelsPerDegree => "836",
            WmoUnitCode::DecibelsPerMetre => "835",
            WmoUnitCode::DecipascalsPerSecondMicrobarPerSecond => "520",
            WmoUnitCode::Dekapascal => "523",
            WmoUnitCode::SquareDegrees => "825",
            WmoUnitCode::DegreesCelsius8 => "350",
            WmoUnitCode::DegreesPerSecond => "321",
            WmoUnitCode::DegreeAngle => "110",
            WmoUnitCode::DegreesTrue => "320",
            WmoUnitCode::Decimetre => "720",
            WmoUnitCode::ElectronVolt => "160",
            WmoUnitCode::Foot => "510",
            WmoUnitCode::AccelerationDueToGravity => "630",
            WmoUnitCode::GramsPerKilogram => "620",
            WmoUnitCode::GramsPerKilogramPerSecond => "621",
            WmoUnitCode::GeopotentialMetre => "631",
            WmoUnitCode::Hour => "131",
            WmoUnitCode::Hectopascal => "530",
            WmoUnitCode::HectopascalsPer3Hours => "533",
            WmoUnitCode::HectopascalsPerHour => "532",
            WmoUnitCode::HectopascalsPerSecond => "531",
            WmoUnitCode::Hectare => "220",
            WmoUnitCode::Kilopascal => "801",
            WmoUnitCode::Kilogram => "002",
            WmoUnitCode::PerSquareKilogramPerSecond => "778",
            WmoUnitCode::KilogramsPerKilogram => "622",
            WmoUnitCode::KilogramsPerKilogramPerSecond => "623",
            WmoUnitCode::KilogramsPerMetre => "775",
            WmoUnitCode::KilogramsPerSquareMetre => "624",
            WmoUnitCode::KilogramsPerSquareMetrePerSecond => "776",
            WmoUnitCode::KilogramsPerCubicMetre => "777",
            WmoUnitCode::Kilometre => "740",
            WmoUnitCode::KilometresPerDay => "742",
            WmoUnitCode::KilometresPerHour => "741",
            WmoUnitCode::Knot => "201",
            WmoUnitCode::KnotsPer1000Metres => "501",
            WmoUnitCode::Litre => "120",
            WmoUnitCode::Lumen => "070",
            WmoUnitCode::LogarithmPerMetre => "772",
            WmoUnitCode::LogarithmPerSquareMetre => "773",
            WmoUnitCode::Lux => "071",
            WmoUnitCode::Metre => "001",
            WmoUnitCode::PerMetre => "743",
            WmoUnitCode::SquareMetres => "734",
            WmoUnitCode::MetresToTheTwoThirdsPowerPerSecond => "769",
            WmoUnitCode::SquareMetresPerHertz => "764",
            WmoUnitCode::SquareMetresPerRadianSquared => "763",
            WmoUnitCode::SquareMetresSecond => "761",
            WmoUnitCode::SquareMetresPerSecond => "735",
            WmoUnitCode::SquareMetresPerSecondSquared => "762",
            WmoUnitCode::CubicMetres => "765",
            WmoUnitCode::CubicMetresPerCubicMetre => "767",
            WmoUnitCode::CubicMetresPerSecond => "766",
            WmoUnitCode::MetresToTheFourthPower => "768",
            WmoUnitCode::Millisievert => "753",
            WmoUnitCode::MetresPerSecond => "731",
            WmoUnitCode::MetresPerSecondPer1000Metres => "733",
            WmoUnitCode::MetresPerSecondPerMetre => "732",
            WmoUnitCode::MetresPerSecondSquared => "760",
            WmoUnitCode::MinuteTime => "130",
            WmoUnitCode::Millimetre => "710",
            WmoUnitCode::MillimetresPerTheSixthPowerPerCubicMetre => "713",
            WmoUnitCode::MillimetresPerHour => "712",
            WmoUnitCode::MillimetresPerSeconds => "711",
            WmoUnitCode::Mole => "006",
            WmoUnitCode::MolesPerMole => "788",
            WmoUnitCode::Month => "430",
            WmoUnitCode::NauticalMile => "200",
            WmoUnitCode::NanobarHpa106 => "535",
            WmoUnitCode::EighthsOfCloud => "310",
            WmoUnitCode::PhUnit => "841",
            WmoUnitCode::Parsec => "171",
            WmoUnitCode::PerCent => "300",
            WmoUnitCode::Radian => "021",
            WmoUnitCode::RadiansPerMetre => "790",
            WmoUnitCode::Second => "003",
            WmoUnitCode::PerSecondSameAsHertz => "441",
            WmoUnitCode::PerSecondSquared => "442",
            WmoUnitCode::SecondsPerMetre => "779",
            WmoUnitCode::Steradian => "022",
            WmoUnitCode::Tonne => "150",
            WmoUnitCode::AtomicMassUnit => "161",
            WmoUnitCode::Week => "230",
        }
    }

    /// Returns the `skos:notation` for the unit (e.g., 'degC', 'm/s').
    pub fn notation(&self) -> &'static str {
        match self {
            WmoUnitCode::MinuteAngle => "",
            WmoUnitCode::SecondAngle => "",
            WmoUnitCode::PartsPerThousand => "0.001",
            WmoUnitCode::Dimensionless => "1",
            WmoUnitCode::Ampere => "A",
            WmoUnitCode::AstronomicUnit => "AU",
            WmoUnitCode::Becquerel => "Bq",
            WmoUnitCode::BecquerelsPerLitre => "Bq_l-1",
            WmoUnitCode::BecquerelsPerSquareMetre => "Bq_m-2",
            WmoUnitCode::BecquerelsPerCubicMetre => "Bq_m-3",
            WmoUnitCode::BecquerelSecondsPerCubicMetre => "Bq_s_m-3",
            WmoUnitCode::Coulomb => "C",
            WmoUnitCode::DegreesCelsiusPer100Metres => "C_-1",
            WmoUnitCode::DegreesCelsiusPerMetre => "C_m-1",
            WmoUnitCode::DegreeCelsius => "Cel",
            WmoUnitCode::DobsonUnit9 => "DU",
            WmoUnitCode::Farad => "F",
            WmoUnitCode::Gray => "Gy",
            WmoUnitCode::Henry => "H",
            WmoUnitCode::Hertz => "Hz",
            WmoUnitCode::Joule => "J",
            WmoUnitCode::JoulesPerKilogram => "J_kg-1",
            WmoUnitCode::JoulesPerSquareMetre => "J_m-2",
            WmoUnitCode::Kelvin => "K",
            WmoUnitCode::KelvinsPerMetre => "K_m-1",
            WmoUnitCode::KelvinSquareMetresPerKilogramPerSecond => "K_m2_kg-1_s-1",
            WmoUnitCode::KelvinMetresPerSecond => "K_m_s-1",
            WmoUnitCode::Newton => "N",
            WmoUnitCode::NewtonsPerSquareMetre => "N_m-2",
            WmoUnitCode::NUnits => "N_units",
            WmoUnitCode::Ohm => "Ohm",
            WmoUnitCode::Pascal => "Pa",
            WmoUnitCode::PascalsPerSecond => "Pa_s-1",
            WmoUnitCode::Siemens => "S",
            WmoUnitCode::SiemensPerMetre => "S_m-1",
            WmoUnitCode::Sievert => "Sv",
            WmoUnitCode::Tesla => "T",
            WmoUnitCode::Volt => "V",
            WmoUnitCode::Watt => "W",
            WmoUnitCode::Kilowatt => "kW",
            WmoUnitCode::WattsPerMetrePerSteradian => "W_m-1_sr-1",
            WmoUnitCode::WattsPerSquareMetre => "W_m-2",
            WmoUnitCode::WattsPerSquareMetrePerSteradian => "W_m-2_sr-1",
            WmoUnitCode::WattsPerSquareMetrePerSteradianCentimetre => "W_m-2_sr-1_cm",
            WmoUnitCode::WattsPerSquareMetrePerSteradianMetre => "W_m-2_sr-1_m",
            WmoUnitCode::WattsPerCubicMetrePerSteradian => "W_m-3_sr-1",
            WmoUnitCode::Weber => "Wb",
            WmoUnitCode::Year => "a",
            WmoUnitCode::CentibarsPer12Hours => "cb_-1",
            WmoUnitCode::CentibarsPerSecond => "cb_s-1",
            WmoUnitCode::Candela => "cd",
            WmoUnitCode::Centimetre => "cm",
            WmoUnitCode::CentimetresPerHour => "cm_h-1",
            WmoUnitCode::CentimetresPerSecond => "cm_s-1",
            WmoUnitCode::Day => "d",
            WmoUnitCode::Decibel6 => "dB",
            WmoUnitCode::DecibelsPerDegree => "dB_deg-1",
            WmoUnitCode::DecibelsPerMetre => "dB_m-1",
            WmoUnitCode::DecipascalsPerSecondMicrobarPerSecond => "dPa_s-1",
            WmoUnitCode::Dekapascal => "daPa",
            WmoUnitCode::SquareDegrees => "deg2",
            WmoUnitCode::DegreesCelsius8 => "degC",
            WmoUnitCode::DegreesPerSecond => "deg_s-1",
            WmoUnitCode::DegreeAngle => "degree_(angle)",
            WmoUnitCode::DegreesTrue => "degrees_true",
            WmoUnitCode::Decimetre => "dm",
            WmoUnitCode::ElectronVolt => "eV",
            WmoUnitCode::Foot => "ft",
            WmoUnitCode::AccelerationDueToGravity => "g",
            WmoUnitCode::GramsPerKilogram => "g_kg-1",
            WmoUnitCode::GramsPerKilogramPerSecond => "g_kg-1_s-1",
            WmoUnitCode::GeopotentialMetre => "gpm",
            WmoUnitCode::Hour => "h",
            WmoUnitCode::Hectopascal => "hPa",
            WmoUnitCode::HectopascalsPer3Hours => "hPa_-1",
            WmoUnitCode::HectopascalsPerHour => "hPa_h-1",
            WmoUnitCode::HectopascalsPerSecond => "hPa_s-1",
            WmoUnitCode::Hectare => "ha",
            WmoUnitCode::Kilopascal => "kPa",
            WmoUnitCode::Kilogram => "kg",
            WmoUnitCode::PerSquareKilogramPerSecond => "kg-2_s-1",
            WmoUnitCode::KilogramsPerKilogram => "kg_kg-1",
            WmoUnitCode::KilogramsPerKilogramPerSecond => "kg_kg-1_s-1",
            WmoUnitCode::KilogramsPerMetre => "kg_m-1",
            WmoUnitCode::KilogramsPerSquareMetre => "kg_m-2",
            WmoUnitCode::KilogramsPerSquareMetrePerSecond => "kg_m-2_s-1",
            WmoUnitCode::KilogramsPerCubicMetre => "kg_m-3",
            WmoUnitCode::Kilometre => "km",
            WmoUnitCode::KilometresPerDay => "km_d-1",
            WmoUnitCode::KilometresPerHour => "km_h-1",
            WmoUnitCode::Knot => "kt",
            WmoUnitCode::KnotsPer1000Metres => "kt_km-1",
            WmoUnitCode::Litre => "l",
            WmoUnitCode::Lumen => "lm",
            WmoUnitCode::LogarithmPerMetre => "log_(m-1)",
            WmoUnitCode::LogarithmPerSquareMetre => "log_(m-2)",
            WmoUnitCode::Lux => "lx",
            WmoUnitCode::Metre => "m",
            WmoUnitCode::PerMetre => "m-1",
            WmoUnitCode::SquareMetres => "m2",
            WmoUnitCode::MetresToTheTwoThirdsPowerPerSecond => "m2_-1",
            WmoUnitCode::SquareMetresPerHertz => "m2_Hz-1",
            WmoUnitCode::SquareMetresPerRadianSquared => "m2_rad-1_s",
            WmoUnitCode::SquareMetresSecond => "m2_s",
            WmoUnitCode::SquareMetresPerSecond => "m2_s-1",
            WmoUnitCode::SquareMetresPerSecondSquared => "m2_s-2",
            WmoUnitCode::CubicMetres => "m3",
            WmoUnitCode::CubicMetresPerCubicMetre => "m3_m-3",
            WmoUnitCode::CubicMetresPerSecond => "m3_s-1",
            WmoUnitCode::MetresToTheFourthPower => "m4",
            WmoUnitCode::Millisievert => "mSv",
            WmoUnitCode::MetresPerSecond => "m_s-1",
            WmoUnitCode::MetresPerSecondPer1000Metres => "m_s-1_km-1",
            WmoUnitCode::MetresPerSecondPerMetre => "m_s-1_m-1",
            WmoUnitCode::MetresPerSecondSquared => "m_s-2",
            WmoUnitCode::MinuteTime => "min",
            WmoUnitCode::Millimetre => "mm",
            WmoUnitCode::MillimetresPerTheSixthPowerPerCubicMetre => "mm6_m-3",
            WmoUnitCode::MillimetresPerHour => "mm_h-1",
            WmoUnitCode::MillimetresPerSeconds => "mm_s-1",
            WmoUnitCode::Mole => "mol",
            WmoUnitCode::MolesPerMole => "mol_mol-1",
            WmoUnitCode::Month => "mon",
            WmoUnitCode::NauticalMile => "nautical_mile",
            WmoUnitCode::NanobarHpa106 => "nbar",
            WmoUnitCode::EighthsOfCloud => "okta",
            WmoUnitCode::PhUnit => "pH_unit",
            WmoUnitCode::Parsec => "pc",
            WmoUnitCode::PerCent => "percent",
            WmoUnitCode::Radian => "rad",
            WmoUnitCode::RadiansPerMetre => "rad_m-1",
            WmoUnitCode::Second => "s",
            WmoUnitCode::PerSecondSameAsHertz => "s-1",
            WmoUnitCode::PerSecondSquared => "s-2",
            WmoUnitCode::SecondsPerMetre => "s_m-1",
            WmoUnitCode::Steradian => "sr",
            WmoUnitCode::Tonne => "t",
            WmoUnitCode::AtomicMassUnit => "u",
            WmoUnitCode::Week => "week",
        }
    }

    /// Returns the `skos:altLabel` for the unit.
    pub fn alt_label(&self) -> &'static str {
        match self {
            // Custom Minute alt label
            WmoUnitCode::MinuteAngle => "min",
            // Custom Second alt label
            WmoUnitCode::SecondAngle => "sec",
            WmoUnitCode::PartsPerThousand => "‰",
            WmoUnitCode::Dimensionless => "1",
            WmoUnitCode::Ampere => "A",
            WmoUnitCode::AstronomicUnit => "AU",
            WmoUnitCode::Becquerel => "Bq",
            WmoUnitCode::BecquerelsPerLitre => "Bq l^-1",
            WmoUnitCode::BecquerelsPerSquareMetre => "Bq m^-2",
            WmoUnitCode::BecquerelsPerCubicMetre => "Bq m^-3",
            WmoUnitCode::BecquerelSecondsPerCubicMetre => "Bq s m^-3",
            WmoUnitCode::Coulomb => "C",
            WmoUnitCode::DegreesCelsiusPer100Metres => "˚C/100 m",
            WmoUnitCode::DegreesCelsiusPerMetre => "˚C/m",
            WmoUnitCode::DegreeCelsius => "˚C",
            WmoUnitCode::DobsonUnit9 => "DU",
            WmoUnitCode::Farad => "F",
            WmoUnitCode::Gray => "Gy",
            WmoUnitCode::Henry => "H",
            WmoUnitCode::Hertz => "Hz",
            WmoUnitCode::Joule => "J",
            WmoUnitCode::JoulesPerKilogram => "J kg^-1",
            WmoUnitCode::JoulesPerSquareMetre => "J m^-2",
            WmoUnitCode::Kelvin => "K",
            WmoUnitCode::KelvinsPerMetre => "K m^-1",
            WmoUnitCode::KelvinSquareMetresPerKilogramPerSecond => "K m^2 kg^-1 s^-1",
            WmoUnitCode::KelvinMetresPerSecond => "K m s^-1",
            WmoUnitCode::Newton => "N",
            WmoUnitCode::NewtonsPerSquareMetre => "N m^-2",
            WmoUnitCode::NUnits => "N units",
            WmoUnitCode::Ohm => "Ω",
            WmoUnitCode::Pascal => "Pa",
            WmoUnitCode::PascalsPerSecond => "Pa s^-1",
            WmoUnitCode::Siemens => "S",
            WmoUnitCode::SiemensPerMetre => "S m^-1",
            WmoUnitCode::Sievert => "Sv",
            WmoUnitCode::Tesla => "T",
            WmoUnitCode::Volt => "V",
            WmoUnitCode::Watt => "W",
            WmoUnitCode::Kilowatt => "kW",
            WmoUnitCode::WattsPerMetrePerSteradian => "W m^-1 sr^-1",
            WmoUnitCode::WattsPerSquareMetre => "W m^-2",
            WmoUnitCode::WattsPerSquareMetrePerSteradian => "W m^-2 sr^-1",
            WmoUnitCode::WattsPerSquareMetrePerSteradianCentimetre => "W m^-2 sr^-1 cm",
            WmoUnitCode::WattsPerSquareMetrePerSteradianMetre => "W m^-2 sr^-1 m",
            WmoUnitCode::WattsPerCubicMetrePerSteradian => "W m^-3 sr^-1",
            WmoUnitCode::Weber => "Wb",
            WmoUnitCode::Year => "a",
            WmoUnitCode::CentibarsPer12Hours => "cb/12 h",
            WmoUnitCode::CentibarsPerSecond => "cb s^-1",
            WmoUnitCode::Candela => "cd",
            WmoUnitCode::Centimetre => "cm",
            WmoUnitCode::CentimetresPerHour => "cm h^-1",
            WmoUnitCode::CentimetresPerSecond => "cm s^-1",
            WmoUnitCode::Day => "d",
            WmoUnitCode::Decibel6 => "dB",
            WmoUnitCode::DecibelsPerDegree => "dB degree^-1",
            WmoUnitCode::DecibelsPerMetre => "dB m^-1",
            WmoUnitCode::DecipascalsPerSecondMicrobarPerSecond => "dPa s^-1",
            WmoUnitCode::Dekapascal => "daPa",
            WmoUnitCode::SquareDegrees => "degrees^2",
            WmoUnitCode::DegreesCelsius8 => "˚C",
            WmoUnitCode::DegreesPerSecond => "degree/s",
            WmoUnitCode::DegreeAngle => "˚",
            WmoUnitCode::DegreesTrue => "˚",
            WmoUnitCode::Decimetre => "dm",
            WmoUnitCode::ElectronVolt => "eV",
            WmoUnitCode::Foot => "ft",
            WmoUnitCode::AccelerationDueToGravity => "g",
            WmoUnitCode::GramsPerKilogram => "g kg^-1",
            WmoUnitCode::GramsPerKilogramPerSecond => "g kg^-1 s^-1",
            WmoUnitCode::GeopotentialMetre => "gpm",
            WmoUnitCode::Hour => "h",
            WmoUnitCode::Hectopascal => "hPa",
            WmoUnitCode::HectopascalsPer3Hours => "hPa/3 h",
            WmoUnitCode::HectopascalsPerHour => "hPa h^-1",
            WmoUnitCode::HectopascalsPerSecond => "hPa s^-1",
            WmoUnitCode::Hectare => "ha",
            WmoUnitCode::Kilopascal => "kPa",
            WmoUnitCode::Kilogram => "kg",
            WmoUnitCode::PerSquareKilogramPerSecond => "kg^-2 s^-1",
            WmoUnitCode::KilogramsPerKilogram => "kg kg^-1",
            WmoUnitCode::KilogramsPerKilogramPerSecond => "kg kg^-1 s^-1",
            WmoUnitCode::KilogramsPerMetre => "km m^-1",
            WmoUnitCode::KilogramsPerSquareMetre => "kg m^-2",
            WmoUnitCode::KilogramsPerSquareMetrePerSecond => "kg m^-2 s^-1",
            WmoUnitCode::KilogramsPerCubicMetre => "kg m^-3",
            WmoUnitCode::Kilometre => "km",
            WmoUnitCode::KilometresPerDay => "km/d",
            WmoUnitCode::KilometresPerHour => "km h^-1",
            WmoUnitCode::Knot => "kt",
            WmoUnitCode::KnotsPer1000Metres => "kt/1000 m",
            WmoUnitCode::Litre => "l",
            WmoUnitCode::Lumen => "lm",
            WmoUnitCode::LogarithmPerMetre => "log (m^-1)",
            WmoUnitCode::LogarithmPerSquareMetre => "log (m^-2)",
            WmoUnitCode::Lux => "lx",
            WmoUnitCode::Metre => "m",
            WmoUnitCode::PerMetre => "m^-1",
            WmoUnitCode::SquareMetres => "m^2",
            WmoUnitCode::MetresToTheTwoThirdsPowerPerSecond => "m^2/3 s^-1",
            WmoUnitCode::SquareMetresPerHertz => "m^2 Hz^-1",
            WmoUnitCode::SquareMetresPerRadianSquared => "m^2 rad^-1 s",
            WmoUnitCode::SquareMetresSecond => "m^2 s",
            WmoUnitCode::SquareMetresPerSecond => "m^2 s^-1",
            WmoUnitCode::SquareMetresPerSecondSquared => "m^2 s^-2",
            WmoUnitCode::CubicMetres => "m^3",
            WmoUnitCode::CubicMetresPerCubicMetre => "m^3 m^-3",
            WmoUnitCode::CubicMetresPerSecond => "m^3 s^-1",
            WmoUnitCode::MetresToTheFourthPower => "m^4",
            WmoUnitCode::Millisievert => "mSv",
            WmoUnitCode::MetresPerSecond => "m s^-1",
            WmoUnitCode::MetresPerSecondPer1000Metres => "m s^-1/1000 m",
            WmoUnitCode::MetresPerSecondPerMetre => "m s^-1/m",
            WmoUnitCode::MetresPerSecondSquared => "m s^-2",
            WmoUnitCode::MinuteTime => "min",
            WmoUnitCode::Millimetre => "mm",
            WmoUnitCode::MillimetresPerTheSixthPowerPerCubicMetre => "mm^6 m^-3",
            WmoUnitCode::MillimetresPerHour => "mm h^-1",
            WmoUnitCode::MillimetresPerSeconds => "mm s^-1",
            WmoUnitCode::Mole => "mol",
            WmoUnitCode::MolesPerMole => " mol mol^-1",
            WmoUnitCode::Month => "mon",
            WmoUnitCode::NauticalMile => " ",
            WmoUnitCode::NanobarHpa106 => "nbar",
            WmoUnitCode::EighthsOfCloud => "okta",
            WmoUnitCode::PhUnit => "pH unit",
            WmoUnitCode::Parsec => "pc",
            WmoUnitCode::PerCent => "%",
            WmoUnitCode::Radian => "rad",
            WmoUnitCode::RadiansPerMetre => "rad m^-1",
            WmoUnitCode::Second => "s",
            WmoUnitCode::PerSecondSameAsHertz => "s^-1",
            WmoUnitCode::PerSecondSquared => "s^-2",
            WmoUnitCode::SecondsPerMetre => "s m^-1",
            WmoUnitCode::Steradian => "sr",
            WmoUnitCode::Tonne => "t",
            WmoUnitCode::AtomicMassUnit => "u",
            WmoUnitCode::Week => "wks",
        }
    }
}
