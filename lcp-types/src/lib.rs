use serde::{Deserialize, Serialize};

pub mod crypto;
pub mod link;

#[derive(Serialize, Deserialize)]
pub struct LicenseDocument {
    pub id: String,
    pub issued: chrono::DateTime<chrono::Utc>,
    pub provider: String,
    pub updated: Option<chrono::DateTime<chrono::Utc>>,
}
