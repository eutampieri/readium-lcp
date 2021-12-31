use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Rights {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<chrono::DateTime<chrono::Utc>>,
}
