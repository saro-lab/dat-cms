use crate::env::ENV;
use crate::middleware::database::db_pool;
use crate::middleware::error::ApiResult;
use crate::service::cms;
use axum::extract::{Path, Query};
use axum::routing::{get, post};
use axum::{Extension, Router};
use serde::Deserialize;
use std::net::IpAddr;

pub static API_VERSION: &str = "v1";

pub async fn router() -> Router {
    Router::new()
        .route(format!("/{API_VERSION}/cert/{{signature_algorithm}}/{{crypto_algorithm}}/{{certificate_propagation_delay_seconds}}/{{dat_issuance_duration_seconds}}/{{dat_ttl_seconds}}").as_str(), post(generate_certificate))
        .route(format!("/{API_VERSION}/cert/list/{{version}}").as_str(), get(get_certificate_list))
        .route(format!("/{API_VERSION}/cert/list/{{version}}/verify-only").as_str(), get(get_certificate_list))
        .route("/health", get(health))
        .route("/version", get(version))
        .route("/version/api", get(version_api))
}

async fn health() -> &'static str { "OK" }
async fn version() -> &'static str { &ENV.version }
async fn version_api() -> &'static str { API_VERSION }

pub async fn generate_certificate(
    Path((
        signature_algorithm,
        crypto_algorithm,
        certificate_propagation_delay_seconds,
        dat_issuance_duration_seconds,
        dat_ttl_seconds,
    )): Path<(String, String, i64, i64, i64)>,
    Extension(ip_addr): Extension<IpAddr>
) -> ApiResult<String> {
    let (new_cid, delete_count) = cms::generate(
        signature_algorithm,
        crypto_algorithm,
        certificate_propagation_delay_seconds,
        dat_issuance_duration_seconds,
        dat_ttl_seconds,
        db_pool()
    ).await?;
    tracing::info!("{ip_addr} GENERATE CERTIFICATE [{new_cid:x}] / DELETE {delete_count} CERTIFICATES");
    Ok("OK".to_string())
}

pub async fn get_certificate_list(Path(version): Path<i64>, Extension(ip_addr): Extension<IpAddr>) -> ApiResult<String> {
    let certs = cms::certificates(version, false, db_pool()).await?;
    tracing::info!("{ip_addr} GET {} CERTIFICATES", certs.size());
    Ok(body)
}
