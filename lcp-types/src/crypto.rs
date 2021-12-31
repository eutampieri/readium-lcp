use serde::{Deserialize, Serialize};

pub trait Key {
    fn get_encrypted_value(&self) -> Vec<u8>;
    fn get_encryption_algorithm(&self) -> EncryptionAlgorithm;
}

pub enum EncryptionAlgorithm {
    TripleDES,
    AES128,
    AES256,
    AES128GCM,
    AES192,
    AES192GCM,
    AES256GCM,
}
impl TryFrom<&str> for EncryptionAlgorithm {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "http://www.w3.org/2001/04/xmlenc#tripledes-cbc" => Ok(Self::TripleDES),
            "http://www.w3.org/2001/04/xmlenc#aes128-cbc" => Ok(Self::AES128),
            "http://www.w3.org/2001/04/xmlenc#aes256-cbc" => Ok(Self::AES256),
            "http://www.w3.org/2009/xmlenc11#aes128-gcm" => Ok(Self::AES128GCM),
            "http://www.w3.org/2001/04/xmlenc#aes192-cbc" => Ok(Self::AES192),
            "http://www.w3.org/2001/04/xmlenc#aes192-gcm" => Ok(Self::AES192GCM),
            "http://www.w3.org/2001/04/xmlenc#aes256-gcm" => Ok(Self::AES256GCM),
            _ => Err("Invalid algorithm URI"),
        }
    }
}
/*
    ConcatKDF,
    PBKDF2,
    RSAOAEPMGFSHA1,
    RSAOAEP,
    RSA1_5,
    ECDH,

}*/

#[derive(Serialize, Deserialize)]
pub struct Encryption {
    pub profile: String,
    /// The Content Key (encrypted using the User Key) is used to encrypt the Publication Resources
    pub content_key: ContentKey,
    pub user_key: UserKey,
}

#[derive(Serialize, Deserialize)]
pub struct ContentKey {
    encrypted_value: String,
    algorithm: String,
}

impl Key for ContentKey {
    fn get_encrypted_value(&self) -> Vec<u8> {
        base64::decode(&self.encrypted_value).unwrap()
    }

    fn get_encryption_algorithm(&self) -> EncryptionAlgorithm {
        EncryptionAlgorithm::try_from(self.algorithm.as_str()).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserKey {
    key_check: String,
    algorithm: String,
    pub text_hint: String,
}

impl Key for UserKey {
    fn get_encrypted_value(&self) -> Vec<u8> {
        base64::decode(&self.key_check).unwrap()
    }

    fn get_encryption_algorithm(&self) -> EncryptionAlgorithm {
        EncryptionAlgorithm::try_from(self.algorithm.as_str()).unwrap()
    }
}
