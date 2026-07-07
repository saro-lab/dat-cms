use crate::api_response::ApiResponse;
use crate::codes;
use anyhow::anyhow;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::any::Any;
use thiserror::Error;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Internal")]
    Internal(#[from] anyhow::Error),
    #[error("Database")]
    Database(#[from] sea_orm::error::DbErr),
    #[error("Null")]
    Null(),

    #[allow(unused)]
    #[error("BadRequest")]
    BadRequest(),
    #[error("Unauthorized")]
    Unauthorized(),
    #[error("NotFound")]
    NotFound(String, String, String),

    #[allow(unused)]
    #[error("Code")]
    Code(&'static str),
    #[allow(unused)]
    #[error("CodeMessage")]
    CodeMessage(&'static str, String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            ApiError::BadRequest() => (StatusCode::BAD_REQUEST, code(codes::BAD_REQUEST)),
            ApiError::Unauthorized() => (StatusCode::UNAUTHORIZED, code(codes::UNAUTHORIZED)),
            ApiError::NotFound(method, path, ip) => {
                tracing::error!("404: {} {} {}", method, path, ip);
                (StatusCode::NOT_FOUND, code(codes::NOT_FOUND))
            }

            ApiError::Code(c) => (StatusCode::BAD_REQUEST, code(c)),
            ApiError::CodeMessage(c, message) => {
                (StatusCode::BAD_REQUEST, ApiResponse::message(c, message))
            }

            err => {
                tracing::error!("ERROR: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, code(codes::INTERNAL))
            }
        };

        (status, body).into_response()
    }
}

fn code(code: &str) -> ApiResponse<()> {
    ApiResponse::code(code, None)
}

pub fn handle_panic(err: Box<dyn Any + Send>) -> Response {
    let message = if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else {
        "Unknown panic".to_string()
    };
    ApiError::Internal(anyhow!(message)).into_response()
}
