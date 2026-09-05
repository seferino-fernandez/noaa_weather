use serde::{Deserialize, Serialize};

/// Classifies an observation value using the MADIS quality-control hierarchy.
///
/// See the [MADIS surface observation quality-control notes][madis] for the
/// checks represented by each flag.
///
/// [madis]: https://madis.ncep.noaa.gov/madis_sfc_qc_notes.shtml
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
pub enum QualityControl {
    /// Marks preliminary data on which no quality-control checks have run.
    #[serde(rename = "Z")]
    Z,
    /// Marks data that passed the level-one coarse checks.
    #[serde(rename = "C")]
    C,
    /// Marks screened data that passed quality-control levels one and two.
    #[serde(rename = "S")]
    S,
    /// Marks verified data that passed quality-control levels one through three.
    #[serde(rename = "V")]
    V,
    /// Marks rejected or erroneous data that failed a level-one check.
    #[serde(rename = "X")]
    X,
    /// Marks questioned data that passed level one but failed level two or three.
    #[serde(rename = "Q")]
    Q,
    /// Marks data judged subjectively good.
    #[serde(rename = "G")]
    G,
    /// Marks data judged subjectively bad.
    #[serde(rename = "B")]
    B,
    /// Marks a fallback air temperature used when virtual temperature could not be calculated.
    #[serde(rename = "T")]
    T,
}

impl QualityControl {
    /// Returns the MADIS description associated with this flag.
    pub fn descriptor_value(&self) -> &str {
        match self {
            QualityControl::Z => "Preliminary, no QC",
            QualityControl::C => "Coarse pass, passed level 1",
            QualityControl::S => "Screened, passed levels 1 and 2",
            QualityControl::V => "Verified, passed levels 1, 2, and 3",
            QualityControl::X => "Rejected/erroneous, failed level 1",
            QualityControl::Q => "Questioned, passed level 1, failed 2 or 3",
            QualityControl::G => "Subjective good",
            QualityControl::B => "Subjective bad",
            QualityControl::T => {
                "Virtual temperature could not be calculated, air temperature passing all QC
       checks has been returned"
            }
        }
    }
}
