use crate::entity::dat_certificates;
use crate::env::ENV;
use crate::middleware::error::ApiResult;
use dat::util::now_unix_timestamp;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

pub(crate) type CertificateCount = usize;
pub(crate) type NewCid = i64;
pub(crate) type DeleteCount = u64;

pub async fn get_certificates<C: ConnectionTrait>(verify_only: bool, db: &C) -> ApiResult<(String, CertificateCount)> {
    let certificates = dat_certificates::Entity::find().all(db).await?
        .iter()
        .map(|x|
            x.to_certificate()
                .map(|y| {
                    if y.signature_algorithm() != ENV.signature || y.crypto_algorithm() != ENV.crypto {
                        tracing::warn!("{} {} {} The stored certificate does not match the server settings", x.cid, x.signature_algorithm, x.crypto_algorithm);
                    }
                    y
                })
                .unwrap()
                .export(verify_only)
                .unwrap_or_else(|_| {
                    let export_option = if verify_only { "Export Verify Only" } else { "Export" };
                    tracing::warn!("{} {} {} {} Failed to convert the certificate", export_option, x.cid, x.signature_algorithm, x.crypto_algorithm);
                    "".to_string()
                })
        )
        .filter(|x| !x.is_empty())
        .collect::<Vec<String>>();
    let count = certificates.len();
    Ok((certificates.join("\n"), count))
}

pub async fn generate<C: ConnectionTrait>(db: &C) -> ApiResult<(NewCid, DeleteCount)> {
    let delete_count = cleanup_expired(db).await?;
    let cid = dat_certificates::ActiveModel::generate(ENV.issued_at(), ENV.issue_dur, ENV.dat_ttl, ENV.signature, ENV.crypto)?
        .save(db).await?.cid.unwrap();
    Ok((cid, delete_count))
}

async fn cleanup_expired<C: ConnectionTrait>(db: &C) -> ApiResult<u64> {
    let now = now_unix_timestamp();
    Ok(dat_certificates::Entity::delete_many().filter(dat_certificates::Column::ExpireTime.lt(now)).exec(db).await?.rows_affected)
}
