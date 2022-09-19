use axum::{Router, Extension};
use axum::extract::Query;
use super::HttpContext;
use axum::routing::get;
use super::error::HttpError;
use axum::response::IntoResponse;
use crate::model::channel::{list_channels, Channel};
use crate::model::playlist::{list_playlists, SimplePlaylist};
use crate::model::video::{Video, index::match_videos};
use askama::Template;
use axum::response::Html;
use serde::Deserialize;

#[derive(Deserialize)]
struct SearchQuery {
    pub q: String,
}

#[derive(Template)]
#[template(path = "search.html")]
struct SearchTemplate<'a> {
    videos: &'a [Video],
    channels: &'a [Channel],
    playlists: &'a [SimplePlaylist],
    term: &'a str,
}

//TODO move to index model.
async fn search(context: Extension<HttpContext>, Query(query): Query<SearchQuery>) -> Result<impl IntoResponse, HttpError> {
    let channels = list_channels(&context.db)
        .await?;

    let playlists = list_playlists(&context.db)
        .await?;

    let videos = match_videos(&query.q, &context.db).await?;

    let template = SearchTemplate {
        term: &query.q,
        channels: &channels,
        playlists: &playlists,
        videos: &videos,
    };

    Ok(Html(template.render()?))
}

pub fn router() -> Router {
    axum::Router::new()
        .route("/search", get(search))
}
