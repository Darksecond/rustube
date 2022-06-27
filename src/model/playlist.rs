use sqlx::sqlite::SqlitePool;
use futures::prelude::*;
use super::video::Video;

#[derive(Debug)]
pub struct SimplePlaylist {
    pub title: String,
    pub slug: String,
}

#[derive(Debug)]
pub struct Playlist {
    pub title: String,
    pub slug: String,
    pub items: Vec<PlaylistItem>,
}

#[derive(Debug)]
pub struct PlaylistItem {
    pub id: String,
    pub video: Video,
}

#[derive(Debug)]
pub struct UpsertPlaylist {
    pub title: String,
    pub slug: String,
    pub items: Vec<UpsertPlaylistItem>,
}

#[derive(Debug)]
pub struct UpsertPlaylistItem {
    pub id: String,
}

pub struct PlaylistFromQuery {
    pub id: i64,
    pub title: String,
    pub slug: String,
}

pub struct PlaylistItemFromQuery {
    pub video_id: String,
    pub video_title: String,
    pub video_description: String,
    pub video_path: String,
    pub video_published_at: i64, //TODO Use string
    pub video_duration: i64,
    pub channel_id: String,
    pub channel_title: String,
    pub playstate_id: Option<String>,
    pub playstate_position: Option<i64>,
    pub playstate_modified_at: Option<i64>,
}

impl PlaylistFromQuery {
    pub fn into_playlist(self, items: Vec<PlaylistItem>) -> Playlist {
        Playlist {
            title: self.title,
            slug: self.slug,
            items,
        }
    }
}

impl PlaylistItemFromQuery {
    pub fn into_playlist_item(self) -> PlaylistItem {
        use super::video::VideoFromQuery;

        let id = self.video_id.clone();

        let video = VideoFromQuery {
            channel_id: self.channel_id,
            channel_title: self.channel_title,
            description: self.video_description,
            duration: self.video_duration,
            id: self.video_id,
            path: self.video_path,
            playstate_id: self.playstate_id,
            playstate_modified_at: self.playstate_modified_at,
            playstate_position: self.playstate_position,
            published_at: self.video_published_at,
            title: self.video_title,
        };
        let video = video.into_video();

        PlaylistItem {
            id,
            video,
        }
    }
}

#[tracing::instrument(skip(db))]
pub async fn upsert_playlist(playlist: UpsertPlaylist, db: &SqlitePool) -> Result<(), sqlx::Error> {
    let transaction = db.begin().await?;

    // upsert playlist
    let id: i64 = sqlx::query_scalar!(r#"
                 INSERT INTO playlists
                 (title, slug)
                 VALUES
                 ($1, $2)
                 ON CONFLICT(slug) DO UPDATE SET title=$1
                 RETURNING id
                 "#,
                 playlist.title,
                 playlist.slug)
        .fetch_one(db)
        .await?;

    tracing::trace!(id, "playlist");

    // delete old items
    sqlx::query!(r#"
                 DELETE FROM playlist_items
                 WHERE playlist_id = $1
                 "#,
                 id)
        .execute(db)
        .await?;

    // create items
    for (index, item) in playlist.items.iter().enumerate() {
        let index = index as i64;
        sqlx::query!(r#"
                     INSERT INTO playlist_items
                     (video_id, item_index, playlist_id)
                     VALUES
                     ($1, $2, $3)
                     "#,
                     item.id,
                     index,
                     id)
            .execute(db)
            .await?;
    }

    transaction.commit().await?;
    Ok(())
}

pub async fn get_playlist(slug: &str, db: &SqlitePool) -> Result<Option<Playlist>, sqlx::Error> {
    let playlist = sqlx::query_as!(PlaylistFromQuery, r#"
                                   SELECT id, title, slug FROM playlists WHERE slug=$1
                                   "#, slug)
        .fetch_optional(db)
        .await?;

    if let Some(playlist) = playlist {
        let items: Vec<_> = sqlx::query_as!(PlaylistItemFromQuery, r#"
                                         SELECT videos.id as video_id, videos.title as video_title, videos.description as video_description, videos.path as video_path, videos.published_at as video_published_at, videos.duration as video_duration, videos.channel_id as channel_id,
                                         channels.title AS channel_title,
                                         playstates.id as "playstate_id?", playstates.position as "playstate_position?", playstates.modified_at as "playstate_modified_at?"
                                         FROM playlist_items
                                         INNER JOIN videos ON videos.id = playlist_items.video_id
                                         INNER JOIN channels ON channels.id = videos.channel_id
                                         LEFT JOIN playstates ON playstates.id = videos.id
                                         WHERE playlist_items.playlist_id = $1
                                         ORDER BY playlist_items.item_index ASC
                                    "#, playlist.id)
            .fetch(db)
            .map_ok(PlaylistItemFromQuery::into_playlist_item)
            .try_collect()
            .await?;

        Ok(Some(playlist.into_playlist(items)))
    } else {
        Ok(None)
    }
}

pub async fn list_playlists(db: &SqlitePool) -> Result<Vec<SimplePlaylist>, sqlx::Error> {
    let playlists: Vec<_> = sqlx::query_as!(SimplePlaylist, r#"
                    SELECT title, slug
                    FROM playlists
                    ORDER BY title ASC
                    "#)
        .fetch(db)
        .try_collect()
        .await?;

    Ok(playlists)
}
