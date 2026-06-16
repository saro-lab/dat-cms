use crate::middleware::session::Session;
use anyhow::anyhow;
use axum::http::{ StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Extension;
use dat::error::DatError;
use std::any::Any;
use std::net::IpAddr;
use thiserror::Error;

pub type ApiResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("ERR: Internal Server Error")]
    Internal(#[from] anyhow::Error),

    #[error("ERR: Database error")]
    Database(#[from] sea_orm::error::DbErr),

    #[error("ERR: DAT: {0}")]
    DatError(#[from] DatError),

    #[error("ERR: 400: {0}")]
    BadRequest(String),

    #[error("ERR: 404")]
    NotFound(String, String, String),

    #[error("ERR: Unauthorized access: {1}")]
    Unauthorized(String, IpAddr),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, body) = match &self {
            AppError::Internal(err) => {
                tracing::error!("ERROR: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            AppError::Database(err) => {
                tracing::error!("DB: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            AppError::DatError(err) => {
                tracing::error!("DAT: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            AppError::BadRequest(err) => {
                tracing::error!("BadRequest: {:?}", err);
                (StatusCode::BAD_REQUEST, self.to_string())
            },
            AppError::NotFound(method, path, ip) => {
                tracing::error!("404: {} {} {}", method, path, ip);
                (StatusCode::NOT_FOUND, self.to_string())
            },
            AppError::Unauthorized(token, ip) => {
                tracing::error!("Unauthorized: IP:{} Token:{}", token, ip);
                (StatusCode::UNAUTHORIZED, self.to_string())
            },
        };

        (status, body).into_response()
    }
}

pub async fn handle_error_404(method: axum::http::Method, uri: axum::http::Uri, Extension(session): Extension<Session>) -> Response {
    AppError::NotFound(method.to_string(), uri.path().to_string(), session.ip().to_string()).into_response()
}

pub fn handle_panic(err: Box<dyn Any + Send>) -> Response {
    let message = if let Some(s) = err.downcast_ref::<&str>() { s.to_string() }
    else if let Some(s) = err.downcast_ref::<String>() { s.clone() }
    else { "Unknown panic".to_string() };
    AppError::Internal(anyhow!(message)).into_response()
}
