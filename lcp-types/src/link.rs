use serde::{Deserialize, Serialize};

#[derive(PartialEq)]
pub enum LinkRelationship {
    /// Location where a Reading System can redirect a User looking for additional information about the User Passphrase. Required
    Hint,
    /// Location where the Publication associated with the License Document can be downloaded
    Publication,
    /// Originally `self`, As defined in the IANA registry of link relations: “Conveys an identifier for the link’s context.”
    Link,
    /// Support resources for the user (either a website, an email or a telephone number)
    Support,
    Unknown(String),
}

impl From<&str> for LinkRelationship {
    fn from(s: &str) -> Self {
        match s {
            "hint" => Self::Hint,
            "publication" => Self::Publication,
            "self" => Self::Link,
            "support" => Self::Support,
            _ => Self::Unknown(s.to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Link {
    pub href: String,
    rel: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Originally `type`
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}

impl Link {
    pub fn get_rel(&self) -> LinkRelationship {
        LinkRelationship::from(self.rel.as_str())
    }
}
