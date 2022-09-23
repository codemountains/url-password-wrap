pub mod auth_type;

use crate::model::wrap::auth_type::WrapAuthType;
use crate::model::Id;
use chrono::{DateTime, Utc};

pub struct Wrap {
    pub id: Id<Wrap>,
    pub redirect_url: String,
    pub password: PHCString,
    pub auth_type: WrapAuthType,
    pub comment: String,
    pub expiration_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl Wrap {
    pub fn new(
        id: Id<Wrap>,
        redirect_url: String,
        password: PHCString,
        auth_type: WrapAuthType,
        comment: String,
        expiration_at: DateTime<Utc>,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            redirect_url,
            password,
            auth_type,
            comment,
            expiration_at,
            created_at,
        }
    }
}

pub struct PHCString(pub String);

impl From<String> for PHCString {
    fn from(phc_string: String) -> Self {
        Self(phc_string)
    }
}

pub struct NewWrap {
    pub id: Id<Wrap>,
    pub redirect_url: String,
    pub password: String,
    pub auth_type: WrapAuthType,
    pub comment: String,
    pub expiration_at: DateTime<Utc>,
}

impl NewWrap {
    pub fn new(
        id: Id<Wrap>,
        redirect_url: String,
        password: String,
        auth_type: WrapAuthType,
        comment: String,
        expiration_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            redirect_url,
            password,
            auth_type,
            comment,
            expiration_at,
        }
    }
}
