use axum::{Router, Extension};
use axum::extract::Path;
use axum::routing::get;
use super::HttpContext;
use crate::model::video::{get_video, Video};
use axum::response::IntoResponse;
use tower_http::services::ServeFile;
use tower::ServiceExt;
use super::error::HttpError;
use axum::body::Body;
use axum::http::Request;
use crate::model::channel::{list_channels, Channel};
use askama::Template;
use axum::response::Html;
use serde::Serialize;
use crate::model::playlist::{list_playlists, SimplePlaylist};

#[derive(Serialize)]
struct VideoData<'a> {
    id: &'a str,
    title: &'a str,
    channel: &'a str,
}

#[derive(Template)]
#[template(path = "video.html")]
struct VideoTemplate<'a> {
    data: VideoData<'a>,
    channels: &'a [Channel],
    playlists: &'a [SimplePlaylist],
    video: &'a Video,
    term: &'a str,
}

async fn get_video_mp4(context: Extension<HttpContext>,
                       Path(id): Path<String>,
                       request: Request<Body>) -> Result<impl IntoResponse, HttpError> {

    let video = get_video(&id, &context.db)
        .await?
        .ok_or(HttpError::NotFound)?;

    Ok(
        ServeFile::new(video.path)
        .oneshot(request)
        .await?
      )
}

async fn video(context: Extension<HttpContext>, Path(id): Path<String>) -> Result<impl IntoResponse, HttpError> {
    let video = get_video(&id, &context.db)
        .await?
        .ok_or(HttpError::NotFound)?;

    let channels = list_channels(&context.db)
        .await?;

    let playlists = list_playlists(&context.db)
        .await?;

    let template = VideoTemplate {
        data: VideoData {
            id: &video.id,
            title: &video.title,
            channel: &video.channel.title,
        },
        video: &video,
        playlists: &playlists,
        channels: &channels,
        term: "",
    };

    Ok(Html(template.render()?))
}

pub fn router() -> Router {
    axum::Router::new()
        .route("/video/:id/video.mp4", get(get_video_mp4))
        .route("/video/:id", get(video))
}
