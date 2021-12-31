use serde::{Deserialize, Serialize};

pub enum LinkRelationship {
    /// Location where a Reading System can redirect a User looking for additional information about the User Passphrase. Required
    Hint,
    /// Location where the Publication associated with the License Document can be downloaded
    Publication,
    /// Originally `self`, As defined in the IANA registry of link relations: “Conveys an identifier for the link’s context.”
    Link,
    /// Support resources for the user (either a website, an email or a telephone number)
    Support,
}

impl From<&str> for LinkRelationship {
    fn from(s: &str) -> Self {
        match s {
            "hint" => Self::Hint,
            "publication" => Self::Publication,
            "self" => Self::Link,
            "support" => Self::Support,
            _ => unimplemented!(),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Link {
    pub href: String,
    rel: String,
    pub title: Option<String>,
    #[serde(rename = "type")]
    /// Originally `type`
    pub mime_type: Option<String>,
    pub templated: Option<bool>,
    pub profile: Option<String>,
    pub length: Option<usize>,
    pub hash: Option<String>,
}

impl Link {
    pub fn get_rel(&self) -> LinkRelationship {
        LinkRelationship::from(self.rel.as_str())
    }
}