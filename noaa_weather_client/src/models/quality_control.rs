use serde::{Deserialize, Serialize};

/// For values in observation records, the quality control flag from the MADIS system.
/// The definitions of these flags can be found at https://madis.ncep.noaa.gov/madis_sfc_qc_notes.shtml
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum QualityControl {
    #[serde(rename = "Z")]
    Z,
    #[serde(rename = "C")]
    C,
    #[serde(rename = "S")]
    S,
    #[serde(rename = "V")]
    V,
    #[serde(rename = "X")]
    X,
    #[serde(rename = "Q")]
    Q,
    #[serde(rename = "G")]
    G,
    #[serde(rename = "B")]
    B,
    #[serde(rename = "T")]
    T,
}

impl QualityControl {
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
