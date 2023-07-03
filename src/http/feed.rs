use axum::Router;
use axum::routing::get;
use axum::response::IntoResponse;
use axum::response::Html;
use axum::Extension;
use askama::Template;
use super::{HttpError, HttpContext};
use crate::model::channel::{list_channels, Channel};
use crate::model::video::{list_videos, Video};
use crate::model::playlist::{list_playlists, SimplePlaylist};

#[derive(Template)]
#[template(path = "feed.html")]
struct FeedTemplate<'a> {
    channels: &'a [Channel],
    videos: &'a [Video],
    playlists: &'a [SimplePlaylist],
    term: &'a str,
}

async fn feed(context: Extension<HttpContext>) -> Result<impl IntoResponse, HttpError> {
    let channels = list_channels(&context.db)
        .await?;

    let videos = list_videos(&context.db)
        .await?;

    let playlists = list_playlists(&context.db)
        .await?;

    let template = FeedTemplate {
        channels: &channels,
        videos: &videos,
        playlists: &playlists,
        term: "",
    };

    Ok(Html(template.render()?))
}

pub fn router() -> Router {
    axum::Router::new()
        .route("/feed", get(feed))
}
