use crate::middleware::database::db_pool;
use crate::middleware::error::{handle_error_404, handle_panic};
use crate::middleware::{cron, database};
use crate::service::entity;
use middleware::session::session_layout;
use axum::extract::connect_info::IntoMakeServiceWithConnectInfo;
use axum::middleware::from_fn;
use axum::Router;
use env::ENV;
use middleware::logging;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::catch_panic::CatchPanicLayer;

pub mod env;
pub mod middleware;
pub mod api;
mod service;
mod api_debug;

#[tokio::main]
async fn main() {
    logging::bind();
    database::bind().await.unwrap();
    entity::create_all_table(db_pool()).await.unwrap();
    service::cms::bind();
    cron::bind().await.unwrap();

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", ENV.server.port)).await.unwrap();
    tracing::info!("START {} PORT / DAT Certificate Management Service {}", ENV.server.port, ENV.server.version);

    axum::serve(listener, make_service().await)
        .await.unwrap();
}

async fn make_service() -> IntoMakeServiceWithConnectInfo<Router, SocketAddr> {
    let router = if ENV.server.debug {
        api_debug::debug_router().await
    } else {
        api::router().await
    };
    router
        .layer(ServiceBuilder::new().layer(CatchPanicLayer::custom(handle_panic)))
        .fallback(handle_error_404)
        .layer(from_fn(session_layout))
        .into_make_service_with_connect_info::<SocketAddr>()
}
