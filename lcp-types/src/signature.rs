use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Signature {
    algorithm: String,
    certificate: String,
    value: String,
}
