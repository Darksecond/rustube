use super::prelude::*;
use axum::extract::Path;
use crate::model::channel::{get_channel, list_channels, Channel};
use crate::model::video::{list_videos_for_channel, Video};
use crate::model::playlist::{list_playlists, SimplePlaylist};

#[derive(Template)]
#[template(path = "channel.html")]
struct ChannelTemplate<'a> {
    channel: &'a Channel,
    channels: &'a [Channel],
    videos: &'a [Video],
    playlists: &'a [SimplePlaylist],
    term: &'a str,
}

async fn channel(context: Extension<HttpContext>, Path(id): Path<String>) -> Result<impl IntoResponse, HttpError> {
    let channel = get_channel(&id, &context.db)
        .await?
        .ok_or(HttpError::NotFound)?;

    let videos = list_videos_for_channel(&id, &context.db)
        .await?;

    let channels = list_channels(&context.db)
        .await?;

    let playlists = list_playlists(&context.db)
        .await?;

    let template = ChannelTemplate {
        channel: &channel,
        channels: &channels,
        videos: &videos,
        playlists: &playlists,
        term: "",
    };

    Ok(Html(template.render()?))
}

pub fn router() -> Router {
    axum::Router::new()
        .route("/channel/:id", get(channel))
}
