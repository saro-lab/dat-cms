use crate::entity::dat_cert;
use crate::middleware::error::{ApiError, ApiResult};
use dat::crypto::DatCryptoAlgorithm;
use dat::signature::DatSignatureAlgorithm;
use dat::util::now_unix_timestamp;
use rand::random;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, SelectExt};
use std::str::FromStr;
use dat::certificate::DatCertificate;
use dat::error::DatError;

pub(crate) type CertificateCount = usize;
pub(crate) type NewCid = i64;
pub(crate) type DeleteCount = u64;

pub async fn get_certificates<C: ConnectionTrait>(version: i64, verify_only: bool, db: &C) -> ApiResult<(String, CertificateCount)> {
    let vec = get_all_certificates(db).await?;
    let ver = vec.last().map(|x| x.ver).unwrap_or(0).to_string();
    if ver == "0" {
        return Ok((ver, 0))
    }

    let mut result: Vec<String> = Vec::new();
    result.push(ver);

    get_all_certificates(db)
        .await?.iter()
        .filter(|x| x.ver > version)
        .map(|x| x.to_certificate())
        .collect::<ApiResult<Vec<DatCertificate>>>()?
        .iter().filter(|x| x.signable() || !verify_only)
        .map(|x| x.export(verify_only))
        .collect::<Result<Vec<String>, DatError>>()?
        .iter().for_each(|x| result.push(x.clone()));

    let count = result.len() - 1;

    Ok((result.join("\n"), count))
}

pub async fn get_all_certificates<C: ConnectionTrait>(db: &C) -> ApiResult<Vec<dat_cert::Model>> {
    let now = now_unix_timestamp();
    let vec = dat_cert::Entity::find()
        .filter(dat_cert::Column::ExpireTime.gte(now))
        .order_by_id_asc()
        .all(db).await?;
    Ok(vec)
}

pub async fn generate<C: ConnectionTrait>(
    signature: String,
    crypto: String,
    cron_certificate_propagation_delay_seconds: i64,
    cron_dat_issuance_duration_seconds: i64,
    cron_dat_ttl_seconds: i64,
    db: &C
) -> ApiResult<(NewCid, DeleteCount)> {
    let now = now_unix_timestamp() as i64;
    let delete_count = cleanup_expired(db).await?;
    let cid = generate_cid(db).await?;
    let cid = dat_cert::ActiveModel::generate(
        cid,
        now + cron_certificate_propagation_delay_seconds,
        cron_dat_issuance_duration_seconds,
        cron_dat_ttl_seconds,
        DatSignatureAlgorithm::from_str(&signature)?,
        DatCryptoAlgorithm::from_str(&crypto)?,
    )?
        .save(db).await?.cid.unwrap();
    Ok((cid, delete_count))
}

async fn cleanup_expired<C: ConnectionTrait>(db: &C) -> ApiResult<u64> {
    let clean_date = now_unix_timestamp() - (86400 * 30);
    Ok(dat_cert::Entity::delete_many().filter(dat_cert::Column::ExpireTime.lt(clean_date)).exec(db).await?.rows_affected)
}

async fn generate_cid<C: ConnectionTrait>(db: &C) -> ApiResult<i64> {
    for _ in 0 .. 1000 {
        let cid = random::<u32>() as i64;
        let exists = dat_cert::Entity::find()
            .filter(dat_cert::Column::Cid.eq(cid))
            .exists(db).await?;
        if !exists {
            return Ok(cid);
        }
    }
    Err(ApiError::new500("".to_string()))
}
