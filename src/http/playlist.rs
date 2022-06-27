use super::prelude::*;
use axum::extract::Path;
use crate::model::playlist::{list_playlists, SimplePlaylist};
use crate::model::playlist::{get_playlist, Playlist};
use crate::model::channel::{list_channels, Channel};

#[derive(Template)]
#[template(path = "playlist.html")]
struct PlaylistTemplate<'a> {
    playlist: &'a Playlist,
    channels: &'a [Channel],
    playlists: &'a [SimplePlaylist],
}

async fn playlist(context: Extension<HttpContext>, Path(slug): Path<String>) -> Result<impl IntoResponse, HttpError> {
    let playlist = get_playlist(&slug, &context.db)
        .await?
        .ok_or(HttpError::NotFound)?;

    let channels = list_channels(&context.db)
        .await?;

    let playlists = list_playlists(&context.db)
        .await?;

    let template = PlaylistTemplate {
        playlist: &playlist,
        channels: &channels,
        playlists: &playlists,
    };

    Ok(Html(template.render()?))
}

pub fn router() -> Router {
    axum::Router::new()
        .route("/playlist/:slug", get(playlist))
}
