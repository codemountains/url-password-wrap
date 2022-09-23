use crate::context::errors::AppError;
use crate::context::validate::ValidatedRequest;
use axum::async_trait;
use axum::extract::{FromRequest, RequestParts};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{BoxError, Json};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tracing::log::error;
use validator::Validate;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonErrorResponse {
    error_code: String,
    errors: Vec<String>,
}

impl JsonErrorResponse {
    pub(crate) fn new(error_code: String, errors: Vec<String>) -> Self {
        Self { error_code, errors }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Validation(validation_errors) => {
                error!("{:?}", validation_errors);

                let mut messages: Vec<String> = Vec::new();
                let errors = validation_errors.field_errors();
                for (_, v) in errors.iter() {
                    for validation_error in v.to_owned() {
                        if let Some(msg) = validation_error.clone().message {
                            messages.push(msg.to_string());
                        }
                    }
                }

                (
                    StatusCode::BAD_REQUEST,
                    Json(JsonErrorResponse::new(
                        "invalid_request".to_string(),
                        messages,
                    )),
                )
            }
            AppError::JsonRejection(rejection) => {
                error!("{:?}", rejection);

                let messages = vec![rejection.to_string()];
                (
                    StatusCode::BAD_REQUEST,
                    Json(JsonErrorResponse::new(
                        "invalid_request".to_string(),
                        messages,
                    )),
                )
            }
        }
        .into_response()
    }
}

#[async_trait]
impl<T, B> FromRequest<B> for ValidatedRequest<T>
where
    T: DeserializeOwned + Validate,
    B: http_body::Body + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = AppError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req).await?;
        value.validate()?;
        Ok(ValidatedRequest(value))
    }
}
