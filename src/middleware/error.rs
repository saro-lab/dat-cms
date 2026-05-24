use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use dat::error::DatError;
use sea_orm::DbErr;
use std::fmt;
use std::fmt::Display;

pub type ApiResult<T> = Result<T, ApiError>;
const TEXT_PLAIN: &[(&str, &str); 2] = &[("Content-Type", "text/plain"), ("Err", "")];

#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    body: String,
}

impl ApiError {
    pub fn new(status: StatusCode, body: String) -> Self {
        Self { status, body }
    }
    pub fn new500(body: String) -> Self {
        Self { status: StatusCode::INTERNAL_SERVER_ERROR, body }
    }
    pub fn to_response(self) -> Response {
        (self.status, *TEXT_PLAIN, self.body).into_response()
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.status, self.body)
    }
}

impl From<DatError> for ApiError {
    fn from(error: DatError) -> Self {
        tracing::error!("{:?}", error);
        ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: "ERR: DAT ERROR".to_string(),
        }
    }
}

impl From<DbErr> for ApiError {
    fn from(value: DbErr) -> Self {
        tracing::error!("{:?}", value);
        ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: "ERR: DB ERROR".to_string(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        self.to_response()
    }
}

