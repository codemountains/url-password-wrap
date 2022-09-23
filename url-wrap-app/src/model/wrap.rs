use chrono::{DateTime, TimeZone, Utc};
use url_wrap_kernel::model::wrap::auth_type::WrapAuthType;
use url_wrap_kernel::model::wrap::{NewWrap, Wrap};
use url_wrap_kernel::model::Id;

#[derive(Debug)]
pub struct WrapView {
    pub id: String,
    pub redirect_url: String,
    pub auth_type: u32,
    pub comment: String,
    pub expiration_at: DateTime<Utc>,
}

impl From<Wrap> for WrapView {
    fn from(w: Wrap) -> Self {
        Self {
            id: w.id.value.to_string(),
            redirect_url: w.redirect_url,
            auth_type: w.auth_type.id(),
            comment: w.comment,
            expiration_at: w.expiration_at,
        }
    }
}

pub struct CreateWrap {
    pub redirect_url: String,
    pub password: String,
    pub auth_type: u32,
    pub comment: String,
    pub expiration_at: u32,
}

impl CreateWrap {
    pub fn new(
        redirect_url: String,
        password: String,
        auth_type: u32,
        comment: String,
        expiration_at: u32,
    ) -> Self {
        Self {
            redirect_url,
            password,
            auth_type,
            comment,
            expiration_at,
        }
    }
}

impl TryFrom<CreateWrap> for NewWrap {
    type Error = anyhow::Error;

    fn try_from(cw: CreateWrap) -> Result<Self, Self::Error> {
        let wrap_id = Id::gen();
        let auth_type = WrapAuthType::try_from(cw.auth_type)?;
        let expiration_at = Utc.timestamp(cw.expiration_at as i64, 0u32);

        Ok(NewWrap::new(
            wrap_id,
            cw.redirect_url,
            cw.password,
            auth_type,
            cw.comment,
            expiration_at,
        ))
    }
}

pub struct AuthorizeWrap {
    pub password: String,
}
