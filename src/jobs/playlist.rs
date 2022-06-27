use std::path::Path;
use super::{JobContext, JobError};
use serde::Deserialize;
use sqlx::sqlite::SqlitePool;
use crate::model::playlist::{upsert_playlist, UpsertPlaylist, UpsertPlaylistItem};

#[derive(Debug, Deserialize)]
struct PlaylistFileItem {
    id: String,
}

#[derive(Debug, Deserialize)]
struct PlaylistFile {
    title: String,
    slug: String,
    items: Vec<PlaylistFileItem>,
}

impl PlaylistFile {
    pub fn into_upsert_playlist(self) -> UpsertPlaylist {
        let items = self.items
            .into_iter()
            .map(|item|
                 UpsertPlaylistItem {
                     id: item.id
                 })
            .collect();
        UpsertPlaylist {
            title: self.title,
            slug: self.slug,
            items,
        }
    }
}


async fn process_entry(path: impl AsRef<Path>, db: &SqlitePool) -> Result<(), JobError> {
    let text = tokio::fs::read_to_string(path).await?;
    let playlist: PlaylistFile = toml::from_str(&text)?;
    let playlist = playlist.into_upsert_playlist();
    tracing::debug!(id=%playlist.slug, "processing playlist");

    upsert_playlist(playlist, db)
        .await?;

    Ok(())
}

pub async fn job(context: JobContext) -> Result<(), JobError> {
    let root = &context.config.video_root;

    tracing::info!(?root, "Scanning directory");

    let entries = wax::walk("**/*.playlist.toml", root)?
        .filter_map(|e| e.ok());

    for entry in entries {
        match process_entry(entry.path(), &context.db).await {
            Ok(_) => (),
            Err(error) => {
                //TODO Log path?
                tracing::warn!(%error, "error processing entry");
            },
        }
    }

    Ok(())
}
