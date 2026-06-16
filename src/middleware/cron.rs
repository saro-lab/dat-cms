use crate::env::ENV;
use crate::middleware::database::db_pool;
use crate::middleware::error::ApiResult;
use crate::service::cms;
use tokio_cron_scheduler::{Job, JobScheduler};

pub async fn bind() -> ApiResult<()> {
    if let Some(cron) = ENV.cron.as_ref() {
        let cmd = &cron.cmd;
        cms::register(cmd.clone(), db_pool()).await?;
        let sched = JobScheduler::new().await.unwrap();

        sched.add(
            Job::new_async(cron.expression.clone(), |_,_| {
                Box::pin(async move {
                    tracing::info!("DatCertificate Generate Cron");
                    if let Some(cron) = ENV.cron.as_ref() {
                        cms::register(cron.cmd.clone(), db_pool()).await.unwrap();
                    }
                })
            }).unwrap(),
        ).await.unwrap();

        sched.start().await.unwrap();
    }

    Ok(())
}
