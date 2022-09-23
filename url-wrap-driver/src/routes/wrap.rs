use crate::context::axum_helper::JsonErrorResponse;
use crate::context::validate::ValidatedRequest;
use crate::model::wrap::{JsonAuthorizeWrap, JsonAuthorizedWrapView, JsonCreateWrap, JsonWrapView};
use crate::module::{Modules, ModulesExt};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use std::sync::Arc;
use tracing::error;
use tracing::log::info;
use url_wrap_app::model::wrap::AuthorizeWrap;

pub async fn create_wrap(
    ValidatedRequest(source): ValidatedRequest<JsonCreateWrap>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules.wrap_use_case().register_wrap(source.into()).await;
    res.map(|wv| {
        info!("Created wrap: {}", wv.id);
        let json: JsonWrapView = wv.into();
        (StatusCode::CREATED, Json(json))
    })
    .map_err(|err| {
        error!("{:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

pub async fn get_wrap(
    Path(id): Path<String>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules.wrap_use_case().get_wrap(id).await;
    match res {
        Ok(wv) => wv
            .map(|wv| {
                info!("Found: {}", wv.id);
                let json: JsonWrapView = wv.into();
                (StatusCode::OK, Json(json))
            })
            .ok_or_else(|| {
                error!("Wrap id is not found.");
                StatusCode::NOT_FOUND
            }),
        Err(err) => {
            error!("Unexpected error: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn auth_wrap(
    Path(id): Path<String>,
    ValidatedRequest(source): ValidatedRequest<JsonAuthorizeWrap>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let aw: AuthorizeWrap = source.into();
    let res = modules.wrap_use_case().verify_wrap(id, aw.password).await;
    match res {
        Ok(wv) => wv
            .map(|wv| {
                info!("Found: {}", wv.id);
                let json: JsonAuthorizedWrapView = wv.into();
                (StatusCode::OK, Json(json))
            })
            .ok_or_else(|| {
                error!("Expiration date has expired.");
                let errors = vec!["Expiration date has expired.".to_string()];
                let json = JsonErrorResponse::new("expired".to_string(), errors);
                (StatusCode::FORBIDDEN, Json(json))
            }),
        Err(err) => {
            error!("Unexpected error: {:?}", err);
            let errors = vec!["Authentication failed.".to_string()];
            let json = JsonErrorResponse::new("authentication_failed".to_string(), errors);
            Err((StatusCode::UNAUTHORIZED, Json(json)))
        }
    }
}
