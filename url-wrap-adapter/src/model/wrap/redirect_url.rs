use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use anyhow::anyhow;
use data_encoding::HEXLOWER;
use std::env;
use std::fmt;
use std::fmt::Formatter;
use std::str;

pub struct EncryptedRedirectUrl(String);

impl fmt::Display for EncryptedRedirectUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for EncryptedRedirectUrl {
    type Error = anyhow::Error;

    fn try_from(url: String) -> Result<Self, Self::Error> {
        let encrypted = encrypt_url(url.as_bytes())?;
        Ok(EncryptedRedirectUrl(encrypted))
    }
}

pub struct DecryptedRedirectUrl(String);

impl fmt::Display for DecryptedRedirectUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for DecryptedRedirectUrl {
    type Error = anyhow::Error;

    fn try_from(url: String) -> Result<Self, Self::Error> {
        let decrypted = decrypt_url(&url)?;
        Ok(DecryptedRedirectUrl(decrypted))
    }
}

#[derive(Debug)]
struct EncryptionParameter {
    key: String,
    nonce: String,
}

impl EncryptionParameter {
    fn new(key: String, nonce: String) -> Self {
        Self { key, nonce }
    }
}

fn encrypt_url(url: &[u8]) -> anyhow::Result<String> {
    let parameter = init_encryption_parameter();
    let k = parameter.key.as_bytes();
    let n = parameter.nonce.as_bytes();

    let key = GenericArray::from_slice(k);
    let nonce = Nonce::from_slice(n);

    // encryption
    let cipher = Aes256Gcm::new(key);
    let encrypted_url = cipher
        .encrypt(nonce, url.as_ref())
        .map_err(|e| anyhow!(e))?;

    Ok(HEXLOWER.encode(&encrypted_url))
}

pub fn decrypt_url(encrypted_url: &str) -> anyhow::Result<String> {
    let cipher_url = HEXLOWER
        .decode(encrypted_url.as_ref())
        .map_err(|e| anyhow!(e))?;

    let parameter = init_encryption_parameter();
    let k = parameter.key.as_bytes();
    let n = parameter.nonce.as_bytes();

    let key = GenericArray::from_slice(k);
    let nonce = Nonce::from_slice(n);

    // decryption
    let cipher = Aes256Gcm::new(key);
    let decrypted_url = cipher
        .decrypt(nonce, cipher_url.as_slice())
        .map_err(|e| anyhow!(e))?;

    Ok(str::from_utf8(&decrypted_url)?.to_string())
}

fn init_encryption_parameter() -> EncryptionParameter {
    let key = env::var_os("AES_GCM_SALT")
        .expect("AES_GCM_SALT is undefined.")
        .into_string()
        .expect("AES_GCM_SALT is invalid value.");
    let nonce = env::var_os("AES_GCM_NONCE")
        .expect("AES_GCM_NONCE is undefined.")
        .into_string()
        .expect("AES_GCM_NONCE is invalid value.");

    EncryptionParameter::new(key, nonce)
}
