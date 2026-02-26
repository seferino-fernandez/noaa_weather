use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AlertScope {
    #[serde(rename = "Public")]
    Public,
    #[serde(rename = "Restricted")]
    Restricted,
    #[serde(rename = "Private")]
    Private,
}

impl std::fmt::Display for AlertScope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Public => write!(f, "Public"),
            Self::Restricted => write!(f, "Restricted"),
            Self::Private => write!(f, "Private"),
        }
    }
}
