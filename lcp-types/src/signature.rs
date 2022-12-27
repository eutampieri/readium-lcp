use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Signature {
    algorithm: String,
    certificate: String,
    value: String,
}
