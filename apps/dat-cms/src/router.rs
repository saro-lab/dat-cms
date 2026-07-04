use crate::env::ENV;
use crate::router::debug_router::debug_router;
use crate::infrastructure::session::{session_layout, Session};
use axum::middleware::from_fn;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Router};
use saro_infra::error::{handle_panic, ApiError};
use tower::ServiceBuilder;
use tower_http::catch_panic::CatchPanicLayer;

mod cms_router;
mod debug_router;


pub fn bind_router() -> Router {
    let mut router = Router::new()
        .merge(cms_router::router());

    if ENV.server.debug {
        router = router.merge(debug_router());
    };

    router
        .fallback(handle_error_404)
        .layer(from_fn(session_layout))
        .layer(ServiceBuilder::new().layer(CatchPanicLayer::custom(handle_panic)))
}

async fn handle_error_404(method: axum::http::Method, uri: axum::http::Uri, Extension(session): Extension<Session>) -> Response {
    ApiError::NotFound(method.to_string(), uri.path().to_string(), session.ip().to_string()).into_response()
}
