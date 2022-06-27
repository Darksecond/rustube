mod prelude;
mod video;
mod feed;
mod error;
mod response;
mod assets;
mod channel;
mod playstate;
mod playlist;

use anyhow::Context;
use crate::config::Config;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use axum::{Router, Extension};
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;
use tower_http::add_extension::AddExtensionLayer;

pub use error::HttpError;

#[derive(Clone)]
struct HttpContext {
    config: Arc<Config>,
    db: SqlitePool,
}

pub async fn serve(config: Arc<Config>, db: SqlitePool) -> anyhow::Result<()> {
    use std::net::SocketAddr;

    let app = router().layer(
        ServiceBuilder::new()
            .layer(Extension(HttpContext {
                config: config.clone(),
                db: db.clone(),
            }))
            .layer(AddExtensionLayer::new(HttpContext {
                config: config.clone(),
                db,
            }))
            .layer(TraceLayer::new_for_http())
    );

    tracing::info!("Server starting on port {}", port = config.port);
    axum::Server::bind(&SocketAddr::new("::".parse()?, config.port))
        .serve(app.into_make_service())
        .await
        .context("error running server")
}

fn router() -> Router {
    axum::Router::new()
        .merge(video::router())
        .merge(feed::router())
        .merge(assets::router())
        .merge(channel::router())
        .merge(playstate::router())
        .merge(playlist::router())
}
