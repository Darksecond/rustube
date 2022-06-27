use clap::Parser;
use rustube::config::Config;
use rustube::http;
use sqlx::sqlite::{SqlitePoolOptions, SqlitePool};
use anyhow::Context;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = Arc::new(Config::parse());

    //TODO Use https://docs.rs/sqlx/0.6.0/sqlx/sqlite/struct.SqliteConnectOptions.html
    let db = SqlitePoolOptions::new()
        .connect(&config.database_url)
        .await
        .context("Could not connect to database")?;

    register_exit(db.clone());

    sqlx::migrate!().run(&db).await?;

    rustube::jobs::spawn_jobs(config.clone(), db.clone());

    http::serve(config.clone(), db).await?;

    Ok(())
}

fn register_exit(db: SqlitePool) {
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("failed to listen for event");

        println!("Exiting");
        db.close().await;
        std::process::exit(0);
    });
}
