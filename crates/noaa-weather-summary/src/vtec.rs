//! Decoding the P-VTEC codes NOAA writes hazards in.
//!
//! A `/gridpoints` hazard period names its hazard as a phenomenon code and a
//! significance code — `TO` and `W` for a tornado warning — and nothing in
//! the response spells either of them out. These two tables do.
//!
//! The vocabulary is NWS Instruction 10-1703, *Valid Time Event Code (VTEC)*,
//! Appendix A: §5.2 lists the phenomena and §6 the significances. Two codes
//! that postdate that revision are included as well, `XH` and `CW`, from the
//! Hazard Simplification Project that replaced the excessive-heat and
//! wind-chill products — `CW` in October 2024, `XH` in March 2025; both
//! appear in NOAA's live `/alerts/types` list as "Extreme Heat Warning" and
//! "Cold Weather Advisory".
//!
//! Both functions return `None` for a code they do not know, which is the
//! honest answer: a caller should print the raw code rather than a guess, so
//! a phenomenon NWS adds tomorrow reads as `XX` instead of as nothing.

/// The name of a P-VTEC phenomenon code, such as `TO` for a tornado.
///
/// ```
/// use noaa_weather_summary::vtec;
///
/// assert_eq!(vtec::phenomenon("TO"), Some("Tornado"));
/// assert_eq!(vtec::phenomenon("ZY"), Some("Freezing Spray"));
/// assert_eq!(vtec::phenomenon("QQ"), None);
/// ```
#[must_use]
pub fn phenomenon(code: &str) -> Option<&'static str> {
    // Codes outside NWSI 10-1703 Appendix A §5.2 are deliberately absent, not
    // overlooked. Third-party VTEC tables carry a handful more — IP, TI, HS,
    // SB, SN, SW, RB, SI — that the specification does not define, some with
    // names that read as truncated. Falling through to the raw code is honest;
    // a plausible expansion is not. Add one only with a source that names it.
    Some(match code {
        "AF" => "Ashfall (land)",
        "AS" => "Air Stagnation",
        "BH" => "Beach Hazard",
        "BW" => "Brisk Wind",
        "BZ" => "Blizzard",
        "CF" => "Coastal Flood",
        "CW" => "Cold Weather",
        "DF" => "Debris Flow",
        "DS" => "Dust Storm",
        "DU" => "Blowing Dust",
        "EC" => "Extreme Cold",
        "EH" => "Excessive Heat",
        "EW" => "Extreme Wind",
        "FA" => "Flood",
        "FF" => "Flash Flood",
        "FG" => "Dense Fog (land)",
        "FL" => "Flood (Forecast Points)",
        "FR" => "Frost",
        "FW" => "Fire Weather",
        "FZ" => "Freeze",
        "GL" => "Gale",
        "HF" => "Hurricane Force Wind",
        "HT" => "Heat",
        "HU" => "Hurricane",
        "HW" => "High Wind",
        "HY" => "Hydrologic",
        "HZ" => "Hard Freeze",
        "IS" => "Ice Storm",
        "LE" => "Lake Effect Snow",
        "LO" => "Low Water",
        "LS" => "Lakeshore Flood",
        "LW" => "Lake Wind",
        "MA" => "Marine",
        "MF" => "Dense Fog (marine)",
        "MH" => "Ashfall (marine)",
        "MS" => "Dense Smoke (marine)",
        "RP" => "Rip Current Risk",
        "SC" => "Small Craft",
        "SE" => "Hazardous Seas",
        "SM" => "Dense Smoke (land)",
        "SQ" => "Snow Squall",
        "SR" => "Storm",
        "SS" => "Storm Surge",
        "SU" => "High Surf",
        "SV" => "Severe Thunderstorm",
        "TO" => "Tornado",
        "TR" => "Tropical Storm",
        "TS" => "Tsunami",
        "TY" => "Typhoon",
        "UP" => "Heavy Freezing Spray",
        "WC" => "Wind Chill",
        "WI" => "Wind",
        "WS" => "Winter Storm",
        "WW" => "Winter Weather",
        "XH" => "Extreme Heat",
        "ZF" => "Freezing Fog",
        "ZR" => "Freezing Rain",
        "ZY" => "Freezing Spray",
        _ => return None,
    })
}

/// The name of a P-VTEC significance code: how firmly the office is saying it.
///
/// ```
/// use noaa_weather_summary::vtec;
///
/// assert_eq!(vtec::significance("W"), Some("Warning"));
/// assert_eq!(vtec::significance("Q"), None);
/// ```
#[must_use]
pub fn significance(code: &str) -> Option<&'static str> {
    Some(match code {
        "W" => "Warning",
        "A" => "Watch",
        "Y" => "Advisory",
        "S" => "Statement",
        "F" => "Forecast",
        "O" => "Outlook",
        "N" => "Synopsis",
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_phenomena_decode() {
        assert_eq!(phenomenon("TO"), Some("Tornado"));
        assert_eq!(phenomenon("SV"), Some("Severe Thunderstorm"));
        assert_eq!(phenomenon("FF"), Some("Flash Flood"));
        assert_eq!(phenomenon("HU"), Some("Hurricane"));
        assert_eq!(phenomenon("XH"), Some("Extreme Heat"));
        assert_eq!(phenomenon("CW"), Some("Cold Weather"));
    }

    #[test]
    fn unknown_phenomena_and_significances_are_none() {
        assert_eq!(phenomenon("QQ"), None);
        assert_eq!(phenomenon(""), None);
        assert_eq!(phenomenon("to"), None, "codes are upper case on the wire");
        assert_eq!(significance("Z"), None);
        assert_eq!(significance(""), None);
        assert_eq!(significance("WW"), None);
    }

    #[test]
    fn every_significance_in_the_specification_decodes() {
        for (code, name) in [
            ("W", "Warning"),
            ("A", "Watch"),
            ("Y", "Advisory"),
            ("S", "Statement"),
            ("F", "Forecast"),
            ("O", "Outlook"),
            ("N", "Synopsis"),
        ] {
            assert_eq!(significance(code), Some(name), "{code}");
        }
    }

    /// Appendix A §5.2 lists 56 phenomena; `XH` and `CW` were added after it.
    #[test]
    fn the_phenomenon_table_is_the_whole_specification() {
        let mut known = Vec::new();
        for first in 'A'..='Z' {
            for second in 'A'..='Z' {
                let code = format!("{first}{second}");
                if phenomenon(&code).is_some() {
                    known.push(code);
                }
            }
        }
        assert_eq!(known.len(), 58, "{known:?}");
    }
}
