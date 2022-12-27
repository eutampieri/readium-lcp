use serde::Deserialize;

use crate::crypto::Algorithm;

#[derive(Deserialize, Debug)]
#[serde(rename = "encryption")]
struct Manifest {
    #[serde(rename = "$value")]
    items: Vec<EncryptedData>,
}

#[derive(Deserialize, Debug)]
struct EncryptedData {
    #[serde(rename = "EncryptionMethod")]
    method: EncryptionMethod,
    #[serde(rename = "CipherData")]
    data: CipherData,
    #[serde(rename = "EncryptionProperties")]
    props: EncryptionProperties,
}

#[derive(Deserialize, Debug)]
struct EncryptionMethod {
    #[serde(rename = "Algorithm")]
    algorithm: String,
}
#[derive(Deserialize, Debug)]
struct CipherData {
    #[serde(rename = "CipherReference")]
    reference: CipherReference,
}
#[derive(Deserialize, Debug)]
struct CipherReference {
    #[serde(rename = "URI")]
    uri: String,
}

#[derive(Deserialize, Debug)]
struct EncryptionProperties {
    #[serde(rename = "EncryptionProperty")]
    prop: EncryptionProperty,
}
#[derive(Deserialize, Debug)]
struct EncryptionProperty {
    #[serde(rename = "Compression")]
    compression: Compression,
}
#[derive(Deserialize, Debug)]
struct Compression {
    #[serde(rename = "Method")]
    method: u8,
    #[serde(rename = "OriginalLength")]
    original_length: usize,
}

pub fn parse_manifest(
    manifest: &str,
) -> std::collections::HashMap<String, (Algorithm, Option<usize>)> {
    let manifest: Manifest = serde_xml_rs::from_str(manifest).unwrap();
    manifest
        .items
        .iter()
        .map(|x| {
            (
                x.data.reference.uri.clone(),
                (
                    Algorithm::try_from(x.method.algorithm.as_str()).unwrap(),
                    if x.props.prop.compression.method == 8 {
                        Some(x.props.prop.compression.original_length)
                    } else {
                        None
                    },
                ),
            )
        })
        .collect()
}
