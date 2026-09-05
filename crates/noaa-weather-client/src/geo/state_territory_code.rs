use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Identifies a United States state, district, territory, or associated state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
pub enum StateTerritoryCode {
    /// Identifies Alabama (`AL`).
    #[serde(rename = "AL")]
    Al,
    /// Identifies Alaska (`AK`).
    #[serde(rename = "AK")]
    Ak,
    /// Identifies American Samoa (`AS`).
    #[serde(rename = "AS")]
    As,
    /// Identifies Arkansas (`AR`).
    #[serde(rename = "AR")]
    Ar,
    /// Identifies Arizona (`AZ`).
    #[serde(rename = "AZ")]
    Az,
    /// Identifies California (`CA`).
    #[serde(rename = "CA")]
    Ca,
    /// Identifies Colorado (`CO`).
    #[serde(rename = "CO")]
    Co,
    /// Identifies Connecticut (`CT`).
    #[serde(rename = "CT")]
    Ct,
    /// Identifies Delaware (`DE`).
    #[serde(rename = "DE")]
    De,
    /// Identifies the District of Columbia (`DC`).
    #[serde(rename = "DC")]
    Dc,
    /// Identifies Florida (`FL`).
    #[serde(rename = "FL")]
    Fl,
    /// Identifies Georgia (`GA`).
    #[serde(rename = "GA")]
    Ga,
    /// Identifies Guam (`GU`).
    #[serde(rename = "GU")]
    Gu,
    /// Identifies Hawaii (`HI`).
    #[serde(rename = "HI")]
    Hi,
    /// Identifies Idaho (`ID`).
    #[serde(rename = "ID")]
    Id,
    /// Identifies Illinois (`IL`).
    #[serde(rename = "IL")]
    Il,
    /// Identifies Indiana (`IN`).
    #[serde(rename = "IN")]
    In,
    /// Identifies Iowa (`IA`).
    #[serde(rename = "IA")]
    Ia,
    /// Identifies Kansas (`KS`).
    #[serde(rename = "KS")]
    Ks,
    /// Identifies Kentucky (`KY`).
    #[serde(rename = "KY")]
    Ky,
    /// Identifies Louisiana (`LA`).
    #[serde(rename = "LA")]
    La,
    /// Identifies Maine (`ME`).
    #[serde(rename = "ME")]
    Me,
    /// Identifies Maryland (`MD`).
    #[serde(rename = "MD")]
    Md,
    /// Identifies Massachusetts (`MA`).
    #[serde(rename = "MA")]
    Ma,
    /// Identifies Michigan (`MI`).
    #[serde(rename = "MI")]
    Mi,
    /// Identifies Minnesota (`MN`).
    #[serde(rename = "MN")]
    Mn,
    /// Identifies Mississippi (`MS`).
    #[serde(rename = "MS")]
    Ms,
    /// Identifies Missouri (`MO`).
    #[serde(rename = "MO")]
    Mo,
    /// Identifies Montana (`MT`).
    #[serde(rename = "MT")]
    Mt,
    /// Identifies Nebraska (`NE`).
    #[serde(rename = "NE")]
    Ne,
    /// Identifies Nevada (`NV`).
    #[serde(rename = "NV")]
    Nv,
    /// Identifies New Hampshire (`NH`).
    #[serde(rename = "NH")]
    Nh,
    /// Identifies New Jersey (`NJ`).
    #[serde(rename = "NJ")]
    Nj,
    /// Identifies New Mexico (`NM`).
    #[serde(rename = "NM")]
    Nm,
    /// Identifies New York (`NY`).
    #[serde(rename = "NY")]
    Ny,
    /// Identifies North Carolina (`NC`).
    #[serde(rename = "NC")]
    Nc,
    /// Identifies North Dakota (`ND`).
    #[serde(rename = "ND")]
    Nd,
    /// Identifies Ohio (`OH`).
    #[serde(rename = "OH")]
    Oh,
    /// Identifies Oklahoma (`OK`).
    #[serde(rename = "OK")]
    Ok,
    /// Identifies Oregon (`OR`).
    #[serde(rename = "OR")]
    Or,
    /// Identifies Pennsylvania (`PA`).
    #[serde(rename = "PA")]
    Pa,
    /// Identifies Puerto Rico (`PR`).
    #[serde(rename = "PR")]
    Pr,
    /// Identifies Rhode Island (`RI`).
    #[serde(rename = "RI")]
    Ri,
    /// Identifies South Carolina (`SC`).
    #[serde(rename = "SC")]
    Sc,
    /// Identifies South Dakota (`SD`).
    #[serde(rename = "SD")]
    Sd,
    /// Identifies Tennessee (`TN`).
    #[serde(rename = "TN")]
    Tn,
    /// Identifies Texas (`TX`).
    #[serde(rename = "TX")]
    Tx,
    /// Identifies Utah (`UT`).
    #[serde(rename = "UT")]
    Ut,
    /// Identifies Vermont (`VT`).
    #[serde(rename = "VT")]
    Vt,
    /// Identifies the United States Virgin Islands (`VI`).
    #[serde(rename = "VI")]
    Vi,
    /// Identifies Virginia (`VA`).
    #[serde(rename = "VA")]
    Va,
    /// Identifies Washington (`WA`).
    #[serde(rename = "WA")]
    Wa,
    /// Identifies West Virginia (`WV`).
    #[serde(rename = "WV")]
    Wv,
    /// Identifies Wisconsin (`WI`).
    #[serde(rename = "WI")]
    Wi,
    /// Identifies Wyoming (`WY`).
    #[serde(rename = "WY")]
    Wy,
    /// Identifies the Northern Mariana Islands (`MP`).
    #[serde(rename = "MP")]
    Mp,
    /// Identifies Palau (`PW`).
    #[serde(rename = "PW")]
    Pw,
    /// Identifies the Federated States of Micronesia (`FM`).
    #[serde(rename = "FM")]
    Fm,
    /// Identifies the Marshall Islands (`MH`).
    #[serde(rename = "MH")]
    Mh,
}

impl Display for StateTerritoryCode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Al => write!(f, "AL"),
            Self::Ak => write!(f, "AK"),
            Self::As => write!(f, "AS"),
            Self::Ar => write!(f, "AR"),
            Self::Az => write!(f, "AZ"),
            Self::Ca => write!(f, "CA"),
            Self::Co => write!(f, "CO"),
            Self::Ct => write!(f, "CT"),
            Self::De => write!(f, "DE"),
            Self::Dc => write!(f, "DC"),
            Self::Fl => write!(f, "FL"),
            Self::Ga => write!(f, "GA"),
            Self::Gu => write!(f, "GU"),
            Self::Hi => write!(f, "HI"),
            Self::Id => write!(f, "ID"),
            Self::Il => write!(f, "IL"),
            Self::In => write!(f, "IN"),
            Self::Ia => write!(f, "IA"),
            Self::Ks => write!(f, "KS"),
            Self::Ky => write!(f, "KY"),
            Self::La => write!(f, "LA"),
            Self::Me => write!(f, "ME"),
            Self::Md => write!(f, "MD"),
            Self::Ma => write!(f, "MA"),
            Self::Mi => write!(f, "MI"),
            Self::Mn => write!(f, "MN"),
            Self::Ms => write!(f, "MS"),
            Self::Mo => write!(f, "MO"),
            Self::Mt => write!(f, "MT"),
            Self::Ne => write!(f, "NE"),
            Self::Nv => write!(f, "NV"),
            Self::Nh => write!(f, "NH"),
            Self::Nj => write!(f, "NJ"),
            Self::Nm => write!(f, "NM"),
            Self::Ny => write!(f, "NY"),
            Self::Nc => write!(f, "NC"),
            Self::Nd => write!(f, "ND"),
            Self::Oh => write!(f, "OH"),
            Self::Ok => write!(f, "OK"),
            Self::Or => write!(f, "OR"),
            Self::Pa => write!(f, "PA"),
            Self::Pr => write!(f, "PR"),
            Self::Ri => write!(f, "RI"),
            Self::Sc => write!(f, "SC"),
            Self::Sd => write!(f, "SD"),
            Self::Tn => write!(f, "TN"),
            Self::Tx => write!(f, "TX"),
            Self::Ut => write!(f, "UT"),
            Self::Vt => write!(f, "VT"),
            Self::Vi => write!(f, "VI"),
            Self::Va => write!(f, "VA"),
            Self::Wa => write!(f, "WA"),
            Self::Wv => write!(f, "WV"),
            Self::Wi => write!(f, "WI"),
            Self::Wy => write!(f, "WY"),
            Self::Mp => write!(f, "MP"),
            Self::Pw => write!(f, "PW"),
            Self::Fm => write!(f, "FM"),
            Self::Mh => write!(f, "MH"),
        }
    }
}

impl FromStr for StateTerritoryCode {
    type Err = String;

    fn from_str(state_territory_code: &str) -> Result<Self, Self::Err> {
        match state_territory_code.to_uppercase().as_str() {
            "AL" => Ok(StateTerritoryCode::Al),
            "AK" => Ok(StateTerritoryCode::Ak),
            "AS" => Ok(StateTerritoryCode::As),
            "AR" => Ok(StateTerritoryCode::Ar),
            "AZ" => Ok(StateTerritoryCode::Az),
            "CA" => Ok(StateTerritoryCode::Ca),
            "CO" => Ok(StateTerritoryCode::Co),
            "CT" => Ok(StateTerritoryCode::Ct),
            "DE" => Ok(StateTerritoryCode::De),
            "DC" => Ok(StateTerritoryCode::Dc),
            "FL" => Ok(StateTerritoryCode::Fl),
            "GA" => Ok(StateTerritoryCode::Ga),
            "GU" => Ok(StateTerritoryCode::Gu),
            "HI" => Ok(StateTerritoryCode::Hi),
            "ID" => Ok(StateTerritoryCode::Id),
            "IL" => Ok(StateTerritoryCode::Il),
            "IN" => Ok(StateTerritoryCode::In),
            "IA" => Ok(StateTerritoryCode::Ia),
            "KS" => Ok(StateTerritoryCode::Ks),
            "KY" => Ok(StateTerritoryCode::Ky),
            "LA" => Ok(StateTerritoryCode::La),
            "ME" => Ok(StateTerritoryCode::Me),
            "MD" => Ok(StateTerritoryCode::Md),
            "MA" => Ok(StateTerritoryCode::Ma),
            "MI" => Ok(StateTerritoryCode::Mi),
            "MN" => Ok(StateTerritoryCode::Mn),
            "MS" => Ok(StateTerritoryCode::Ms),
            "MO" => Ok(StateTerritoryCode::Mo),
            "MT" => Ok(StateTerritoryCode::Mt),
            "NE" => Ok(StateTerritoryCode::Ne),
            "NV" => Ok(StateTerritoryCode::Nv),
            "NH" => Ok(StateTerritoryCode::Nh),
            "NJ" => Ok(StateTerritoryCode::Nj),
            "NM" => Ok(StateTerritoryCode::Nm),
            "NY" => Ok(StateTerritoryCode::Ny),
            "NC" => Ok(StateTerritoryCode::Nc),
            "ND" => Ok(StateTerritoryCode::Nd),
            "OH" => Ok(StateTerritoryCode::Oh),
            "OK" => Ok(StateTerritoryCode::Ok),
            "OR" => Ok(StateTerritoryCode::Or),
            "PA" => Ok(StateTerritoryCode::Pa),
            "PR" => Ok(StateTerritoryCode::Pr),
            "RI" => Ok(StateTerritoryCode::Ri),
            "SC" => Ok(StateTerritoryCode::Sc),
            "SD" => Ok(StateTerritoryCode::Sd),
            "TN" => Ok(StateTerritoryCode::Tn),
            "TX" => Ok(StateTerritoryCode::Tx),
            "UT" => Ok(StateTerritoryCode::Ut),
            "VT" => Ok(StateTerritoryCode::Vt),
            "VI" => Ok(StateTerritoryCode::Vi),
            "VA" => Ok(StateTerritoryCode::Va),
            "WA" => Ok(StateTerritoryCode::Wa),
            "WV" => Ok(StateTerritoryCode::Wv),
            "WI" => Ok(StateTerritoryCode::Wi),
            "WY" => Ok(StateTerritoryCode::Wy),
            "MP" => Ok(StateTerritoryCode::Mp),
            "PW" => Ok(StateTerritoryCode::Pw),
            "FM" => Ok(StateTerritoryCode::Fm),
            "MH" => Ok(StateTerritoryCode::Mh),
            _ => Err(format!(
                "Invalid state territory code: {state_territory_code}"
            )),
        }
    }
}
