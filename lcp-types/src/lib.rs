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
    pub updated: Option<chrono::DateTime<chrono::Utc>>,
}
