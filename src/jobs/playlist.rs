use std::path::Path;
use super::{JobContext, JobError};
use serde::Deserialize;
use sqlx::sqlite::SqlitePool;
use crate::model::playlist::{upsert_playlist, UpsertPlaylist, UpsertPlaylistItem};
use time::Date;

#[derive(Debug, Deserialize)]
enum PlaylistFileOrder {
    Order,
    Date,
}

impl Default for PlaylistFileOrder {
    fn default() -> Self {
        PlaylistFileOrder::Order
    }
}

#[derive(Debug, Deserialize)]
struct PlaylistFileItem {
    id: String,
    #[serde(default)]
    date: Option<Date>,
}

#[derive(Debug, Deserialize)]
struct PlaylistFile {
    title: String,
    slug: String,
    #[serde(default)]
    order: PlaylistFileOrder,
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

    pub async fn order_by_date(&mut self, db: &SqlitePool) -> Result<(), sqlx::Error> {
        use crate::model::video::get_video;

        for item in &mut self.items {
            if let Some(video) = get_video(&item.id, db).await? {
                item.date = Some(video.published_at);
            }
        }

        self.items.sort_by(|a, b| a.date.partial_cmp(&b.date).unwrap());

        Ok(())
   }
}


async fn process_entry(path: impl AsRef<Path>, db: &SqlitePool) -> Result<(), JobError> {
    let text = tokio::fs::read_to_string(path).await?;
    let mut playlist: PlaylistFile = toml::from_str(&text)?;

    match playlist.order {
        PlaylistFileOrder::Order => (),
        PlaylistFileOrder::Date => playlist.order_by_date(db).await?,
    }

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
