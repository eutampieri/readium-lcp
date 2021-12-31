use serde::{Deserialize, Serialize};

pub mod crypto;
pub mod link;
pub mod rights;
pub mod signature;
pub mod user;

#[derive(Serialize, Deserialize)]
pub struct LicenseDocument {
    pub id: String,
    pub issued: chrono::DateTime<chrono::Utc>,
    pub provider: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<chrono::DateTime<chrono::Utc>>,
    pub encryption: crypto::Encryption,
    pub links: Vec<link::Link>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<rights::Rights>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<user::User>,
    pub signature: signature::Signature,
}
