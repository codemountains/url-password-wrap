use serde::{Deserialize, Serialize};
use url_wrap_app::model::wrap::{AuthorizeWrap, CreateWrap, WrapView};
use validator::Validate;

const MIN_VALUE: i64 = u32::MIN as i64; // 0
const MAX_VALUE: i64 = u32::MAX as i64; // 4_294_967_295

#[derive(Debug, Serialize)]
pub struct JsonWrapView {
    pub id: String,
    pub auth_type: u32,
    pub comment: String,
    pub expiration_at: String,
}

impl From<WrapView> for JsonWrapView {
    fn from(wv: WrapView) -> Self {
        Self {
            id: wv.id,
            auth_type: wv.auth_type,
            comment: wv.comment,
            expiration_at: wv.expiration_at.to_rfc3339(),
        }
    }
}

#[derive(Deserialize, Debug, Validate)]
pub struct JsonCreateWrap {
    #[validate(
        url(message = "`redirectUrl` is invalid URL format."),
        required(message = "`redirectUrl` is null.")
    )]
    #[serde(rename = "redirectUrl")]
    pub redirect_url: Option<String>,
    #[validate(
        length(min = 1, message = "`password` is empty."),
        required(message = "`password` is null.")
    )]
    pub password: Option<String>,
    #[validate(range(min = 1, max = 2, message = "`authType` is 1 or 2."))]
    #[serde(rename = "authType")]
    pub auth_type: i64,
    #[validate(required(message = "`comment` is null."))]
    pub comment: Option<String>,
    #[validate(range(
        min = "MIN_VALUE",
        max = "MAX_VALUE",
        message = "`expirationAt` is minimum 0 and maximum 4294967295."
    ))]
    #[serde(rename = "expirationAt")]
    pub expiration_at: i64,
}

impl From<JsonCreateWrap> for CreateWrap {
    fn from(jc: JsonCreateWrap) -> Self {
        CreateWrap {
            redirect_url: jc.redirect_url.unwrap(),
            password: jc.password.unwrap(),
            auth_type: jc.auth_type as u32,
            comment: jc.comment.unwrap(),
            expiration_at: jc.expiration_at as u32,
        }
    }
}

#[derive(Deserialize, Debug, Validate)]
pub struct JsonAuthorizeWrap {
    #[validate(
        length(min = 1, message = "`password` is empty."),
        required(message = "`password` is null.")
    )]
    pub password: Option<String>,
}

impl From<JsonAuthorizeWrap> for AuthorizeWrap {
    fn from(aw: JsonAuthorizeWrap) -> Self {
        Self {
            password: aw.password.unwrap(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct JsonAuthorizedWrapView {
    pub id: String,
    pub redirect_url: String,
    pub expiration_at: String,
}

impl From<WrapView> for JsonAuthorizedWrapView {
    fn from(wv: WrapView) -> Self {
        Self {
            id: wv.id,
            redirect_url: wv.redirect_url,
            expiration_at: wv.expiration_at.to_rfc3339(),
        }
    }
}
