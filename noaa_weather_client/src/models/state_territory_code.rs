use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StateTerritoryCode {
    #[serde(rename = "AL")]
    Al,
    #[serde(rename = "AK")]
    Ak,
    #[serde(rename = "AS")]
    As,
    #[serde(rename = "AR")]
    Ar,
    #[serde(rename = "AZ")]
    Az,
    #[serde(rename = "CA")]
    Ca,
    #[serde(rename = "CO")]
    Co,
    #[serde(rename = "CT")]
    Ct,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "DC")]
    Dc,
    #[serde(rename = "FL")]
    Fl,
    #[serde(rename = "GA")]
    Ga,
    #[serde(rename = "GU")]
    Gu,
    #[serde(rename = "HI")]
    Hi,
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "IL")]
    Il,
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "IA")]
    Ia,
    #[serde(rename = "KS")]
    Ks,
    #[serde(rename = "KY")]
    Ky,
    #[serde(rename = "LA")]
    La,
    #[serde(rename = "ME")]
    Me,
    #[serde(rename = "MD")]
    Md,
    #[serde(rename = "MA")]
    Ma,
    #[serde(rename = "MI")]
    Mi,
    #[serde(rename = "MN")]
    Mn,
    #[serde(rename = "MS")]
    Ms,
    #[serde(rename = "MO")]
    Mo,
    #[serde(rename = "MT")]
    Mt,
    #[serde(rename = "NE")]
    Ne,
    #[serde(rename = "NV")]
    Nv,
    #[serde(rename = "NH")]
    Nh,
    #[serde(rename = "NJ")]
    Nj,
    #[serde(rename = "NM")]
    Nm,
    #[serde(rename = "NY")]
    Ny,
    #[serde(rename = "NC")]
    Nc,
    #[serde(rename = "ND")]
    Nd,
    #[serde(rename = "OH")]
    Oh,
    #[serde(rename = "OK")]
    Ok,
    #[serde(rename = "OR")]
    Or,
    #[serde(rename = "PA")]
    Pa,
    #[serde(rename = "PR")]
    Pr,
    #[serde(rename = "RI")]
    Ri,
    #[serde(rename = "SC")]
    Sc,
    #[serde(rename = "SD")]
    Sd,
    #[serde(rename = "TN")]
    Tn,
    #[serde(rename = "TX")]
    Tx,
    #[serde(rename = "UT")]
    Ut,
    #[serde(rename = "VT")]
    Vt,
    #[serde(rename = "VI")]
    Vi,
    #[serde(rename = "VA")]
    Va,
    #[serde(rename = "WA")]
    Wa,
    #[serde(rename = "WV")]
    Wv,
    #[serde(rename = "WI")]
    Wi,
    #[serde(rename = "WY")]
    Wy,
    #[serde(rename = "MP")]
    Mp,
    #[serde(rename = "PW")]
    Pw,
    #[serde(rename = "FM")]
    Fm,
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
                "Invalid state territory code: {}",
                state_territory_code
            )),
        }
    }
}
