use crate::entity::dat_cms_cert;
use crate::middleware::error::{ApiError, ApiResult};
use dat::certificate::DatCertificate;
use dat::crypto::DatCryptoAlgorithm;
use dat::error::DatError;
use dat::signature::DatSignatureAlgorithm;
use dat::util::now_unix_timestamp;
use rand::random;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, SelectExt};
use std::str::FromStr;
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
use std::sync::OnceLock;
use tokio::sync::RwLock;

// cache time = 60 secs
const CACHE_TIME: u64 = 60;

pub(crate) type LastCertificateVersion = i64;
static CACHE_EXPIRE: OnceLock<AtomicU64> = OnceLock::new();
static CACHE_VERSION: OnceLock<AtomicI64> = OnceLock::new();
static CACHE_CERTIFICATES: OnceLock<RwLock<Vec<SerializedCertificate>>> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct SerializedCertificate {
    pub(crate) version: i64,
    pub(crate) full: String,
    pub(crate) verify_only: String,
}

pub struct Certificates {
    version: i64,
    list: Vec<String>
}

impl Certificates {
    pub fn size(&self) -> usize {
        self.list.len()
    }

    pub fn export(&self, prefix_version: bool) -> String {
        let mut result = String::new();

        if prefix_version {
            result.push_str(self.version.to_string().as_str());
            result.push('\n');
        }

        for node in &self.list {
            result.push('\n');
            result.push_str(node);
        }
        result
    }
}

pub fn bind() {
    CACHE_EXPIRE.set(AtomicU64::new(0)).expect("service::cms::bind() OnceLock Error");
    CACHE_VERSION.set(AtomicI64::new(0)).expect("service::cms::bind() OnceLock Error");
    CACHE_CERTIFICATES.set(RwLock::new(Vec::new())).expect("service::cms::bind() OnceLock Error");
}
pub async fn certificates<C: ConnectionTrait>(version: i64, verify_only: bool, db: &C) -> ApiResult<Certificates> {
    let now = now_unix_timestamp();
    let certificates = CACHE_CERTIFICATES.get().unwrap();
    let cache_expire = CACHE_EXPIRE.get().unwrap();
    let cache_version = CACHE_VERSION.get().unwrap();

    if cache_expire.load(Ordering::Acquire) < now {
        let mut certs_write = certificates.write().await;
        if cache_expire.load(Ordering::Acquire) < now {
            let new_certs = dat_cms_cert::Entity::find()
                .filter(dat_cms_cert::Column::Expire.gte(now))
                .order_by_id_asc()
                .all(db).await?
                .iter()
                .map(|x| x.serialize_certificate())
                .collect::<ApiResult<Vec<SerializedCertificate>>>()?;

            let new_cache_version = new_certs.last().map(|x| x.version).unwrap_or(0);
            *certs_write = new_certs;
            cache_version.store(new_cache_version, Ordering::Release);

            cache_expire.store(now + CACHE_TIME, Ordering::Release);
        }
    }

    let list = certificates.read().await.iter()
        .filter(|x| x.version > version)
        .map(|x| if verify_only { x.verify_only.clone() } else { x.full.clone() })
        .collect::<Vec<String>>();

    Ok(Certificates {
        version: CACHE_VERSION.get().unwrap().load(Ordering::Relaxed),
        list
    })
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
    let cid = dat_cms_cert::ActiveModel::generate(
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
    Ok(dat_cms_cert::Entity::delete_many().filter(dat_cms_cert::Column::ExpireTime.lt(clean_date)).exec(db).await?.rows_affected)
}

async fn generate_cid<C: ConnectionTrait>(db: &C) -> ApiResult<i64> {
    for _ in 0 .. 1000 {
        let cid = random::<u32>() as i64;
        let exists = dat_cms_cert::Entity::find()
            .filter(dat_cms_cert::Column::Cid.eq(cid))
            .exists(db).await?;
        if !exists {
            return Ok(cid);
        }
    }
    Err(ApiError::new500("".to_string()))
}
