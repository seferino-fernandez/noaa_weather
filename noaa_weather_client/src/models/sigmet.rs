use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sigmet {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "issueTime", skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<String>,
    #[serde(
        rename = "fir",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub fir: Option<Option<String>>,
    /// ATSU Identifier
    #[serde(rename = "atsu", skip_serializing_if = "Option::is_none")]
    pub atsu: Option<String>,
    #[serde(
        rename = "sequence",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sequence: Option<Option<String>>,
    #[serde(
        rename = "phenomenon",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub phenomenon: Option<Option<String>>,
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}

impl Sigmet {
    pub fn new() -> Sigmet {
        Sigmet {
            id: None,
            issue_time: None,
            fir: None,
            atsu: None,
            sequence: None,
            phenomenon: None,
            start: None,
            end: None,
        }
    }
}
