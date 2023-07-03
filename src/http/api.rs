use axum::Router;
use axum::routing::get;
use axum::response::IntoResponse;
use axum::Extension;
use axum::extract::{Path, Query};
use axum::Json;
use serde::Deserialize;
use super::{HttpError, HttpContext};
use crate::model::channel::list_channels;
use crate::model::video::index::match_videos;
use crate::model::video::{list_videos, list_videos_for_channel};
use crate::model::playlist::{list_playlists, get_playlist};

async fn videos(context: Extension<HttpContext>) -> Result<impl IntoResponse, HttpError> {
    let videos = list_videos(&context.db)
        .await?;

    Ok(Json(videos))
}

async fn playlists(context: Extension<HttpContext>) -> Result<impl IntoResponse, HttpError> {
    let playlists = list_playlists(&context.db)
        .await?;

    Ok(Json(playlists))
}

async fn playlist_videos(context: Extension<HttpContext>, Path(slug): Path<String>) -> Result<impl IntoResponse, HttpError> {
    let playlist = get_playlist(&slug, &context.db)
        .await?;

    let videos = if let Some(playlist) = playlist {
        playlist.items.iter().map(|item| item.video.clone()).collect()
    } else {
        Vec::new()
    };

    Ok(Json(videos))
}

async fn channels(context: Extension<HttpContext>) -> Result<impl IntoResponse, HttpError> {
    let playlists = list_channels(&context.db)
        .await?;

    Ok(Json(playlists))
}

async fn channel_videos(context: Extension<HttpContext>, Path(id): Path<String>) -> Result<impl IntoResponse, HttpError> {
    let videos = list_videos_for_channel(&id, &context.db)
        .await?;

    Ok(Json(videos))
}

#[derive(Deserialize)]
struct SearchQuery {
    pub q: String,
}

async fn search(context: Extension<HttpContext>, Query(query): Query<SearchQuery>) -> Result<impl IntoResponse, HttpError> {
    let videos = match_videos(&query.q, &context.db)
        .await?;

    Ok(Json(videos))
}

pub fn router() -> Router {
    axum::Router::new()
        .route("/api/videos", get(videos))

        .route("/api/playlists", get(playlists))
        .route("/api/playlists/:slug/videos", get(playlist_videos))

        .route("/api/channels", get(channels))
        .route("/api/channels/:id/videos", get(channel_videos))

        .route("/api/search", get(search))
}
