use serde::{Deserialize, Serialize};

mod tripledes;

pub trait Key {
    fn get_encrypted_value(&self) -> Vec<u8>;
    fn get_encryption_algorithm(&self) -> Algorithm;
    fn decrypt(&self, key: &[u8]) -> Vec<u8> {
        self.get_encryption_algorithm()
            .decrypt(key, &self.get_encrypted_value())
    }
}

pub enum Algorithm {
    TripleDES,
    AES128,
    AES256,
    AES128GCM,
    AES192,
    AES192GCM,
    AES256GCM,
}
impl TryFrom<&str> for Algorithm {
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

impl Algorithm {
    pub fn decrypt(&self, key: &[u8], ciphertext: &[u8]) -> Vec<u8> {
        match self {
            Algorithm::TripleDES => todo!(),
            Algorithm::AES128 => todo!(),
            Algorithm::AES256 => {
                use aes::Aes256;
                use block_modes::block_padding::ZeroPadding;
                use block_modes::{BlockMode, Cbc};
                type Aes256Cbc = Cbc<Aes256, ZeroPadding>;
                let iv = &ciphertext[..16];
                let ciphertext = &ciphertext[16..];
                let cipher = Aes256Cbc::new_from_slices(key, &iv).unwrap();
                cipher.decrypt_vec(ciphertext).unwrap()
            }
            Algorithm::AES128GCM => todo!(),
            Algorithm::AES192 => todo!(),
            Algorithm::AES192GCM => todo!(),
            Algorithm::AES256GCM => todo!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Encryption {
    pub profile: String,
    /// The Content Key (encrypted using the User Key) is used to encrypt the Publication Resources
    pub content_key: ContentKey,
    pub user_key: UserKey,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentKey {
    encrypted_value: String,
    algorithm: String,
}

impl Key for ContentKey {
    fn get_encrypted_value(&self) -> Vec<u8> {
        base64::decode(&self.encrypted_value).unwrap()
    }

    fn get_encryption_algorithm(&self) -> Algorithm {
        Algorithm::try_from(self.algorithm.as_str()).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserKey {
    key_check: String,
    algorithm: String,
    pub text_hint: String,
}

impl Key for UserKey {
    fn get_encrypted_value(&self) -> Vec<u8> {
        base64::decode(&self.key_check).unwrap()
    }

    fn get_encryption_algorithm(&self) -> Algorithm {
        Algorithm::try_from(self.algorithm.as_str()).unwrap()
    }
}
