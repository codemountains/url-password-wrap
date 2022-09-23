mod password;
mod redirect_url;

use crate::model::wrap::password::HashedPassword;
use crate::model::wrap::redirect_url::{DecryptedRedirectUrl, EncryptedRedirectUrl};
use chrono::{TimeZone, Utc};
use mongodb::bson::Timestamp;
use serde::{Deserialize, Serialize};
use url_wrap_kernel::model::wrap::{NewWrap, Wrap};

#[derive(Debug, Deserialize, Serialize)]
pub struct WrapDocument {
    #[serde(rename = "_id")]
    pub id: String,
    pub redirect_url: String,
    pub password: String,
    pub auth_type: String,
    pub comment: String,
    pub expiration_at: Timestamp,
    pub created_at: Timestamp,
}

impl WrapDocument {
    pub fn verify_password(&self, password: &str) -> anyhow::Result<()> {
        let hashed_password = HashedPassword::new(&self.password);
        hashed_password.verify(password)
    }
}

impl TryFrom<WrapDocument> for Wrap {
    type Error = anyhow::Error;

    fn try_from(wd: WrapDocument) -> Result<Self, Self::Error> {
        let decrypted_redirect_url: DecryptedRedirectUrl = wd.redirect_url.try_into()?;

        let expiration_at = Utc.timestamp(wd.expiration_at.time as i64, wd.expiration_at.increment);
        let created_at = Utc.timestamp(wd.created_at.time as i64, wd.created_at.increment);

        Ok(Wrap {
            id: wd.id.try_into()?,
            redirect_url: decrypted_redirect_url.to_string(),
            password: wd.password.into(),
            auth_type: wd.auth_type.into(),
            comment: wd.comment,
            expiration_at,
            created_at,
        })
    }
}

impl TryFrom<NewWrap> for WrapDocument {
    type Error = anyhow::Error;

    fn try_from(nw: NewWrap) -> Result<Self, Self::Error> {
        let encrypted_redirect_url: EncryptedRedirectUrl = nw.redirect_url.try_into()?;
        let hashed_password: HashedPassword = nw.password.try_into()?;

        Ok(WrapDocument {
            id: nw.id.value.to_string(),
            redirect_url: encrypted_redirect_url.to_string(),
            password: hashed_password.to_string(),
            auth_type: nw.auth_type.to_string(),
            comment: nw.comment,
            expiration_at: Timestamp {
                time: nw.expiration_at.timestamp() as u32,
                increment: 0u32,
            },
            created_at: Timestamp {
                time: Utc::now().timestamp() as u32,
                increment: 0u32,
            },
        })
    }
}
