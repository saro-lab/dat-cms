use crate::middleware::error::ApiResult;
use sea_orm::{ConnectionTrait, DatabaseConnection, Schema};

pub mod dat_cms_cert;

pub async fn create_all_table(db: &DatabaseConnection) -> ApiResult<()>
{
    // dat certificates
    db.execute(
        Schema::new(db.get_database_backend())
            .create_table_from_entity(dat_cms_cert::Entity)
            .if_not_exists()
    ).await?;

    Ok(())
}
