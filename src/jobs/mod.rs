mod scan;
mod playlist;
mod index;

use crate::config::Config;
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;
use thiserror::Error;
use std::time::Duration;

#[derive(Error, Debug)]
pub enum JobError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    DirWalk(#[from] wax::BuildError<'static>),

    #[error(transparent)]
    Deserialize(#[from] serde_json::Error),

    #[error(transparent)]
    Toml(#[from] toml::de::Error),
}

#[derive(Clone)]
pub struct JobContext {
    config: Arc<Config>,
    db: SqlitePool,
}

#[tracing::instrument(skip(context, job))]
pub async fn run_job<F>(context: JobContext, name: &str, interval: Duration, job: fn(JobContext) -> F) where F: futures::Future<Output=Result<(), JobError>> + Send + 'static {
    let mut interval = tokio::time::interval(interval);
    loop {
        interval.tick().await;
        tracing::info!("Started");
        match job(context.clone()).await {
            Ok(_) => {
                tracing::info!("Success");
            },
            Err(error) => {
                tracing::error!(%error, "Failure");
            },
        }
    }
}

pub fn spawn_jobs(config: Arc<Config>, db: SqlitePool) {
    let context = JobContext {
        config,
        db,
    };

    //TODO Less often
    tokio::spawn(run_job(context.clone(), "scan", Duration::from_secs(60), scan::job));
    tokio::spawn(run_job(context.clone(), "playlist", Duration::from_secs(60), playlist::job));
    tokio::spawn(run_job(context.clone(), "index", Duration::from_secs(60), index::job));
}
