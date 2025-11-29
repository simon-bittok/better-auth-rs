use std::path::PathBuf;

use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};

use crate::config::ConfigResult;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RsaJwtConfig {
    pub(crate) private_key: PathBuf,
    pub(crate) public_key: PathBuf,
    pub(crate) exp: i64,
}

impl RsaJwtConfig {
    /// Reads the public key from the configured path and returns a DecodingKey.
    ///
    /// # Errors
    /// * Missing or unreadable key file
    /// * Invalid key format
    pub fn encoding_key(&self) -> ConfigResult<EncodingKey> {
        let key_data = std::fs::read(&self.private_key)?;

        EncodingKey::from_rsa_pem(&key_data).map_err(Into::into)
    }

    /// Reads the public key from the configured path and returns a DecodingKey.
    ///
    /// # Errors
    /// * Missing or unreadable key file
    /// * Invalid key format
    pub fn decoding_key(&self) -> ConfigResult<DecodingKey> {
        let key_data = std::fs::read(&self.public_key)?;

        DecodingKey::from_rsa_pem(&key_data).map_err(Into::into)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthConfig {
    pub(crate) access: RsaJwtConfig,
    pub(crate) refresh: RsaJwtConfig,
}

impl AuthConfig {
    pub fn access(&self) -> &RsaJwtConfig {
        &self.access
    }

    pub fn refresh(&self) -> &RsaJwtConfig {
        &self.refresh
    }
}
